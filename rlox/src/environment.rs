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
            } else {
                false
            }
        
    }

    pub fn get(&mut self, name: String) -> Option<Object> {
            return if self.values.contains_key(&name) {
                let val = self.values.remove(&name).unwrap();
                self.values.insert(name, val.clone());
                return Some(val);
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

    pub fn build(parent: Environment) -> Self {
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
    pub fn build(parent: Environment) -> Self {
        Environment {
            env: Rc::from(RefCell::new(EnvInner::build(parent)))
        }
    }
    
    pub fn ancestor(&self, hops: usize) -> Option<Environment> {
        return if hops == 0 {
            Some(self.clone())
        } else if let Some(env) = &mut self.env.borrow_mut().parent {
            return env.ancestor(hops - 1)
        } else {
            None
        }
    }
    
    pub fn assign(&self, name: String, val: Object) -> bool {
        self.env.borrow_mut().assign(name, val)
    }

    pub fn get(&self, name: String) -> Option<Object> {
        self.env.borrow_mut().get(name)   
    }

    pub fn getAt(&self, name: String, hops: usize) -> Option<Object> {
        if let Some(env) =  &mut self.ancestor(hops) {
            return env.get(name)
        }
        None
    }

    pub fn assignAt(&self, name: String, val: Object, hops: usize) -> bool {
        if let Some(env) =  &mut self.ancestor(hops) {
            return env.assign(name, val)
        }
        false
    }

    pub fn define(&self, name: String, val: Object) {
        self.env.borrow_mut().define(name, val)
    }

    pub fn defineAt(&self, name: String, val: Object, hops: usize) {
        if let Some(env) =  &mut self.ancestor(hops) {
            env.define(name, val)
        }
    }

}