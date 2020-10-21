use crate::interpreter::Object;
use crate::scanner::*;
use std::time::SystemTime;
use std::rc::Rc;
use std::cell::RefCell;
pub trait SystemCalls {
    fn print(&mut self, arg: &Object);
    fn time(&mut self,) -> Result<Object, LoxError>;
}

pub struct SystemInterface();
pub struct SystemInterfaceMock {
    pub print_cache: Rc<RefCell<Vec<Object>>>
}

impl SystemCalls for SystemInterface {

    fn print(&mut self, arg: &Object) {
        println!("[print] {}", arg);
    }
    
    fn time(&mut self) -> Result<Object, LoxError> {
        let curr_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(Object::Num(curr_time.as_millis() as f64))
    }

}

impl SystemCalls for SystemInterfaceMock {

    fn print(&mut self, arg: &Object) {
        self.print_cache.borrow_mut().push(arg.clone());
    }
    
    fn time(&mut self) -> Result<Object, LoxError> {
        Ok(Object::Num(0.0))
    }

}