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
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

test_succeed!(after_else, ".././test-scripts/return/after_else.lox", "ok");

test_succeed!(after_if, ".././test-scripts/return/after_if.lox", "ok");

test_succeed!(
    after_while,
    ".././test-scripts/return/after_while.lox",
    "ok"
);

test_fail!(
    at_top_level,
    ".././test-scripts/return/at_top_level.lox",
    LoxError::SemanticError(String::from("return"), 1, String::from(""))
);

test_succeed!(
    in_function,
    ".././test-scripts/return/in_function.lox",
    "ok"
);

test_succeed!(in_method, ".././test-scripts/return/in_method.lox", "ok");

test_succeed!(
    return_nil_if_no_value,
    ".././test-scripts/return/return_nil_if_no_value.lox",
    Object::Nil
);
