
use crate::frontend::parser::Parser;
use crate::frontend::lexer::*;
use std::fs::read_to_string;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::runtime::interpreter::{Interpreter};
use crate::frontend::resolver::Resolver;
use crate::runtime::system_calls::SystemInterfaceMock;
use logos::{source::Source, Logos};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::definitions::object::Object;
use crate::error::LoxError;
use super::*;

test_succeed!(
    empty_file,
    ".././test-scripts/empty_file.lox"
);

test_succeed!(
    precedence,
    ".././test-scripts/precedence.lox",
    14,
    8,
    4,
    0,
    true,
    true,
    true,
    true,
    0,
    0,
    4
);