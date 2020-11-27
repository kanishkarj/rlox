
use std::time::SystemTime;
use std::fmt::Error;
use std::fmt::Formatter;
use rlox_core::error::LoxError;
use rlox_core::frontend::definitions::literal::Literal;
use rlox_core::frontend::definitions::token::Token;
use crate::debug::disassemble_inst;
use std::collections::HashMap;
use std::ops::{Add,Mul,Div,Sub};
use std::fmt::Display;
const MAX_STACK: usize = 1000;

type NativeFn = fn(args: Vec<Object>) -> Object;

/**
 * TODO:
 * print stack trace,
 * special functions for breakpoint, stack trace etc.
 * make stack a static array, it will help keep stack clean when returning from functions.
 * object pooling
 * consider ecs.
*/

#[derive(Debug, Clone)]
pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
    Function(FuncSpec),
    NativeFunction(NativeFn),
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
}
impl Eq for Object {}

#[derive(Debug, Clone)]
pub struct Local {
    pub name: Token,
    pub depth: i32,
}
#[derive(Debug, Clone)]
pub struct FuncSpec {
    pub arity: u32,
    pub chunks: Vec<OpCode>,
    pub name: Option<String>,
    pub fn_type: FunctionType,
    pub locals: Vec<Local>,
    pub scope_depth: i32,
}

impl FuncSpec{ 
    pub fn new(arity: u32, name: Option<String>, fn_type: FunctionType) -> Self {
        FuncSpec {
            arity, chunks: vec![], name, fn_type, locals: vec![], scope_depth: 0
        }
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
    Constant(u32, usize),
    // Unary
    Negate(u32),
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

    //Control Flow
    JumpIfFalse(u32, usize),
    Jump(u32, usize),

    //Fn
    FnCall(u32, usize),

    //Weird
    StackPop,
    NilVal,
    NoOp,
}

macro_rules! binary_op {
    ($self:ident, $op:ident) => {
        {
               let b = $self.pop_stack().unwrap(); 
               let a = $self.pop_stack().unwrap();
            //    println!("operands: {} {}", a , b);
               let x = a.$op(&b,0).unwrap();
            //    println!("op: {}", x);
               $self.push_stack(x);
        }
    }
}

struct PrintVec(Vec<Object>);

impl Display for PrintVec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
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
    func: FuncSpec,
    ip: usize,
    slot: usize
}

impl CallFrame {
    pub fn new(func: FuncSpec, ip: usize, slot: usize) -> Self {
        CallFrame {
            func,
            ip,
            slot,
        }
    }
}

pub struct VM {
    frames: Vec<CallFrame>,

    constant_pool: Vec<Object>,
    stack: Vec<Object>,
    sp: usize,
    globals: HashMap<String, Object>
}

impl VM {
    pub fn new(constant_pool: Vec<Object>, func: FuncSpec) -> Self {
        let mut vm = VM {
            constant_pool,
            stack: vec![],
            sp:0,
            globals: HashMap::new(),
            frames: vec![CallFrame::new(func,0,0)]
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

    pub fn push_inst(&mut self, op: OpCode) -> usize {
        self.frames.last_mut().unwrap().func.chunks.push(op);
        self.len_inst() - 1
    }
    
    pub fn push_const(&mut self, val: Object) -> usize {
        self.constant_pool.push(val);
        self.len_const() - 1
    }

    pub fn len_inst(&self) -> usize {
        self.frames.last().unwrap().func.chunks.len()
    }

    pub fn len_const(&self) -> usize {
        self.constant_pool.len()
    }
    
    pub fn get_inst(&self, offset: usize) -> Option<&OpCode> {
        self.frames.last().unwrap().func.chunks.get(offset)
    }
    
    pub fn get_const(&self, pos: usize) -> Option<&Object> {
        self.constant_pool.get(pos)
    }

    pub fn define_native_fn<S: AsRef<str>>(&mut self, name: S, fn_def: NativeFn) {
        self.globals.insert(name.as_ref().to_string(), Object::NativeFunction(fn_def));
    }

    //TODO: try prefetching
//TODO: this whole thing barely does any error handling
    pub fn run(&mut self, is_debug: bool) -> Result<(), VmErr>{
        use OpCode::*;

        // let frame = self.frames.last_mut().unwrap();

        loop {
            //TODO: handle debugging
            // if is_debug {
            //     disassemble_inst(&self, self.ip);
            // }
            // println!("stack: {:?}", self.stack);
            let ip = self.frames.last_mut().unwrap().ip;
            self.frames.last_mut().unwrap().ip += 1;

            // println!("exec: {:?}", self.frames.last_mut().unwrap().func.chunks[ip]);
            match self.frames.last_mut().unwrap().func.chunks[ip] {
                Constant(_, pos) => {
                    // println!("const: {:?}", self.constant_pool[pos]);
                // this will create new copies everytime. think over
                self.push_stack(self.constant_pool[pos].clone())},
                Exit(_) => {
                    // println!("{:?}", self.stack);
                    return Ok(())
                },
                Negate(_) => {
                    todo!()
                },
                Add(_) => {
                    binary_op!(self,add)
                },
                Divide(_) => {
                    binary_op!(self,div)
                },
                Multiply(_) => {
                    binary_op!(self,mul)
                },
                Subs(_) => {
                    binary_op!(self,sub)
                },
                GreaterThan(_) => {binary_op!(self,gt)},
                GreaterThanEq(_) => {binary_op!(self,gte)},
                LesserThan(_) => {binary_op!(self,lt)},
                LesserThanEq(_) => {binary_op!(self,lte)},
                NotEqualTo(_) => {let val = self.pop_stack().unwrap() != self.pop_stack().unwrap(); self.push_stack(Object::Bool(val));},
                EqualTo(_) => {let val = self.pop_stack().unwrap() == self.pop_stack().unwrap(); self.push_stack(Object::Bool(val));},
                BoolOr(_) => {todo!();},
                BoolAnd(_) => {todo!();},
                NilVal => {self.push_stack(Object::Nil)},
                Print(_) => {
                    // println!("stack {:?}", self.stack);

                    let x = self.pop_stack().unwrap();
                    println!("[print] {}", x);
                },
                StackPop => {self.pop_stack();},
                DefineGlobal(_, pos) => {
                    let name = self.constant_pool[pos].to_string();
                    let val = self.pop_stack().unwrap().clone();
                    self.globals.insert(name, val);
                }
                GetGlobal(_, pos) => {
                    //TODO: actually check if it's a string
                    let name = self.constant_pool[pos].to_string();
                    if let Some(val) = self.globals.get(&name) {
                        self.push_stack(val.clone());
                    } else {
                        println!("errr gg");
                    }
                }
                SetGlobal(_, pos) => {
                    //TODO: actually check if it's a string
                    let name = self.constant_pool[pos].to_string();
                    let val = self.pop_stack().unwrap().clone();
                    self.globals.insert(name, val);
                }
                GetLocal(_, pos) => {
                    // println!("stack: {} | {}", PrintVec(self.stack.clone()), self.frames.last_mut().unwrap().slot + pos);
                    if let Some(val) = self.stack.get(self.frames.last_mut().unwrap().slot + pos) {
                        self.push_stack(val.clone());
                    } else {
                        println!("er gl");
                    }
                }
                SetLocal(_, pos) => {
                    if let Some(val) = self.stack.last() {
                        self.stack[self.frames.last_mut().unwrap().slot + pos] = val.clone();
                    } else {
                        println!("err sl");
                    }
                }
                JumpIfFalse(_, offset) => {
                    if let Some(Object::Bool(false)) = self.stack.last() {
                        self.frames.last_mut().unwrap().ip = offset;
                    } else if let Some(Object::Bool(true)) = self.stack.last() {
                        
                    } else {
                        println!("err jmp");
                    }
                }
                Jump(_, offset) => {
                    self.frames.last_mut().unwrap().ip = offset;
                }
                FnCall(_, args_count) => {
                    // TODO: args count check
                    let stack_len = self.stack.len()-args_count;
                    // let frame = self.frames.last().unwrap();
                    if let Object::Function(func) = &self.stack[stack_len - 1] {
                        self.frames.push(CallFrame::new(func.clone(),0,stack_len));
                    } else if let Object::NativeFunction(func) = self.stack[stack_len - 1].clone() {
                        // TODO: impl native fn calls.
                        let ret_val = func(self.stack[stack_len..(stack_len+args_count)].to_vec());
                        
                        for _ in 0..(args_count+1) {
                            self.stack.pop();
                        }
                        
                        self.stack.push(ret_val);
                    } else {
                        println!("err fnc: {:?}", self.stack.last());
                    }
                }
                Return(_) => {
                    // println!("stack {:?}", self.stack);
                    //TODO: use slots here
                    let val = self.pop_stack().unwrap();
                    while self.stack.len() >= self.frames.last().unwrap().slot {
                        self.stack.pop();
                    }
                    self.frames.pop();
                    self.push_stack(val);
                },
                NoOp => {

                }
            };
        }
    }

    pub fn push_stack(&mut self, val: Object) {
        self.sp +=1;
        // println!("[push] {:?}", val);
        self.stack.push(val);
    }

    pub fn pop_stack(&mut self) -> Option<Object> {
        if self.sp == 0 {
            None
        } else {
            self.sp -= 1;
            let x = self.stack.pop().clone();
            // println!("[pop] {:?}", x);
            return x;
        }
    }
}