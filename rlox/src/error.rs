use crate::interpreter::Object;
use std::fmt::Display;

// make return, break into another enum and extend
#[derive(Debug, Clone)]
pub enum LoxError {
    ScannerError(String, u32),
    ParserError(String, u32),
    RuntimeError(String, u32),
    SemanticError(String, u32),
    ReturnVal(Object, u32),
    Break(u32),
    Continue(u32),
}

impl Display for LoxError {
    fn fmt(
        &self,
        writer: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            LoxError::ScannerError(msg, line_no) => {
                writer.write_fmt(format_args!("[ScannerError:L{}] {}", line_no, msg))
            }
            LoxError::ParserError(msg, line_no) => {
                writer.write_fmt(format_args!("[ParserError:L{}] {}", line_no, msg))
            }
            LoxError::RuntimeError(msg, line_no) => {
                writer.write_fmt(format_args!("[RuntimeError:L{}] {}", line_no, msg))
            }
            LoxError::SemanticError(msg, line_no) => {
                writer.write_fmt(format_args!("[SemanticError:L{}] {}", line_no, msg))
            }
            _ => panic!("Handle Types cannot be displayed"),
        }
    }
}

impl std::convert::From<std::time::SystemTimeError> for LoxError {
    fn from(err: std::time::SystemTimeError) -> Self {
        LoxError::RuntimeError(err.to_string().to_string(), 0)
    }
}

