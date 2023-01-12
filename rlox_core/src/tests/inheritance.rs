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

test_succeed!(
    constructor,
    ".././test-scripts/inheritance/constructor.lox",
    "value"
);

test_fail!(
    inherit_from_function,
    ".././test-scripts/inheritance/inherit_from_function.lox",
    LoxError::RuntimeError(String::from("Subclass"), 3, String::from(""))
);

test_fail!(
    inherit_from_nil,
    ".././test-scripts/inheritance/inherit_from_nil.lox",
    LoxError::RuntimeError(String::from("Foo"), 2, String::from(""))
);

test_fail!(
    inherit_from_number,
    ".././test-scripts/inheritance/inherit_from_number.lox",
    LoxError::RuntimeError(String::from("Foo"), 2, String::from(""))
);

test_succeed!(
    inherit_methods,
    ".././test-scripts/inheritance/inherit_methods.lox",
    "foo",
    "bar",
    "bar"
);

test_fail!(
    parenthesized_superclass,
    ".././test-scripts/inheritance/parenthesized_superclass.lox",
    LoxError::ParserError(String::from("("), 4, String::from(""))
);

test_succeed!(
    set_fields_from_base_class,
    ".././test-scripts/inheritance/set_fields_from_base_class.lox",
    "foo 1",
    "foo 2",
    "bar 1",
    "bar 2",
    "bar 1",
    "bar 2"
);
