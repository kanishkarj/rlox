use std::collections::HashMap;
use crate::interpreter::Object;
use crate::scanner::LoxError;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct EnvInner {
    parent: Option<Environment>,
    values: HashMap<String,Object>,
}



impl EnvInner {
    pub fn define(&mut self, name: String, val: Object) {
        self.values.insert(name, val);
    }

    pub fn assign(&mut self, name: String, val: Object) -> bool {
            return if self.values.contains_key(&name) {
                self.values.insert(name, val);
                true
            } else if let Some(parent) = &mut self.parent {
                parent.env.borrow_mut().assign(name, val);
                true
            } else {
                false
            }
        
    }

    pub fn get(&mut self, name: String) -> Option<Object> {
            return if self.values.contains_key(&name) {
                let val = self.values.remove(&name).unwrap();
                self.values.insert(name, val.clone());
                return Some(val);
            } else if let Some(parent) = &mut self.parent {
                parent.env.borrow_mut().get(name)
            } else {
                None
            }
    }

    pub fn new() -> Self {
        EnvInner {
            values: HashMap::new(),
            parent: None
        }
    }

    pub fn form(parent: Environment) -> Self {
        EnvInner {
            values: HashMap::new(),
            parent: Some(parent)
        }
    }
}

#[derive(Debug,Clone)]
pub struct Environment {
    env: Rc<RefCell<EnvInner>>
}

impl Environment {
    
    pub fn new() -> Self {
        Environment {
            env: Rc::from(RefCell::new(EnvInner::new()))
        }
    }
    pub fn form(parent: Environment) -> Self {
        Environment {
            env: Rc::from(RefCell::new(EnvInner::form(parent)))
        }
    }
    // pub fn wrap(to_wrap: Environment) -> Self {
    //     Environment {
    //         env: Rc::from(to_wrap.env)
    //     }
    // }
    pub fn assign(&mut self, name: String, val: Object) -> bool {
        self.env.borrow_mut().assign(name, val)
    }

    pub fn get(&mut self, name: String) -> Option<Object> {
        self.env.borrow_mut().get(name)   
    }
    pub fn define(&mut self, name: String, val: Object) {
        self.env.borrow_mut().define(name, val)
    }
}