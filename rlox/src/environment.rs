use std::collections::HashMap;
use crate::interpreter::Object;
use crate::scanner::LoxError;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct EnvInner {
    parent: Option<LocalEnvironment>,
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

    pub fn build(parent: LocalEnvironment) -> Self {
        EnvInner {
            values: HashMap::new(),
            parent: Some(parent)
        }
    }
}

#[derive(Debug,Clone)]
pub struct GlobalEnvironment {
    env: Rc<RefCell<EnvInner>>
}
#[derive(Debug,Clone)]
pub struct LocalEnvironment {
    env: Rc<RefCell<EnvInner>>
}

impl LocalEnvironment {
    
    pub fn new() -> Self {
        LocalEnvironment {
            env: Rc::from(RefCell::new(EnvInner::new()))
        }
    }
    pub fn build(parent: LocalEnvironment) -> Self {
        LocalEnvironment {
            env: Rc::from(RefCell::new(EnvInner::build(parent)))
        }
    }
    pub fn ancestor(&self, hops: usize) -> Option<Self> {
        return if hops == 0 {
            Some(self.clone())
        } else if let Some(env) = &mut self.env.borrow_mut().parent {
            return env.ancestor(hops - 1)
        } else {
            None
        }
    }
    pub fn getAt(&self, name: String, hops: usize) -> Option<Object> {
        if let Some(env) =  &mut self.ancestor(hops) {
            return env.env.borrow_mut().get(name)  
        }
        None
    }

    pub fn assignAt(&self, name: String, val: Object, hops: usize) -> bool {
        if let Some(env) =  &mut self.ancestor(hops) {
            return env.env.borrow_mut().assign(name, val)
        }
        false
    }

    pub fn defineAt(&self, name: String, val: Object, hops: usize) {
        if let Some(env) =  &mut self.ancestor(hops) {
            env.env.borrow_mut().define(name, val)
        }
    }

}
impl GlobalEnvironment {
    pub fn new() -> Self {
        GlobalEnvironment {
            env: Rc::from(RefCell::new(EnvInner::new()))
        }
    }
    
    pub fn assign(&self, name: String, val: Object) -> bool {
        self.env.borrow_mut().assign(name, val)
    }

    pub fn get(&self, name: String) -> Option<Object> {
        self.env.borrow_mut().get(name)   
    }

    pub fn define(&self, name: String, val: Object) {
        self.env.borrow_mut().define(name, val)
    }
}

impl From<GlobalEnvironment> for LocalEnvironment {
    fn from(val: GlobalEnvironment) -> Self { 
        LocalEnvironment {
            env: val.env
        }
    }
}