
use rlox_core::runtime::definitions::object::Object;
use crate::debug::disassemble_inst;
use std::collections::HashMap;
use std::ops::{Add,Mul,Div,Sub};

const MAX_STACK: usize = 1000;

// TODO: ensure every primitive Object is immutable
// TODO: runner interface which both the vm and treewalker can implement

pub enum VmErr {
    CompileError,
    RuntimeError
}

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Return(u32),
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
               let x = a.$op(&b,0).unwrap();
            //    println!("op: {}", x);
               $self.push_stack(x);
        }
    }
}

pub struct VM {
    chunk: Vec<OpCode>,
    constant_pool: Vec<Object>,
    ip: usize,
    stack: Vec<Object>,
    sp: usize,
    globals: HashMap<String, Object>
}

impl VM {
    pub fn new(chunk: Vec<OpCode>, constant_pool: Vec<Object>) -> Self {
        VM {
            chunk,
            constant_pool,
            ip: 0,
            stack: vec![],
            sp:0,
            globals: HashMap::new()
        }
    }

    pub fn push_inst(&mut self, op: OpCode) -> usize {
        self.chunk.push(op);
        self.len_inst() - 1
    }
    
    pub fn push_const(&mut self, val: Object) -> usize {
        self.constant_pool.push(val);
        self.len_const() - 1
    }

    pub fn len_inst(&self) -> usize {
        self.chunk.len()
    }

    pub fn len_const(&self) -> usize {
        self.constant_pool.len()
    }
    
    pub fn get_inst(&self, offset: usize) -> Option<&OpCode> {
        self.chunk.get(offset)
    }
    
    pub fn get_const(&self, pos: usize) -> Option<&Object> {
        self.constant_pool.get(pos)
    }
    //TODO: try prefetching
//TODO: this whole thing barely does any error handling
    pub fn run(&mut self, is_debug: bool) -> Result<(), VmErr>{
        use OpCode::*;
        loop {
            if is_debug {
                disassemble_inst(&self, self.ip);
            }
            // println!("stack: {:?}", self.stack);
            match self.chunk[self.ip] {
                Constant(_, pos) => {
                    // println!("{}", self.constant_pool[pos]);
                // this will create new copies everytime. think over
                self.push_stack(self.constant_pool[pos].clone())},
                Return(_) => return Ok(()),
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
                NilVal => {todo!();},
                Print(_) => {
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
                    if let Some(val) = self.stack.get(pos) {
                        self.push_stack(val.clone());
                    } else {
                        println!("er gl");
                    }
                }
                SetLocal(_, pos) => {
                    if let Some(val) = self.stack.last() {
                        self.stack[pos] = val.clone();
                    } else {
                        println!("err sl");
                    }
                }
                JumpIfFalse(_, offset) => {
                    if let Some(Object::Bool(false)) = self.stack.last() {
                        self.ip = offset;
                    } else if let Some(Object::Bool(true)) = self.stack.last() {
                        
                    } else {
                        println!("err jmp");
                    }
                }
                Jump(_, offset) => {
                    self.ip = offset;
                }
                NoOp => {

                }
            };
            self.ip += 1;
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