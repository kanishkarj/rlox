use crate::{chunk::Object, gc::{heap::Heap, root::{CustomClone, Trace}}};
use std::{cell::RefCell, collections::HashMap, fmt::Debug};

pub struct Class {
    pub name: String,
    pub methods: RefCell<HashMap<String, Object>>,
}

impl Class {
    pub fn new(name: String) -> Class {
        Class {
            name,
            methods: RefCell::new(HashMap::new()),
        }
    }
    pub fn set_method(&self, k: String, v: Object) {
        self.methods.borrow_mut().insert(k, v);
    }
    pub fn get_method(&self, k: &String, gc: &Heap) -> Option<Object> {
        self.methods.borrow().get(k).map(|v| v.clone(gc))
    }
    pub fn add_super_class(&self, super_class: &Class, gc: &Heap) {
        // self.methods.borrow_mut().extend();
        for (name, method) in super_class.methods.borrow().iter() {
            if !self.methods.borrow().contains_key(name) {
                self.methods.borrow_mut().insert(name.clone(), method.clone(gc));
            }
        }
    }
}

impl Trace for Class {
    fn trace(&mut self) {
        todo!()
    }
}

impl CustomClone for Class {
    fn clone(&self, gc: &crate::gc::heap::Heap) -> Self {
        Class {
            name: self.name.clone(),
            methods: self.methods.clone(gc),
        }
    }
}

impl Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Class<{}>", self.name))
    }
}

