
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
                class_in_body,
                ".././test-scripts/while/class_in_body.lox",
                LoxError::ParserError(String::from("class"), 2, String::from(""))
            );

            
                test_succeed!(
                    closure_in_body,
                    ".././test-scripts/while/closure_in_body.lox",
                    1,2,3
                );

                
            test_fail!(
                fun_in_body,
                ".././test-scripts/while/fun_in_body.lox",
                LoxError::ParserError(String::from("foo"), 2, String::from(""))
            );

            
                test_succeed!(
                    return_closure,
                    ".././test-scripts/while/return_closure.lox",
                    "i"
                );

                
                test_succeed!(
                    return_inside,
                    ".././test-scripts/while/return_inside.lox",
                    "i"
                );

                
                test_succeed!(
                    syntax,
                    ".././test-scripts/while/syntax.lox",
                    1,2,3,0,1,2
                );

                
            test_fail!(
                var_in_body,
                ".././test-scripts/while/var_in_body.lox",
                LoxError::ParserError(String::from("var"), 2, String::from(""))
            );

            