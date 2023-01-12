use rlox_core::frontend::lexer::*;
use rlox_core::frontend::parser::Parser;
use rlox_core::frontend::resolver::Resolver;
use rlox_core::runtime::interpreter::Interpreter;
use std::fs::read_to_string;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
// use rlox_core::runtime::system_calls::SystemInterfaceMock;
// use logos::{source::Source, Logos};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
// use rlox_core::runtime::definitions::object::Object;
use super::*;
use rlox_core::error::LoxError;
use rlox_core::runtime::definitions::lox_class::{LoxClass, LoxInstance};
use std::collections::HashMap;
test_succeed!(closure, ".././test-scripts/this/closure.lox", "Foo");

test_succeed!(
    nested_class,
    ".././test-scripts/this/nested_class.lox",
    Class::new(String::from("Outer")),
    Class::new(String::from("Outer")),
    Class::new(String::from("Inner"))
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
