use crate::interpreter::{Interpreter, Object};
use crate::error::LoxError;
use crate::environment::LocalEnvironment;
use std::rc::Rc;
use crate::grammar::lox_class::LoxClass;
use crate::grammar::lox_function::{LoxFunction, LoxLambda};

pub trait LoxCallable: LoxCallableClone {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError>;
    fn arity(&self) -> usize;
}

pub trait LoxCallableClone {
    fn clone_box(&self) -> Box<dyn LoxCallable>;
}

impl<T> LoxCallableClone for T
where
    T: 'static + LoxCallable + Clone,
{
    fn clone_box(&self) -> Box<dyn LoxCallable> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn LoxCallable> {
    fn clone(&self) -> Box<dyn LoxCallable> {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn LoxCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "callable")
    }
}

