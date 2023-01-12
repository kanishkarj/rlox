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
    class_in_body,
    "../test-scripts/for/class_in_body.lox",
    LoxError::ParserError(String::from("class"), 2, String::from(""))
);

test_succeed!(
    closure_in_body,
    "../test-scripts/for/closure_in_body.lox",
    4,
    1,
    4,
    2,
    4,
    3
);

test_fail!(
    fun_in_body,
    "../test-scripts/for/fun_in_body.lox",
    LoxError::ParserError(String::from("foo"), 2, String::from(""))
);

test_succeed!(
    return_closure,
    "../test-scripts/for/return_closure.lox",
    "i"
);

test_succeed!(return_inside, "../test-scripts/for/return_inside.lox", "i");

test_succeed!(scope, "../test-scripts/for/scope.lox", 0, -1, "after", 0);

test_fail!(
    statement_condition,
    "../test-scripts/for/statement_condition.lox",
    LoxError::ParserError(String::from("{"), 3, String::from(""))
);

test_fail!(
    statement_increment,
    "../test-scripts/for/statement_increment.lox",
    LoxError::ParserError(String::from("{"), 2, String::from(""))
);

test_fail!(
    statement_initializer,
    "../test-scripts/for/statement_initializer.lox",
    LoxError::ParserError(String::from("{"), 3, String::from(""))
);

test_succeed!(
    syntax,
    "../test-scripts/for/syntax.lox",
    1,
    2,
    3,
    0,
    1,
    2,
    "done",
    0,
    1,
    0,
    1,
    2,
    0,
    1
);

test_fail!(
    var_in_body,
    "../test-scripts/for/var_in_body.lox",
    LoxError::ParserError(String::from("var"), 2, String::from(""))
);
