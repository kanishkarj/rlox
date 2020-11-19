use crate::frontend::definitions::stmt::Function;
use crate::runtime::environment::LocalEnvironment;
use std::cell::RefCell;
use crate::runtime::definitions::lox_class::LoxInstance;
use crate::runtime::interpreter::{Interpreter};
use crate::frontend::definitions::expr::Lambda;
use crate::runtime::definitions::lox_callable::LoxCallable;
use crate::error::LoxError;
use std::rc::Rc;
use crate::runtime::definitions::object::Object;

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: RefCell<Function>,
    closure: LocalEnvironment,
    is_init: bool,
}

impl LoxFunction {
    pub fn new(declaration: Function, closure: LocalEnvironment, is_init: bool) -> Self {
        LoxFunction {
            declaration: RefCell::new(declaration),
            closure,
            is_init,
        }
    }
    pub fn bind(&self, inst: Rc<LoxInstance>) -> Self {
        let env = LocalEnvironment::build(self.closure.clone());
        env.define_at("this".to_string(), Object::Instance(inst), 0);
        LoxFunction::new(self.declaration.borrow().clone(), env, self.is_init)
    }
}

#[derive(Debug, Clone)]
pub struct LoxLambda {
    declaration: RefCell<Lambda>,
    closure: LocalEnvironment,
}

impl LoxLambda {
    pub fn new(declaration: Lambda, closure: LocalEnvironment) -> Self {
        LoxLambda {
            declaration: RefCell::new(declaration),
            closure,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, intrprt: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError> {
        let env = LocalEnvironment::build(self.closure.clone());
        for (param, arg) in self.declaration.borrow().params.iter().zip(args) {
            env.define_at(param.lexeme.clone(), arg, 0);
        }
        let val = intrprt.execute_block(&self.declaration.borrow().body, env);
        if let Err(LoxError::ReturnVal(val, _)) = val {
            if self.is_init {
                return Ok(self
                    .closure
                    .get_at("this".to_string(), 0)
                    .unwrap_or(Object::Nil));
            }
            return Ok(val);
        }
        if self.is_init {
            return Ok(self.closure.get_at("this".to_string(), 0).unwrap());
        }
        return val;
    }
    fn arity(&self) -> usize {
        self.declaration.borrow().params.len()
    }
    fn get_name(&self) -> String {
        self.declaration.borrow().name.lexeme.clone()
    }
}

impl LoxCallable for LoxLambda {
    fn call(&self, intrprt: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError> {
        let env = LocalEnvironment::build(self.closure.clone());
        for (param, arg) in self.declaration.borrow().params.iter().zip(args) {
            env.define_at(param.lexeme.clone(), arg, 0);
        }
        let val = intrprt.execute_block(&self.declaration.borrow_mut().body, env);
        if let Err(LoxError::ReturnVal(val, _)) = val {
            return Ok(val);
        }
        val
    }
    fn arity(&self) -> usize {
        self.declaration.borrow().params.len()
    }
    fn get_name(&self) -> String {
        String::from("Lambda")
    }
}

