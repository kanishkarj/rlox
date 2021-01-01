// use crate::runtime::definitions::object::Object;
// use crate::frontend::lexer::*;
use crate::{chunk::Object, gc::{heap::Heap, root::CustomClone}};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::SystemTime;
use rlox_core::error::LoxError;

pub trait SystemCalls {
    fn print(&mut self, arg: &Object, gc: &Heap);
    fn time(&mut self) -> Result<Object, LoxError>;
    fn print_error(&mut self, err: LoxError);
}

pub struct SystemInterface();
pub struct SystemInterfaceMock {
    pub print_cache: Rc<RefCell<Vec<Object>>>,
}

impl SystemCalls for SystemInterface {
    fn print(&mut self, arg: &Object, gc: &Heap) {
        println!("[print] {}", arg);
    }

    fn time(&mut self) -> Result<Object, LoxError> {
        let curr_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(Object::Num(curr_time.as_millis() as f64))
    }

    fn print_error(&mut self, err: LoxError) {
        println!("[error] {}", err);
    }
}

impl SystemCalls for SystemInterfaceMock {
    fn print(&mut self, arg: &Object, gc: &Heap) {
        match arg {
            Object::ClassDef(val) => self.print_cache.borrow_mut().push(Object::Str(val.name.clone())),
            Object::InstanceDef(val) => self.print_cache.borrow_mut().push(Object::Str(val.name.clone())),
            _ => self.print_cache.borrow_mut().push(arg.clone(gc)),
        };
    }

    fn time(&mut self) -> Result<Object, LoxError> {
        Ok(Object::Num(0.0))
    }

    fn print_error(&mut self, err: LoxError) {}
}
