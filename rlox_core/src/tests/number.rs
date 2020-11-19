
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

            test_fail!(
                decimal_point_at_eof,
                ".././test-scripts/number/decimal_point_at_eof.lox",
                LoxError::ParserError(String::from(""), 3, String::from(""))
            );

            
            test_fail!(
                leading_dot,
                ".././test-scripts/number/leading_dot.lox",
                LoxError::ParserError(String::from("."), 2, String::from(""))
            );

            
                test_succeed!(
                    literals,
                    ".././test-scripts/number/literals.lox",
                    123,987654,0,0,123.456,-0.001
                );


                
            test_fail!(
                trailing_dot,
                ".././test-scripts/number/trailing_dot.lox",
                LoxError::ParserError(String::from(";"), 2, String::from(""))
            );

            