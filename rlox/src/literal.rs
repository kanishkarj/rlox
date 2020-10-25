use crate::interpreter::Object;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    NUM(f64),
    STRING(String),
    BOOL(bool),
    NIL,
}

impl Display for Literal {
    fn fmt(
        &self,
        writer: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Literal::NUM(val) => writer.write_str(&val.to_string()),
            Literal::STRING(val) => writer.write_str(&val.to_string()),
            Literal::BOOL(val) => writer.write_str(&val.to_string()),
            Literal::NIL => writer.write_str("Nil"),
        }
    }
}

