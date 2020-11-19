
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
                    line_at_eof,
                    ".././test-scripts/comments/line_at_eof.lox",
                    "ok"
                );

                test_succeed!(
                    only_line_comment_and_line,
                    ".././test-scripts/comments/only_line_comment_and_line.lox"
                );
                
                test_succeed!(
                    only_line_comment,
                    ".././test-scripts/comments/only_line_comment.lox"
                );
                
                test_succeed!(
                    unicode,
                    ".././test-scripts/comments/unicode.lox",
                    "ok"
                );

                