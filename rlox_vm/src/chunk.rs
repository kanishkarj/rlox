
use std::cell::RefCell;
use std::rc::Rc;
use std::time::SystemTime;
use std::fmt::Error;
use std::fmt::Formatter;
use rlox_core::error::LoxError;
use rlox_core::frontend::definitions::literal::Literal;
use rlox_core::frontend::definitions::token::Token;
// use crate::debug::disassemble_inst;
use std::collections::HashMap;
use std::ops::{Add,Mul,Div,Sub};
use std::fmt::Display;
use crate::system_calls::SystemCalls;

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
*/

#[derive(Debug, Clone)]
pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
    // TODO: non closures can be made functions instead of closures
    Function(FuncSpec),
    NativeFunction(NativeFn),
    Closure(Box<FuncSpec>),
}

#[derive(Debug, Clone)]
pub enum UpValue{
    Open(usize),
    Closed(Object)
}
#[derive(Debug, Clone)]
pub struct UpValueWrap(pub RefCell<UpValue>);

impl UpValueWrap {
    pub fn new (upval: UpValue) -> Self {
        UpValueWrap{0:RefCell::new(upval)}
    }
    pub fn update (&self, upval: UpValue){
       self.0.replace(upval);
    }
    pub fn get(&self) -> UpValue {
        self.0.borrow().clone()
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
            Object::Function(val) => writer.write_fmt(format_args!("Function<{:?}>", val.name)),
            Object::NativeFunction(_) => writer.write_fmt(format_args!("NativeFunction<>")),
            Object::Closure(val) => writer.write_fmt(format_args!("Closure<{:?}>", val.name)),
        }
    }
}

impl Object {
    pub fn add(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Num(l+r)),
            (Str(ref l), Str(ref r)) => {let mut l = l.clone(); l.push_str(r); Ok(Object::Str(l))},
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn sub(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Num(l-r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn mul(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Num(l*r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn div(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Num(l/r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn gt(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Bool(l>r)),
            (Str(ref l), Str(ref r)) => Ok(Object::Bool(l>r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn gte(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Bool(l>=r)),
            (Str(ref l), Str(ref r)) => Ok(Object::Bool(l>=r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn lt(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            
            (Num(ref l), Num(ref r)) => Ok(Object::Bool(l<r)),
            (Str(ref l), Str(ref r)) => Ok(Object::Bool(l<r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn lte(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Bool(l<=r)),
            (Str(ref l), Str(ref r)) => Ok(Object::Bool(l<=r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn bool_or(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Bool(ref l), Bool(ref r)) => Ok(Object::Bool(*l || *r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Boolean".to_string(),
            ))}
        }
    }
    pub fn bool_and(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Bool(ref l), Bool(ref r)) => Ok(Object::Bool(*l && *r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Boolean".to_string(),
            ))}
        }
    }
    pub fn neg(&self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self) {
            (Num(ref l)) => Ok(Object::Num(- *l)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                self.to_string(),
                line_no,
                "Operands not Num".to_string(),
            ))}
        }
    }
    
    pub fn not(&self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self) {
            (Bool(ref l)) => Ok(Object::Bool(! *l)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                self.to_string(),
                line_no,
                "Operands not Boolean".to_string(),
            ))}
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
#[derive(Debug, Clone)]
pub struct FuncSpec {
    pub arity: u32,
    pub chunks: Vec<OpCode>,
    pub name: Option<String>,
    pub fn_type: FunctionType,
    pub locals: Vec<Local>,
    pub scope_depth: i32,
    // index, isLocal
    pub upvalues: Vec<(usize, bool)>,
    pub upvalues_ref: RefCell<Vec<Rc<UpValueWrap>>>,
}

impl FuncSpec{ 
    pub fn new(arity: u32, name: Option<String>, fn_type: FunctionType) -> Self {
        FuncSpec {
            arity, chunks: vec![], name, fn_type, locals: vec![], scope_depth: 0, upvalues: vec![], upvalues_ref: RefCell::new(vec![])
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

    pub fn add_upvalue(&mut self, index:usize, is_local: bool) -> usize {
        for i in 0..self.upvalues.len() {
            if self.upvalues[i] == (index, is_local) {
                return i;
            }
        }
        self.upvalues.push((index, is_local));
        self.upvalues.len() - 1
    }
}

#[derive(Debug, Clone)]
pub enum FunctionType {
    FUNCTION,
    SCRIPT,
}

// TODO: ensure every primitive Object is immutable
// TODO: runner interface which both the vm and treewalker can implement

pub enum VmErr {
    CompileError,
    RuntimeError
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

    //Control Flow
    JumpIfFalse(u32, usize),
    Jump(u32, usize),

    //Fn
    Call(u32, usize),
    Closure(u32, usize),

    //Helpers
    StackPop,
    CloseUpvalue,
    NilVal,
    NoOp,
    PrintStackTrace,
}

macro_rules! binary_op {
    ($self:ident, $op:ident, $line_no:ident) => {
        {
                let b = $self.pop_stack().unwrap(); 
               let a = $self.pop_stack().unwrap();
               let x = a.$op(&b,$line_no)?;
               $self.push_stack(x);
        }
    }
}

struct PrintVec(Vec<Object>);

impl Display for PrintVec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if(self.0.len() < 1) {
            return Ok(())
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

struct CallFrame {
    func: Box<FuncSpec>,
    ip: usize,
    slot: usize
}

impl CallFrame {
    pub fn new(func: Box<FuncSpec>, ip: usize, slot: usize) -> Self {
        CallFrame {
            func,
            ip,
            slot,
        }
    }
}

pub struct VM<T: SystemCalls> {
    frames: Vec<CallFrame>,

    constant_pool: Vec<Object>,
    stack: Vec<Object>,
    sp: usize,
    globals: HashMap<String, Object>,
    open_upvalues: RefCell<Vec<Rc<UpValueWrap>>>,
    sys_interface: T
}

impl<T: SystemCalls> VM<T> {
    pub fn new(sys_interface: T, constant_pool: Vec<Object>, func: FuncSpec) -> Self {
        let mut vm = VM {
            constant_pool,
            stack: vec![],
            sp:0,
            globals: HashMap::new(),
            frames: vec![CallFrame::new(Box::new(func),0,0)],
            open_upvalues: RefCell::new(vec![]),
            sys_interface
        };

        vm.define_native_fn("clock", |_| {
            let curr_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
            Object::Num(curr_time.as_millis() as f64)
        });

        vm.define_native_fn("native_add", |args| {
            args[0].add(&args[1],0).unwrap()
        });

        vm
    }

    pub fn define_native_fn<S: AsRef<str>>(&mut self, name: S, fn_def: NativeFn) {
        self.globals.insert(name.as_ref().to_string(), Object::NativeFunction(fn_def));
    }

    //TODO: try prefetching
//TODO: this whole thing barely does any error handling
    pub fn run(&mut self, is_debug: bool) -> Result<(), LoxError>{
        use OpCode::*;

        // let frame = self.frames.last_mut().unwrap();

        loop {
            //TODO: handle debugging
            // if is_debug {
            //     disassemble_inst(&self, self.ip);
            // }
            let ip = self.frames.last_mut().unwrap().ip;
            self.frames.last_mut().unwrap().ip += 1;

            // println!("exec: {:?}", self.frames.last_mut().unwrap().func.chunks[ip]);
            match self.frames.last_mut().unwrap().func.chunks[ip] {
                Constant(pos) => {
                    // println!("const: {:?}", self.constant_pool[pos]);
                // this will create new copies everytime. think over
                self.push_stack(self.constant_pool[pos].clone())},
                Exit(_) => {
                    // println!("{:?}", self.stack);
                    return Ok(())
                },
                Negate(line_no) => {
                    let a = self.pop_stack().unwrap();
                    self.push_stack(a.neg(line_no)?);
                },
                Not(line_no) => {
                    let a = self.pop_stack().unwrap();
                    self.push_stack(a.not(line_no)?);
                },
                Add(line_no) => {
                    binary_op!(self,add,line_no)
                },
                Divide(line_no) => {
                    binary_op!(self,div,line_no)
                },
                Multiply(line_no) => {
                    binary_op!(self,mul,line_no)
                },
                Subs(line_no) => {
                    binary_op!(self,sub,line_no)
                },
                GreaterThan(line_no) => {binary_op!(self,gt,line_no)},
                GreaterThanEq(line_no) => {binary_op!(self,gte,line_no)},
                LesserThan(line_no) => {binary_op!(self,lt,line_no)},
                LesserThanEq(line_no) => {binary_op!(self,lte,line_no)},
                NotEqualTo(_) => {let val = self.pop_stack().unwrap() != self.pop_stack().unwrap(); self.push_stack(Object::Bool(val));},
                EqualTo(_) => {let val = self.pop_stack().unwrap() == self.pop_stack().unwrap(); self.push_stack(Object::Bool(val));},
                BoolOr(line_no) => {binary_op!(self,bool_or,line_no)},
                BoolAnd(line_no) => {binary_op!(self,bool_and,line_no)},
                NilVal => {self.push_stack(Object::Nil)},
                Print(_) => {
                    let x = self.pop_stack().unwrap();
                    self.sys_interface.print(&x);
                },
                StackPop => {self.pop_stack();},
                DefineGlobal(line_no, pos) => {
                    let name = self.constant_pool[pos].to_string();
                    if let Some(val) = self.pop_stack() {
                        self.globals.insert(name, val.clone());
                    } else {
                        return Err(LoxError::RuntimeError("gg".to_string(),line_no,"".to_string()))
                    }
                }
                GetGlobal(line_no, pos) => {
                    //TODO: actually check if it's a string
                    let name = self.constant_pool[pos].to_string();
                    if let Some(val) = self.globals.get(&name) {
                        self.push_stack(val.clone());
                    } else {
                        return Err(LoxError::RuntimeError("gg".to_string(),line_no,"".to_string()))
                    }
                }
                SetGlobal(line_no, pos) => {
                    //TODO: actually check if it's a string
                    let name = self.constant_pool[pos].to_string();
                    if let Some(val) = self.stack.last() {
                        if self.globals.insert(name, val.clone()).is_none() {
                            return Err(LoxError::RuntimeError("unknown".to_string(),line_no,"".to_string()))
                        }
                    } else {
                        return Err(LoxError::RuntimeError("gg".to_string(),line_no,"".to_string()))
                    }
                }
                GetLocal(_, pos) => {
                    if let Some(val) = self.stack.get(self.frames.last_mut().unwrap().slot + pos) {
                        self.push_stack(val.clone());
                    } else {
                        return Err(LoxError::RuntimeError("gl".to_string(),0,"".to_string()))
                    }
                }
                SetLocal(_, pos) => {
                    if let Some(val) = self.stack.last() {
                        self.stack[self.frames.last_mut().unwrap().slot + pos] = val.clone();
                    } else {
                        return Err(LoxError::RuntimeError("sl".to_string(),0,"".to_string()))
                    }
                }
                SetUpvalue(_, pos) => {
                    if let Some(val) = self.stack.last() {
                        let x = self.frames.last().unwrap().func.upvalues_ref.borrow().get(pos).unwrap().clone().get();
                        match x {
                            UpValue::Open(up_pos) => {
                                self.stack[up_pos] = val.clone();
                            },
                            UpValue::Closed(_) => {
                                self.frames.last().unwrap().func.upvalues_ref.borrow_mut()[pos].update(UpValue::Closed(val.clone()));
                            }
                            _ => {}
                        }
                    } else {
                        return Err(LoxError::RuntimeError("su".to_string(),0,"".to_string()))
                    } 
                }
                GetUpvalue(_, pos) => {
                    // use std::borrow::Borrow; 
                    // let x = self.frames.last().unwrap().func.upvalues_ref.borrow().get(pos).unwrap().clone().borrow();
                    // let x = &*x.get(pos).unwrap().borrow();
                    match self.frames.last().unwrap().func.upvalues_ref.clone().borrow().get(pos).unwrap().clone().get() {
                        UpValue::Open(up_pos) => {
                            // println!("upping Open {:?}", up_pos);
                            self.push_stack(self.stack[up_pos].clone());
                        },
                        UpValue::Closed(val) => {
                            // println!("upping Closed {}", val);
                            self.push_stack(val.clone());
                        }
                        _ => {}
                    }
                }
                JumpIfFalse(_, offset) => {
                    if let Some(Object::Bool(false)) = self.stack.last() {
                        self.frames.last_mut().unwrap().ip = offset;
                    } else if let Some(Object::Bool(true)) = self.stack.last() {
                        
                    } else {
                        return Err(LoxError::RuntimeError("jif".to_string(),0,"".to_string()))
                    }
                }
                Jump(_, offset) => {
                    self.frames.last_mut().unwrap().ip = offset;
                }
                Call(_, args_count) => {
                    // TODO: args count check
                    let stack_len = self.stack.len()-args_count;
                    // let frame = self.frames.last().unwrap();
                    if let Object::Closure(func) = &self.stack[stack_len - 1] {
                        self.frames.push(CallFrame::new(func.clone(),0,stack_len));
                        // println!("upvals {:?}: {:?}", func.name, func.upvalues);
                        // println!("open upvals {:?}: {:?}", func.name, self.open_upvalues);
                    } else if let Object::NativeFunction(func) = self.stack[stack_len - 1].clone() {
                        let ret_val = func(self.stack[stack_len..(stack_len+args_count)].to_vec());
  
                        for _ in 0..(args_count) {
                            self.stack.pop();
                        }
                        
                        self.stack.push(ret_val);
                    } else {
                        return Err(LoxError::RuntimeError("c".to_string(),0,"".to_string()))
                    }
                }
                Closure(_, pos) => {
                    if let Object::Closure(func) = self.constant_pool[pos].clone() {
                        for up_val in func.upvalues.iter() {
                            let (index, is_local) = &up_val;
                            // println!("{:?} {} {}", func.name, self.stack[self.frames.last().unwrap().slot + index], is_local);
                            if *is_local {
                                // println!("up open {:?} {:?}", func.name, self.frames.last().unwrap().slot + index);
                                // check first if an upvalue thing exists already for this particular local. if yes, don't add the following.
                                func.upvalues_ref.borrow_mut().push(self.capture_upvalue(self.frames.last().unwrap().slot + index));
                            } else {
                                // println!("{:?}", up_val);
                                func.upvalues_ref.borrow_mut().push(self.frames.last().unwrap().func.upvalues_ref.borrow().get(*index).unwrap().clone());
                            }
                        }
                        self.push_stack(Object::Closure(func));
                    }
                }
                Return(_) => {
                    // println!("upvals: {:?}", self.open_upvalues);
                    //TODO: use slots here
                    let val = self.pop_stack().unwrap();
                    // TODO: static size stacks, we can just change index instead of actually popping
                    let frame_base = self.frames.last().unwrap().slot;
                    // println!("stack before: {} {}", PrintVec(self.stack.clone()), frame_base);
                    let mut x = 0;
                    while self.stack.len() > frame_base {
                        let i = self.stack.len();
                        // println!("ret Pop: {} {}", self.stack.last().unwrap(), i - frame_base);
                        self.close_value(i - 1);
                        // if self.frames.last().unwrap().func.locals[i-frame_base].is_closed {
                        // }
                        x += 1;
                        self.pop_stack();
                    }
                    
                    //removing the function object
                    self.pop_stack();

                    self.frames.pop();
                    self.push_stack(val);
                },
                NoOp => {

                },
                CloseUpvalue => {
                    // let ln = self.frames.last().unwrap().func.upvalues_ref.borrow().len();
                    // for i in (0..ln).rev() {
                    //     let top = self.pop_stack().unwrap();
                    //     match &*self.frames.last().unwrap().func.upvalues_ref.clone().borrow().get(i).unwrap().clone().borrow() {
                    //         UpValue::Open(up_pos) => {
                    //             self.frames.last().unwrap().func.upvalues_ref.clone().borrow_mut()[i] = Rc::new(RefCell::new(UpValue::Closed(top)))
                    //         },
                    //         _ => {}
                    //     }
                    // }
                    let frame_base = self.frames.last().unwrap().slot;
                    let i = self.stack.len()-1;
                    self.close_value(i);
                    self.pop_stack();
                },
                PrintStackTrace => {
                    println!("stacktrace: {}", PrintVec(self.stack.clone()));
                }
            };
        }
    }

    fn close_value(&mut self, ind: usize) {
        let ln = self.open_upvalues.borrow().len();
        for i in (0..ln).rev() {
            let top = self.stack.last().unwrap().clone();
            let mut up_pos = -1;
            
            match self.open_upvalues.clone().borrow().get(i).unwrap().clone().get() {
                UpValue::Open(tmp) => {
                    up_pos = tmp as i32;
                },
                _ => {}
            }
            if up_pos == ind as i32 {
                self.open_upvalues.borrow().get(i).unwrap().update(UpValue::Closed(top));
                // TODO: 
                self.open_upvalues.borrow_mut().remove(i);
            }
        }
    }

    pub fn push_stack(&mut self, val: Object) {
        self.sp +=1;
        self.stack.push(val);
    }

    pub fn pop_stack(&mut self) -> Option<Object> {
        if self.sp == 0 {
            None
        } else {
            self.sp -= 1;
            let x = self.stack.pop().clone();
            return x;
        }
    }

    pub fn capture_upvalue(&self, pos: usize) -> Rc<UpValueWrap> {
        // use std::borrow::Borrow;
        for val in self.open_upvalues.borrow().iter() {
            if let UpValue::Open(pos1) = val.get() {
                if pos1 == pos {
                    return Rc::clone(val)
                }
            }
        }
        let x = Rc::new(UpValueWrap::new(UpValue::Open(pos)));
        let y = Rc::clone(&x);
        self.open_upvalues.borrow_mut().push(y);
        x
    }
}