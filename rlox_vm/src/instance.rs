use std::{cell::RefCell, collections::HashMap, ops::{Deref, DerefMut}};

use crate::{chunk::{FuncSpec, Object}, class::Class, gc::{heap::Heap, root::{CustomClone, Root, Trace, UniqueRoot}}};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Instance {
    pub class: Root<Class>,
    fields: RefCell<HashMap<String, Object>>,
}

impl Instance {
    pub fn new(class: Root<Class>) -> Self {
        Instance {
            class,
            fields: RefCell::new(HashMap::new()),
        }
    }
    pub fn set(&self, k: String, v: Object) {
        self.fields.borrow_mut().insert(k, v);
    }
    pub fn get(&self, k: &String, gc: &Heap) -> Option<Object> {
        self.fields.borrow_mut().get(k).map(|v| v.clone(gc))
    }
    pub fn get_class_name(&self) -> String {
        self.class.name.clone()
    }
}

impl Deref for Instance {
    type Target = Root<Class>;

    fn deref(&self) -> &Self::Target {
        &self.class
    }
}

impl DerefMut for Instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.class
    }
}

impl Trace for Instance {
    fn trace(&mut self) {
        todo!()
    }
}

impl CustomClone for Instance {
    fn clone(&self, gc: &crate::gc::heap::Heap) -> Self {
        Instance {
            class: self.class.clone(gc),
            fields: self.fields.clone(gc),
        }
    }
}

#[derive(Debug)]
pub struct InstanceBoundMethod {
    pub receiver: Object,
    pub method: UniqueRoot<FuncSpec>
}


impl Trace for InstanceBoundMethod {
    fn trace(&mut self) {
        todo!()
    }
}

impl CustomClone for InstanceBoundMethod {
    fn clone(&self, gc: &crate::gc::heap::Heap) -> Self {
        InstanceBoundMethod {
            receiver: self.receiver.clone(gc),
            method: self.method.clone(gc),
        }
    }
}

impl InstanceBoundMethod {
    pub fn new(receiver: Object, method: UniqueRoot<FuncSpec>) -> Self {
        InstanceBoundMethod {
            receiver: receiver,
            method: method,
        }
    }
}

