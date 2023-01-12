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

test_succeed!(
    associativity,
    "../test-scripts/assignment/associativity.lox",
    "c",
    "c",
    "c"
);

test_succeed!(
    global,
    "../test-scripts/assignment/global.lox",
    "before",
    "after",
    "arg",
    "arg"
);

test_succeed!(
    local,
    "../test-scripts/assignment/local.lox",
    "before",
    "after",
    "arg",
    "arg"
);

test_succeed!(
    syntax,
    "../test-scripts/assignment/syntax.lox",
    "var",
    "var"
);

test_fail!(
    grouping,
    "../test-scripts/assignment/grouping.lox",
    LoxError::RuntimeError(String::from("="), 2, "".to_string())
);

test_fail!(
    infix_operator,
    "../test-scripts/assignment/infix_operator.lox",
    LoxError::RuntimeError(String::from("="), 3, "".to_string())
);

test_fail!(
    prefix_operator,
    "../test-scripts/assignment/prefix_operator.lox",
    LoxError::RuntimeError(String::from("="), 2, "".to_string())
);

test_fail!(
    undefined,
    "../test-scripts/assignment/undefined.lox",
    LoxError::SemanticError(String::from("unknown"), 1, "".to_string())
);

test_fail!(
    to_this,
    "../test-scripts/assignment/to_this.lox",
    LoxError::RuntimeError(String::from("="), 3, "".to_string())
);
