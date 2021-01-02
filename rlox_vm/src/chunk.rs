use rlox_core::frontend::definitions::literal::Literal;
use rlox_core::frontend::definitions::token::Token;
use rlox_core::{error::LoxError, frontend::definitions::token_type::TokenType};
use std::cell::RefCell;
use std::fmt::Error;
use std::fmt::Formatter;
use std::time::SystemTime;
// use crate::debug::disassemble_inst;
use crate::{
    class::Class,
    gc::{
        heap::Heap,
        root::{CustomClone, CustomVecOps, Root, UniqueRoot},
    },
    instance::{Instance, InstanceBoundMethod},
    system_calls::SystemCalls,
};
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

const MAX_STACK: usize = 1000;

type NativeFn = fn(args: Vec<Object>) -> Object;

/**
 * TODO:
 * print stack trace,
 * special functions for breakpoint, stack trace etc.
 * make stack a static array, it will help keep stack clean when returning from functions.
 * object pooling
 * consider ecs.
 * in all vecs assign a decently large capacity
 * pass strings by ref and garbage collect them too
 * trace strings too, string pooling, handle weak references to these strings while garbage collecting
 * we don't need to separate method resolution and call always, optimize it chapter 28.5
 * none of the get* should pop values off the stack, only take ref
*/

#[derive(Debug)]
pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
    // TODO: non closures can be made functions instead of closures
    // Function(FuncSpec),
    NativeFunction(NativeFn),
    Closure(UniqueRoot<FuncSpec>),
    ClassDef(Root<Class>),
    InstanceDef(Root<Instance>),
    InstanceBindDef(Root<InstanceBoundMethod>),
}

impl CustomClone for Object {
    fn clone(&self, gc: &Heap) -> Self {
        match self {
            Object::Str(v) => Object::Str(v.clone()),
            Object::Num(v) => Object::Num(*v),
            Object::Bool(v) => Object::Bool(*v),
            Object::Nil => Object::Nil,
            Object::NativeFunction(v) => Object::NativeFunction(v.clone()),
            Object::Closure(v) => Object::Closure(v.clone(gc)),
            Object::ClassDef(v) => Object::ClassDef(v.clone(gc)),
            Object::InstanceDef(v) => Object::InstanceDef(v.clone(gc)),
            Object::InstanceBindDef(v) => Object::InstanceBindDef(v.clone(gc)),
        }
    }
}

#[derive(Debug)]
pub enum UpValue {
    Open(usize),
    Closed(Object),
}

impl CustomClone for UpValue {
    fn clone(&self, gc: &Heap) -> Self {
        match self {
            UpValue::Open(val) => UpValue::Open(*val),
            UpValue::Closed(val) => UpValue::Closed(val.clone(gc)),
        }
    }
}

#[derive(Debug)]
pub struct UpValueWrap(pub RefCell<UpValue>);

impl CustomClone for UpValueWrap {
    fn clone(&self, gc: &Heap) -> Self {
        UpValueWrap {
            0: self.0.clone(gc),
        }
    }
}

impl UpValueWrap {
    pub fn new(upval: UpValue) -> Self {
        UpValueWrap {
            0: RefCell::new(upval),
        }
    }
    pub fn update(&self, upval: UpValue) {
        self.0.replace(upval);
    }
    pub fn get(&self, gc: &Heap) -> UpValue {
        self.0.borrow().clone(gc)
    }
}

impl From<Literal> for Object {
    fn from(val: Literal) -> Self {
        match val {
            Literal::NUM(v) => Object::Num(v),
            Literal::STRING(v) => Object::Str(v),
            Literal::BOOL(v) => Object::Bool(v),
            Literal::NIL => Object::Nil,
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        use Object::*;

        match (self, other) {
            (&Str(ref a), &Str(ref b)) => a == b,
            (&Num(ref a), &Num(ref b)) => a == b,
            (&Bool(ref a), &Bool(ref b)) => a == b,
            (&Nil, &Nil) => true,
            _ => false,
        }
    }
}

impl Display for Object {
    fn fmt(
        &self,
        writer: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Object::Str(val) => writer.write_str(&val.to_string()),
            Object::Num(val) => writer.write_str(&val.to_string()),
            Object::Bool(val) => writer.write_str(&val.to_string()),
            Object::Nil => writer.write_str("Nil"),
            // Object::Function(val) => writer.write_fmt(format_args!("Function<{:?}>", val.name)),
            Object::NativeFunction(_) => writer.write_fmt(format_args!("NativeFunction<>")),
            Object::Closure(val) => writer.write_fmt(format_args!("Closure<{:?}>", val.name)),
            Object::ClassDef(val) => writer.write_fmt(format_args!("Class<{}>", val.name)),
            Object::InstanceDef(val) => {
                writer.write_fmt(format_args!("Instance<{}>", val.class.name))
            }
            Object::InstanceBindDef(val) => writer.write_fmt(format_args!("InstanceBind<>")),
        }
    }
}

impl Object {
    pub fn add(&self, other: &Self, line_no: u32) -> Result<Self, LoxError> {
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Num(l + r)),
            (Str(ref l), Str(ref r)) => {
                let mut l = l.clone();
                l.push_str(r);
                Ok(Object::Str(l))
            }
            // TODO: import error def
            _ => Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            )),
        }
    }
    pub fn sub(&self, other: &Self, line_no: u32) -> Result<Self, LoxError> {
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Num(l - r)),
            // TODO: import error def
            _ => Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            )),
        }
    }
    pub fn mul(&self, other: &Self, line_no: u32) -> Result<Self, LoxError> {
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Num(l * r)),
            // TODO: import error def
            _ => Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            )),
        }
    }
    pub fn div(&self, other: &Self, line_no: u32) -> Result<Self, LoxError> {
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Num(l / r)),
            // TODO: import error def
            _ => Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            )),
        }
    }
    pub fn gt(&self, other: &Self, line_no: u32) -> Result<Self, LoxError> {
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Bool(l > r)),
            (Str(ref l), Str(ref r)) => Ok(Object::Bool(l > r)),
            // TODO: import error def
            _ => Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            )),
        }
    }
    pub fn gte(&self, other: &Self, line_no: u32) -> Result<Self, LoxError> {
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Bool(l >= r)),
            (Str(ref l), Str(ref r)) => Ok(Object::Bool(l >= r)),
            // TODO: import error def
            _ => Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            )),
        }
    }
    pub fn lt(&self, other: &Self, line_no: u32) -> Result<Self, LoxError> {
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Bool(l < r)),
            (Str(ref l), Str(ref r)) => Ok(Object::Bool(l < r)),
            // TODO: import error def
            _ => Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            )),
        }
    }
    pub fn lte(&self, other: &Self, line_no: u32) -> Result<Self, LoxError> {
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Bool(l <= r)),
            (Str(ref l), Str(ref r)) => Ok(Object::Bool(l <= r)),
            // TODO: import error def
            _ => Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            )),
        }
    }
    pub fn bool_or(&self, other: &Self, line_no: u32) -> Result<Self, LoxError> {
        use Object::*;
        match (self, other) {
            (Bool(ref l), Bool(ref r)) => Ok(Object::Bool(*l || *r)),
            // TODO: import error def
            _ => Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Boolean".to_string(),
            )),
        }
    }
    pub fn bool_and(&self, other: &Self, line_no: u32) -> Result<Self, LoxError> {
        use Object::*;
        match (self, other) {
            (Bool(ref l), Bool(ref r)) => Ok(Object::Bool(*l && *r)),
            // TODO: import error def
            _ => Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Boolean".to_string(),
            )),
        }
    }
    pub fn neg(&self, line_no: u32) -> Result<Self, LoxError> {
        use Object::*;
        match (self) {
            (Num(ref l)) => Ok(Object::Num(-*l)),
            // TODO: import error def
            _ => Err(LoxError::RuntimeError(
                self.to_string(),
                line_no,
                "Operands not Num".to_string(),
            )),
        }
    }

    pub fn not(&self, line_no: u32) -> Result<Self, LoxError> {
        use Object::*;
        match (self) {
            (Bool(ref l)) => Ok(Object::Bool(!*l)),
            // TODO: import error def
            _ => Err(LoxError::RuntimeError(
                self.to_string(),
                line_no,
                "Operands not Boolean".to_string(),
            )),
        }
    }
}
impl Eq for Object {}

#[derive(Debug, Clone)]
pub struct Local {
    pub name: Token,
    pub depth: i32,
    pub is_closed: bool,
}
#[derive(Debug)]
pub struct FuncSpec {
    pub arity: u32,
    pub chunks: Vec<OpCode>,
    pub name: Option<String>,
    pub fn_type: FunctionType,
    pub locals: Vec<Local>,
    pub scope_depth: i32,
    // index, isLocal
    pub upvalues: Vec<(usize, bool)>,
    pub upvalues_ref: RefCell<Vec<Root<UpValueWrap>>>,
}

impl CustomClone for FuncSpec {
    fn clone(&self, gc: &Heap) -> Self {
        FuncSpec {
            arity: self.arity.clone(),
            chunks: self.chunks.clone(),
            name: self.name.clone(),
            fn_type: self.fn_type.clone(),
            locals: self.locals.clone(),
            scope_depth: self.scope_depth.clone(),
            upvalues: self.upvalues.clone(),
            upvalues_ref: self.upvalues_ref.clone(gc),
        }
    }
}

impl FuncSpec {
    pub fn new(arity: u32, name: Option<String>, fn_type: FunctionType) -> Self {
        let mut locals = vec![];
        // println!("fn {:?} {:?}", name, fn_type);
        if FunctionType::FUNCTION == fn_type || FunctionType::LAMBDA == fn_type {
            locals.push(Local {
                name: Token::new(TokenType::IDENTIFIER, 0, None, String::from("")),
                depth: 0,
                is_closed: false,
            });
        } else if fn_type == FunctionType::INIT || fn_type == FunctionType::METHOD {
            locals.push(Local {
                name: Token::new(TokenType::IDENTIFIER, 0, None, String::from("this")),
                depth: 0,
                is_closed: false,
            });
        }
        FuncSpec {
            arity,
            chunks: vec![],
            name,
            fn_type,
            locals,
            scope_depth: 0,
            upvalues: vec![],
            upvalues_ref: RefCell::new(vec![]),
        }
    }

    pub fn resolve_local(&mut self, token: &Token) -> i32 {
        for i in (0..self.locals.len()).rev() {
            if self.locals[i].name.lexeme == token.lexeme {
                return i as i32;
            }
        }
        return -1;
    }

    pub fn add_upvalue(&mut self, index: usize, is_local: bool) -> usize {
        for i in 0..self.upvalues.len() {
            if self.upvalues[i] == (index, is_local) {
                return i;
            }
        }
        self.upvalues.push((index, is_local));
        self.upvalues.len() - 1
    }

    // pub fn get_chunks(&self) -> &mut Vec<OpCode> {
    //     &mut self.chunks
    // }

    // pub fn get_local(&self, ind: usize) -> &mut Local {
    //     &mut self.locals.get_mut(ind).unwrap()
    // }
    // pub fn get_upval(&self, ind: usize) -> &mut (usize, bool) {
    //     &mut self.upvalues.get_mut(ind).unwrap()
    // }
    // pub fn get_upval_ref(&self, ind: usize) -> &mut UpValueWrap {
    //     &mut self.upvalues_ref.borrow_mut().get_mut(ind).unwrap()
    // }

    // pub fn get_name(&self) -> String {
    //     match self.name {
    //         Some(name) => name,
    //         _ => String::from("")
    //     }
    // }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionType {
    FUNCTION,
    SCRIPT,
    METHOD,
    INIT,
    LAMBDA,
}

// TODO: ensure every primitive Object is immutable
// TODO: runner interface which both the vm and treewalker can implement

pub enum VmErr {
    CompileError,
    RuntimeError,
}

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Return(u32),
    Exit(u32),
    Constant(usize),
    // Unary
    Negate(u32),
    Not(u32),

    //Binary
    Add(u32),
    Divide(u32),
    Multiply(u32),
    Subs(u32),
    GreaterThan(u32),
    GreaterThanEq(u32),
    LesserThan(u32),
    LesserThanEq(u32),
    NotEqualTo(u32),
    EqualTo(u32),

    // Boolean
    BoolAnd(u32),
    BoolOr(u32),

    //Stmt
    Print(u32),

    //Variable
    DefineGlobal(u32, usize),
    GetGlobal(u32, usize),
    SetGlobal(u32, usize),

    GetLocal(u32, usize),
    SetLocal(u32, usize),

    GetUpvalue(u32, usize),
    SetUpvalue(u32, usize),

    GetProperty(u32, usize),
    SetProperty(u32, usize),

    GetSuper(u32, usize),

    //Control Flow
    JumpIfFalse(u32, usize),
    Jump(u32, usize),

    //Fn
    Call(u32, usize),
    Closure(u32, usize),
    ClassDef(u32, usize),
    MethodDef(u32, usize),
    Inherit(u32),

    //Helpers
    StackPop,
    CloseUpvalue,
    NilVal,
    NoOp,
    PrintStackTrace,
}

macro_rules! binary_op {
    ($self:ident, $op:ident, $line_no:ident, $gc: ident) => {{
        let b = $self.pop_stack($gc).unwrap();
        let a = $self.pop_stack($gc).unwrap();
        let x = a.$op(&b, $line_no)?;
        $self.push_stack(x);
    }};
}

struct PrintVec(Vec<Object>);

impl Display for PrintVec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if (self.0.len() < 1) {
            return Ok(());
        }

        let mut comma_separated = String::new();

        for num in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "{}", comma_separated)
    }
}

pub struct CallFrame {
    pub func: UniqueRoot<FuncSpec>,
    ip: usize,
    slot: usize,
}

impl CallFrame {
    pub fn new(func: UniqueRoot<FuncSpec>, ip: usize, slot: usize) -> Self {
        CallFrame { func, ip, slot }
    }
}

pub struct VM<T: SystemCalls> {
    pub frames: Vec<CallFrame>,

    pub constant_pool: Vec<Object>,
    pub stack: Vec<Object>,
    sp: usize,
    pub globals: HashMap<String, Object>,
    pub open_upvalues: RefCell<Vec<Root<UpValueWrap>>>,
    sys_interface: T,
}

impl<T: SystemCalls> VM<T> {
    pub fn new(sys_interface: T, constant_pool: Vec<Object>, func: FuncSpec, gc: &Heap) -> Self {
        let mut vm = VM {
            constant_pool,
            stack: vec![],
            sp: 0,
            globals: HashMap::new(),
            frames: vec![CallFrame::new(gc.get_unique_root(func), 0, 0)],
            open_upvalues: RefCell::new(vec![]),
            sys_interface,
        };

        vm.define_native_fn("clock", |_| {
            let curr_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            Object::Num(curr_time.as_millis() as f64)
        });

        vm.define_native_fn("native_add", |args| args[0].add(&args[1], 0).unwrap());

        vm
    }

    pub fn define_native_fn<S: AsRef<str>>(&mut self, name: S, fn_def: NativeFn) {
        self.globals
            .insert(name.as_ref().to_string(), Object::NativeFunction(fn_def));
    }

    //TODO: try prefetching
    //TODO: this whole thing barely does any error handling
    pub fn run(&mut self, is_debug: bool, gc: &Heap) -> Result<(), LoxError> {
        use OpCode::*;

        // let frame = self.frames.last_mut().unwrap();
        let mut i = 0;
        loop {
            if i % 40 == 0 {
                // gc.collect_free(self);
            }
            i += 1;
            //TODO: handle debugging
            // if is_debug {
            //     disassemble_inst(&self, self.ip);
            // }
            let ip = self.frames.last_mut().unwrap().ip;
            self.frames.last_mut().unwrap().ip += 1;

            // println!("exec: {:?}", self.frames.last_mut().unwrap().func.chunks[ip]);
            match self.frames.last_mut().unwrap().func.chunks[ip] {
                Constant(pos) => {
                    // this will create new copies everytime. think over
                    self.push_stack(self.constant_pool[pos].clone(&gc))
                }
                Exit(_) => return Ok(()),
                Negate(line_no) => {
                    let a = self.pop_stack(gc).unwrap();
                    self.push_stack(a.neg(line_no)?);
                }
                Not(line_no) => {
                    let a = self.pop_stack(gc).unwrap();
                    self.push_stack(a.not(line_no)?);
                }
                Add(line_no) => {
                    binary_op!(self, add, line_no, gc)
                }
                Divide(line_no) => {
                    binary_op!(self, div, line_no, gc)
                }
                Multiply(line_no) => {
                    binary_op!(self, mul, line_no, gc)
                }
                Subs(line_no) => {
                    binary_op!(self, sub, line_no, gc)
                }
                GreaterThan(line_no) => {
                    binary_op!(self, gt, line_no, gc)
                }
                GreaterThanEq(line_no) => {
                    binary_op!(self, gte, line_no, gc)
                }
                LesserThan(line_no) => {
                    binary_op!(self, lt, line_no, gc)
                }
                LesserThanEq(line_no) => {
                    binary_op!(self, lte, line_no, gc)
                }
                NotEqualTo(_) => {
                    let val = self.pop_stack(gc).unwrap() != self.pop_stack(gc).unwrap();
                    self.push_stack(Object::Bool(val));
                }
                EqualTo(_) => {
                    let val = self.pop_stack(gc).unwrap() == self.pop_stack(gc).unwrap();
                    self.push_stack(Object::Bool(val));
                }
                BoolOr(line_no) => {
                    binary_op!(self, bool_or, line_no, gc)
                }
                BoolAnd(line_no) => {
                    binary_op!(self, bool_and, line_no, gc)
                }
                NilVal => self.push_stack(Object::Nil),
                Print(_) => {
                    let x = self.pop_stack(gc).unwrap();
                    self.sys_interface.print(&x, gc);
                }
                StackPop => {
                    self.pop_stack(gc);
                }
                DefineGlobal(line_no, pos) => {
                    let name = self.constant_pool[pos].to_string();
                    if let Some(val) = self.stack.last() {
                        self.globals.insert(name, val.clone(&gc));
                    } else {
                        return Err(LoxError::RuntimeError(
                            "dg".to_string(),
                            line_no,
                            "".to_string(),
                        ));
                    }
                }
                GetGlobal(line_no, pos) => {
                    //TODO: actually check if it's a string
                    let name = self.constant_pool[pos].to_string();
                    if let Some(val) = self.globals.get(&name) {
                        self.push_stack(val.clone(&gc));
                    } else {
                        return Err(LoxError::RuntimeError(
                            "gg".to_string(),
                            line_no,
                            "".to_string(),
                        ));
                    }
                }
                SetGlobal(line_no, pos) => {
                    //TODO: actually check if it's a string
                    let name = self.constant_pool[pos].to_string();
                    if let Some(val) = self.stack.last() {
                        if self.globals.insert(name, val.clone(&gc)).is_none() {
                            return Err(LoxError::RuntimeError(
                                "unknown".to_string(),
                                line_no,
                                "".to_string(),
                            ));
                        }
                    } else {
                        return Err(LoxError::RuntimeError(
                            "sg".to_string(),
                            line_no,
                            "".to_string(),
                        ));
                    }
                }
                GetLocal(line_no, pos) => {
                    // println!("{} {} stacktrace: {}", self.frames.last_mut().unwrap().slot , pos, PrintVec(self.stack.clone(&gc)));
                    if let Some(val) = self.stack.get(self.frames.last_mut().unwrap().slot + pos) {
                        self.push_stack(val.clone(&gc));
                    } else {
                        return Err(LoxError::RuntimeError(
                            "gl".to_string(),
                            line_no,
                            "".to_string(),
                        ));
                    }
                }
                SetLocal(line_no, pos) => {
                    if let Some(val) = self.stack.last() {
                        self.stack[self.frames.last_mut().unwrap().slot + pos] = val.clone(&gc);
                    } else {
                        return Err(LoxError::RuntimeError(
                            "sl".to_string(),
                            line_no,
                            "".to_string(),
                        ));
                    }
                }
                GetProperty(line_no, pos) => {
                    // println!("stacktrace: {}", PrintVec(self.stack.clone(&gc)));
                    if let Some(Object::InstanceDef(inst)) = self.pop_stack(gc) {
                        // TODO: String/identifier check
                        let prop = self.constant_pool[pos].to_string();
                        if let Some(field) = inst.get(&prop, gc) {
                            self.push_stack(field);
                        } else if self.bind_method(&inst, &inst.class, &prop, gc).is_ok() {
                        } else {
                            return Err(LoxError::RuntimeError(
                                "gp".to_string(),
                                line_no,
                                "".to_string(),
                            ));
                        }
                    } else {
                        return Err(LoxError::RuntimeError(
                            "gp".to_string(),
                            line_no,
                            "".to_string(),
                        ));
                    }
                }
                SetProperty(line_no, pos) => {
                    let val = self.pop_stack(gc).unwrap();
                    if let Some(Object::InstanceDef(inst)) = &self.stack.last() {
                        // TODO: String/identifier check
                        let prop = self.constant_pool[pos].to_string();
                        inst.set(prop, val.clone(gc));
                        self.pop_stack(gc);
                        self.push_stack(val);
                    } else {
                        return Err(LoxError::RuntimeError(
                            "sp".to_string(),
                            line_no,
                            "".to_string(),
                        ));
                    }
                }
                SetUpvalue(line_no, pos) => {
                    if let Some(val) = self.stack.last() {
                        let x = self
                            .frames
                            .last()
                            .unwrap()
                            .func
                            .upvalues_ref
                            .borrow()
                            .get(pos)
                            .unwrap()
                            .clone(&gc)
                            .get(&gc);
                        match x {
                            UpValue::Open(up_pos) => {
                                self.stack[up_pos] = val.clone(&gc);
                            }
                            UpValue::Closed(line_no) => {
                                self.frames.last().unwrap().func.upvalues_ref.borrow_mut()[pos]
                                    .update(UpValue::Closed(val.clone(&gc)));
                            }
                            _ => {}
                        }
                    } else {
                        return Err(LoxError::RuntimeError(
                            "su".to_string(),
                            line_no,
                            "".to_string(),
                        ));
                    }
                }
                GetUpvalue(line_no, pos) => {
                    // use std::borrow::Borrow;
                    // let x = self.frames.last().unwrap().func.upvalues_ref.borrow().get(pos).unwrap().clone(&gc).borrow();
                    // let x = &*x.get(pos).unwrap().borrow();
                    match self
                        .frames
                        .last()
                        .unwrap()
                        .func
                        .upvalues_ref
                        .clone(&gc)
                        .borrow()
                        .get(pos)
                        .unwrap()
                        .clone(&gc)
                        .get(&gc)
                    {
                        UpValue::Open(up_pos) => {
                            // println!("upping Open {:?}", up_pos);
                            self.push_stack(self.stack[up_pos].clone(&gc));
                        }
                        UpValue::Closed(val) => {
                            // println!("upping Closed {}", val);
                            self.push_stack(val.clone(&gc));
                        }
                        _ => {}
                    }
                }
                JumpIfFalse(line_no, offset) => {
                    if let Some(Object::Bool(false)) = self.stack.last() {
                        self.frames.last_mut().unwrap().ip = offset;
                    } else if let Some(Object::Bool(true)) = self.stack.last() {
                    } else {
                        return Err(LoxError::RuntimeError(
                            "jif".to_string(),
                            line_no,
                            "".to_string(),
                        ));
                    }
                }
                Jump(line_no, offset) => {
                    self.frames.last_mut().unwrap().ip = offset;
                }
                Call(line_no, args_count) => {
                    // println!("stacktrace: {}", PrintVec(self.stack.clone(&gc)));
                    let stack_len = self.stack.len() - args_count - 1;
                    // let frame = self.frames.last().unwrap();
                    if let Object::Closure(func) = &self.stack[stack_len] {
                        if func.arity != args_count as u32 {
                            return Err(LoxError::RuntimeError(
                                "arg cnt fn".to_string(),
                                0,
                                "".to_string(),
                            ));
                        }
                        self.frames
                            .push(CallFrame::new(gc.clone_unique_root(func), 0, stack_len));
                        // println!("upvals {:?}: {:?}", func.name, func.upvalues);
                        // println!("open upvals {:?}: {:?}", func.name, self.open_upvalues);
                    } else if let Object::NativeFunction(func) = self.stack[stack_len].clone(&gc) {
                        // TODO: args count check
                        let ret_val =
                            func(self.to_vec(&self.stack[stack_len..(stack_len + args_count)], gc));

                        for _ in 0..(args_count) {
                            self.stack.pop();
                        }

                        self.stack.push(ret_val);
                    } else if let Object::ClassDef(val) = &self.stack[stack_len] {
                        let mut init = None;
                        // println!("arg {} {}", val.name, args_count);
                        if let Some(Object::Closure(initializer)) =
                            val.get_method(&String::from("init"), gc)
                        {
                            if initializer.arity != args_count as u32 {
                                return Err(LoxError::RuntimeError(
                                    "arg cnt cl".to_string(),
                                    0,
                                    "".to_string(),
                                ));
                            }
                            init = Some(initializer);
                        } else if args_count != 0 {
                            return Err(LoxError::RuntimeError(
                                "arg cnt cli".to_string(),
                                0,
                                "".to_string(),
                            ));
                        }
                        self.replace_top_stack(
                            Object::InstanceDef(gc.get_root(Instance::new(val.clone(gc)))),
                            args_count,
                        );
                        if let Some(func) = &init {
                            // TODO: Args are not parsed
                            self.frames.push(CallFrame::new(
                                gc.clone_unique_root(func),
                                0,
                                stack_len,
                            ));
                        }
                        // TODO: arg count should be zero here
                    } else if let Object::InstanceBindDef(val) = &self.stack[stack_len] {
                        if val.method.arity != args_count as u32 {
                            return Err(LoxError::RuntimeError(
                                "arg cnt idef".to_string(),
                                0,
                                "".to_string(),
                            ));
                        }
                        self.frames.push(CallFrame::new(
                            gc.clone_unique_root(&val.method),
                            0,
                            stack_len,
                        ));
                        self.replace_top_stack(val.receiver.clone(gc), args_count);
                    } else {
                        return Err(LoxError::RuntimeError(
                            "call".to_string(),
                            0,
                            "".to_string(),
                        ));
                    }
                }
                Closure(line_no, pos) => {
                    if let Object::Closure(func) = self.constant_pool[pos].clone(&gc) {
                        for up_val in func.upvalues.iter() {
                            let (index, is_local) = &up_val;
                            if *is_local {
                                // println!("up open {:?} {:?}", func.name, self.frames.last().unwrap().slot + index);
                                // check first if an upvalue thing exists already for this particular local. if yes, don't add the following.
                                func.upvalues_ref.borrow_mut().push(
                                    self.capture_upvalue(
                                        self.frames.last().unwrap().slot + index,
                                        gc,
                                    ),
                                );
                            } else {
                                // println!("{:?}", up_val);
                                func.upvalues_ref.borrow_mut().push(
                                    self.frames
                                        .last()
                                        .unwrap()
                                        .func
                                        .upvalues_ref
                                        .borrow()
                                        .get(*index)
                                        .unwrap()
                                        .clone(&gc),
                                );
                            }
                        }
                        self.push_stack(Object::Closure(func));
                    } else {
                        return Err(LoxError::RuntimeError("cls".to_string(), 0, "".to_string()));
                    }
                }
                Return(line_no) => {
                    //TODO: use slots here
                    let val = self.pop_stack(gc).unwrap();
                    // TODO: static size stacks, we can just change index instead of actually popping
                    let frame_base = self.frames.last().unwrap().slot;
                    // println!("stack before: {} {}", PrintVec(self.stack.clone()), frame_base);
                    let mut x = 0;
                    while self.stack.len() > frame_base {
                        let i = self.stack.len();
                        // println!("ret Pop: {} {}", self.stack.last().unwrap(), i - frame_base);
                        self.close_value(i - 1, gc);
                        // if self.frames.last().unwrap().func.locals[i-frame_base].is_closed {
                        // }
                        x += 1;
                        self.pop_stack(gc);
                    }

                    //removing the function object
                    // self.pop_stack(gc);
                    self.frames.pop();
                    self.push_stack(val);
                }
                NoOp => {}
                CloseUpvalue => {
                    // let ln = self.frames.last().unwrap().func.upvalues_ref.borrow().len();
                    // for i in (0..ln).rev() {
                    //     let top = self.pop_stack(gc).unwrap();
                    //     match &*self.frames.last().unwrap().func.upvalues_ref.clone().borrow().get(i).unwrap().clone().borrow() {
                    //         UpValue::Open(up_pos) => {
                    //             self.frames.last().unwrap().func.upvalues_ref.clone().borrow_mut()[i] = Rc::new(RefCell::new(UpValue::Closed(top)))
                    //         },
                    //         _ => {}
                    //     }
                    // }
                    let frame_base = self.frames.last().unwrap().slot;
                    let i = self.stack.len() - 1;
                    // println!("cup stacktrace: {} {}", PrintVec(self.stack.clone(&gc)), i);
                    self.close_value(i, gc);
                    self.pop_stack(gc);
                }
                PrintStackTrace => {
                    println!("stacktrace: {}", PrintVec(self.stack.clone(&gc)));
                }
                ClassDef(line_no, pos) => {
                    let name = self.constant_pool[pos].to_string();
                    self.push_stack(Object::ClassDef(gc.get_root(Class::new(name))))
                }
                MethodDef(line_no, pos) => {
                    let method = self.pop_stack(gc).unwrap();
                    if let Object::ClassDef(class) = self.stack.last().unwrap().clone(gc) {
                        // println!("mdef: {:?}", class.name);
                        let prop = self.constant_pool[pos].to_string();
                        class.set_method(prop, method);
                    } else {
                        return Err(LoxError::RuntimeError(
                            "mdef".to_string(),
                            line_no,
                            "".to_string(),
                        ));
                    }
                }
                Inherit(line_no) => {
                    if let Some(Object::ClassDef(child_class)) = self.pop_stack(gc) {
                        if let Some(Object::ClassDef(super_class)) = self.stack.last() {
                            child_class.add_super_class(&super_class, gc);
                        } else {
                            return Err(LoxError::RuntimeError(
                                "inh".to_string(),
                                line_no,
                                "".to_string(),
                            ));
                        }
                    } else {
                        return Err(LoxError::RuntimeError(
                            "inh".to_string(),
                            line_no,
                            "".to_string(),
                        ));
                    }
                }
                GetSuper(line_no, pos) => {
                    //TODO: actually check if it's a string
                    let name = self.constant_pool[pos].to_string();
                    if let Some(Object::ClassDef(super_class)) = self.pop_stack(gc) {
                        if let Some(Object::InstanceDef(inst)) = self.pop_stack(gc) {
                            // if self.globals.insert(name, val.clone(&gc)).is_none() {
                            //     return Err(LoxError::RuntimeError("unknown".to_string(),line_no,"".to_string()))
                            // }
                            self.bind_method(&inst, &super_class, &name, gc)?;
                            // break;
                        }
                    } else {
                        return Err(LoxError::RuntimeError(
                            "gsup".to_string(),
                            line_no,
                            "".to_string(),
                        ));
                    }
                }
            };
        }
    }

    fn to_vec(&self, slice: &[Object], gc: &Heap) -> Vec<Object> {
        let mut res = vec![];
        for it in slice {
            res.push(it.clone(gc))
        }
        res
    }

    fn close_value(&mut self, ind: usize, gc: &Heap) {
        let ln = self.open_upvalues.borrow().len();
        for i in (0..ln).rev() {
            let top = self.stack.last().unwrap().clone(&gc);
            let mut up_pos = -1;

            match self
                .open_upvalues
                .clone(&gc)
                .borrow()
                .get(i)
                .unwrap()
                .clone(&gc)
                .get(&gc)
            {
                UpValue::Open(tmp) => {
                    up_pos = tmp as i32;
                }
                _ => {}
            }
            if up_pos == ind as i32 {
                self.open_upvalues
                    .borrow()
                    .get(i)
                    .unwrap()
                    .update(UpValue::Closed(top));
                // TODO:
                self.open_upvalues.borrow_mut().remove(i);
            }
        }
    }

    pub fn push_stack(&mut self, val: Object) {
        self.sp += 1;
        self.stack.push(val);
    }

    pub fn replace_top_stack(&mut self, val: Object, pos: usize) {
        let len = self.stack.len();
        self.stack[len - pos - 1] = val;
    }

    pub fn pop_stack(&mut self, gc: &Heap) -> Option<Object> {
        if self.sp == 0 {
            None
        } else {
            self.sp -= 1;
            let x = self.stack.pop().clone(&gc);
            return x;
        }
    }

    pub fn capture_upvalue(&mut self, pos: usize, gc: &Heap) -> Root<UpValueWrap> {
        // use std::borrow::Borrow;
        for val in self.open_upvalues.borrow().iter() {
            if let UpValue::Open(pos1) = val.get(&gc) {
                if pos1 == pos {
                    return val.clone(&gc);
                }
            }
        }
        let x = gc.get_root(UpValueWrap::new(UpValue::Open(pos)));
        let y = x.clone(&gc);
        self.open_upvalues.borrow_mut().push(y);
        x
    }

    fn bind_method(
        &mut self,
        inst: &Root<Instance>,
        class: &Root<Class>,
        prop: &String,
        gc: &Heap,
    ) -> Result<(), LoxError> {
        if let Some(Object::Closure(method)) = class.get_method(prop, gc) {
            let bound = InstanceBoundMethod::new(Object::InstanceDef(gc.clone_root(inst)), method);
            self.push_stack(Object::InstanceBindDef(gc.get_root(bound)));
            Ok(())
        } else {
            Err(LoxError::RuntimeError("bm".to_string(), 0, "".to_string()))
        }
    }
}
