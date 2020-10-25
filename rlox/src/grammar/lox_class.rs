use crate::grammar::lox_function::LoxFunction;
use std::rc::Rc;
use crate::interpreter::{Object, Interpreter};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::grammar::lox_callable::LoxCallable;
use crate::error::LoxError;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
    methods: Rc<HashMap<String, Rc<LoxFunction>>>,
    super_class: Option<Rc<LoxClass>>,
}

impl LoxClass {
    pub fn new(
        name: String,
        methods: Rc<HashMap<String, Rc<LoxFunction>>>,
        super_class: Option<Rc<LoxClass>>,
    ) -> Self {
        LoxClass {
            name,
            methods,
            super_class,
        }
    }
    pub fn find_method(&self, name: &String) -> Option<Rc<LoxFunction>> {
        if let Some(mth) = self.methods.get(name) {
            return Some(mth).cloned();
        } else {
            if let Some(super_class) = &self.super_class {
                return super_class.find_method(name).clone();
            }
        }
        None
    }
    pub fn bind_method(
        &self,
        name: &Token,
        instance: Rc<LoxInstance>,
    ) -> Result<Rc<LoxFunction>, LoxError> {
        if let Some(mth) = self.methods.get(&name.lexeme) {
            return Ok(Rc::new(mth.bind(instance)));
        } else if let Some(super_class) = &self.super_class {
            if let Some(mth) = super_class.find_method(&name.lexeme) {
                return Ok(Rc::new(mth.bind(instance)));
            }
        }

        Err(LoxError::RuntimeError(
            "Only Instances have properties".to_string(),
            name.line_no,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct LoxInstance {
    pub klass: LoxClass,
    fields: RefCell<HashMap<String, Object>>,
}

impl LoxInstance {
    pub fn new(klass: LoxClass) -> Self {
        LoxInstance {
            klass,
            fields: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, name: &Token) -> Option<Object> {
        self.fields.borrow().get(&name.lexeme).cloned()
    }

    pub fn set(&self, name: &Token, value: Object) {
        self.fields.borrow_mut().insert(name.lexeme.clone(), value);
    }
}

impl LoxCallable for LoxClass {
    fn call(&self, intrprt: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError> {
        let instance = Rc::new(LoxInstance::new(self.clone()));
        if let Some(init) = self.find_method(&"init".to_string()) {
            init.bind(Rc::clone(&instance)).call(intrprt, args)?;
        }
        return Ok(Object::Instance(instance));
    }
    fn arity(&self) -> usize {
        if let Some(init) = self.find_method(&"init".to_string()) {
            init.arity()
        } else {
            0
        }
    }
}

