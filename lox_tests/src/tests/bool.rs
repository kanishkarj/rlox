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

test_succeed!(not, "../test-scripts/bool/not.lox", false, true, true);

test_succeed!(
    equality,
    "../test-scripts/bool/equality.lox",
    true,
    false,
    false,
    true,
    false,
    false,
    false,
    false,
    false,
    false,
    true,
    true,
    false,
    true,
    true,
    true,
    true,
    true
);
