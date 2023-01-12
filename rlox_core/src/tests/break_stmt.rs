use super::*;
use crate::error::LoxError;
use crate::frontend::lexer::*;
use crate::frontend::parser::Parser;
use crate::frontend::resolver::Resolver;
use crate::runtime::definitions::object::Object;
use crate::runtime::interpreter::Interpreter;
use crate::runtime::system_calls::SystemInterfaceMock;
use logos::{source::Source, Logos};
use std::any::Any;
use std::cell::RefCell;
use std::fs::read_to_string;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

test_fail!(
    class,
    ".././test-scripts/break/class.lox",
    LoxError::ParserError(String::from("break"), 2, String::from(""))
);

test_fail!(
    function,
    ".././test-scripts/break/function.lox",
    LoxError::Break(2)
);

test_fail!(
    global_scope,
    ".././test-scripts/break/global_scope.lox",
    LoxError::Break(1)
);

test_fail!(
    local_scope,
    ".././test-scripts/break/local_scope.lox",
    LoxError::Break(2)
);

test_succeed!(loops, ".././test-scripts/break/loop.lox", 1, 2);
