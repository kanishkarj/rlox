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
    bool,
    "../test-scripts/call/bool.lox",
    LoxError::RuntimeError(String::from("true"), 1, String::from(""))
);

test_fail!(
    nil,
    "../test-scripts/call/nil.lox",
    LoxError::RuntimeError(String::from("Nil"), 1, String::from(""))
);

test_fail!(
    string,
    "../test-scripts/call/string.lox",
    LoxError::RuntimeError(String::from("str"), 1, String::from(""))
);

test_fail!(
    object,
    "../test-scripts/call/object.lox",
    LoxError::RuntimeError(String::from("Instance<Foo>"), 4, String::from(""))
);
