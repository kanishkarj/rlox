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

// test_fail!(
//     error_after_multiline,
//     ".././test-scripts/string/error_after_multiline.lox",
//     LoxError::ParserError(String::from(""), 0, String::from(""))
// );

test_succeed!(
    literals,
    ".././test-scripts/string/literals.lox",
    "()",
    "a string",
    "A~¶Þॐஃ"
);

//TODO: handle multiline string
// test_succeed!(
//     multiline,
//     ".././test-scripts/string/multiline.lox",
//     "1\n2\n3"
// );

test_fail!(
    unterminated,
    ".././test-scripts/string/unterminated.lox",
    LoxError::ParserError(
        String::from("\"this string has no close quote"),
        2,
        String::from("")
    )
);
