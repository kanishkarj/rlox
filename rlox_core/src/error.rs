use crate::runtime::definitions::object::Object;
use std::fmt::Display;

// make return, break into another enum and extend
#[derive(Debug, Clone)]
pub enum LoxError {
    ScannerError(String, u32, String),
    ParserError(String, u32, String),
    RuntimeError(String, u32, String),
    SemanticError(String, u32, String),
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
            LoxError::ScannerError(literal, line_no, msg) => writer.write_fmt(format_args!(
                "[ScannerError:L{}:{}] {}",
                line_no, literal, msg
            )),
            LoxError::ParserError(literal, line_no, msg) => writer.write_fmt(format_args!(
                "[ParserError:L{}:{}] {}",
                line_no, literal, msg
            )),
            LoxError::RuntimeError(literal, line_no, msg) => writer.write_fmt(format_args!(
                "[RuntimeError:L{}:{}] {}",
                line_no, literal, msg
            )),
            LoxError::SemanticError(literal, line_no, msg) => writer.write_fmt(format_args!(
                "[SemanticError:L{}:{}] {}",
                line_no, literal, msg
            )),
            LoxError::ReturnVal(_, line_no) => writer.write_fmt(format_args!(
                "[Improper return:L{}] {}",
                line_no, "Return statements allowed only inside function/lambdas."
            )),
            LoxError::Break(line_no) => writer.write_fmt(format_args!(
                "[Improper break:L{}] {}",
                line_no, "break statements allowed only inside loops."
            )),
            LoxError::Continue(line_no) => writer.write_fmt(format_args!(
                "[Improper continue:L{}] {}",
                line_no, "continue statements allowed only inside loops."
            )),
        }
    }
}

impl std::convert::From<std::time::SystemTimeError> for LoxError {
    fn from(err: std::time::SystemTimeError) -> Self {
        LoxError::RuntimeError(err.to_string().to_string(), 0, "".to_string())
    }
}

impl std::convert::From<std::num::ParseFloatError> for LoxError {
    fn from(err: std::num::ParseFloatError) -> Self {
        LoxError::RuntimeError(err.to_string().to_string(), 0, "".to_string())
    }
}
