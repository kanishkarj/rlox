use super::*;
use crate::error::LoxError;
use crate::frontend::lexer::*;
use crate::frontend::parser::Parser;
use crate::frontend::resolver::Resolver;
use crate::runtime::definitions::lox_class::{LoxClass, LoxInstance};
use crate::runtime::definitions::object::Object;
use crate::runtime::interpreter::Interpreter;
use crate::runtime::system_calls::SystemInterfaceMock;
use logos::{source::Source, Logos};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
test_succeed!(closure, ".././test-scripts/this/closure.lox", "Foo");

test_succeed!(
    nested_class,
    ".././test-scripts/this/nested_class.lox",
    LoxInstance::new(LoxClass::new(
        String::from("Outer"),
        Rc::new(HashMap::new()),
        None
    )),
    LoxInstance::new(LoxClass::new(
        String::from("Outer"),
        Rc::new(HashMap::new()),
        None
    )),
    LoxInstance::new(LoxClass::new(
        String::from("Inner"),
        Rc::new(HashMap::new()),
        None
    ))
);

test_succeed!(
    nested_closure,
    ".././test-scripts/this/nested_closure.lox",
    "Foo"
);

test_fail!(
    this_at_top_level,
    ".././test-scripts/this/this_at_top_level.lox",
    LoxError::SemanticError(String::from("this"), 1, String::from(""))
);

test_succeed!(
    this_in_method,
    ".././test-scripts/this/this_in_method.lox",
    "baz"
);

test_fail!(
    this_in_top_level_function,
    ".././test-scripts/this/this_in_top_level_function.lox",
    LoxError::SemanticError(String::from("this"), 2, String::from(""))
);
