use crate::error::LoxError;
use crate::runtime::definitions::lox_callable::LoxCallable;
use crate::runtime::definitions::lox_class::{LoxClass, LoxInstance};
use crate::frontend::definitions::literal::Literal;
use std::fmt::Display;
use std::rc::Rc;
// obj.get not handled
#[derive(Debug, Clone)]
pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
    Function(Rc<dyn LoxCallable>),
    Class(Rc<LoxClass>),
    Instance(Rc<LoxInstance>),
}

impl From<Literal> for Object {
    fn from(val: Literal) -> Self {
        match val {
            Literal::NUM(v) => Object::Num(v),
            Literal::STRING(v) => Object::Str(v),
            Literal::BOOL(v) => Object::Bool(v),
            Literal::NIL => Object::Nil,
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        use Object::*;

        match (self, other) {
            (&Str(ref a), &Str(ref b)) => a == b,
            (&Num(ref a), &Num(ref b)) => a == b,
            (&Bool(ref a), &Bool(ref b)) => a == b,
            (&Nil, &Nil) => true,
            _ => false,
        }
    }
}

impl Display for Object {
    fn fmt(
        &self,
        writer: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Object::Str(val) => writer.write_str(&val.to_string()),
            Object::Num(val) => writer.write_str(&val.to_string()),
            Object::Bool(val) => writer.write_str(&val.to_string()),
            Object::Nil => writer.write_str("Nil"),
            Object::Function(val) => writer.write_fmt(format_args!("Function<{}>", val.get_name())),
            Object::Class(val) => writer.write_fmt(format_args!("Class<{}>", &val.name)),
            Object::Instance(val) => {
                writer.write_fmt(format_args!("Instance<{}>", &val.klass.name))
            }
        }
    }
}

impl Object {
    pub fn add(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Num(l+r)),
            (Str(ref l), Str(ref r)) => {let mut l = l.clone(); l.push_str(r); Ok(Object::Str(l))},
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn sub(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Num(l-r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn mul(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Num(l*r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn div(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Num(l/r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn gt(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Bool(l>r)),
            (Str(ref l), Str(ref r)) => Ok(Object::Bool(l>r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn gte(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Bool(l>=r)),
            (Str(ref l), Str(ref r)) => Ok(Object::Bool(l>=r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn lt(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            
            (Num(ref l), Num(ref r)) => Ok(Object::Bool(l<r)),
            (Str(ref l), Str(ref r)) => Ok(Object::Bool(l<r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
    pub fn lte(&self, other: &Self, line_no: u32) -> Result<Self, LoxError>{
        use Object::*;
        match (self, other) {
            (Num(ref l), Num(ref r)) => Ok(Object::Bool(l<=r)),
            (Str(ref l), Str(ref r)) => Ok(Object::Bool(l<=r)),
            // TODO: import error def
            _ => {Err(LoxError::RuntimeError(
                other.to_string(),
                line_no,
                "Operands not Num or String".to_string(),
            ))}
        }
    }
}
impl Eq for Object {}


