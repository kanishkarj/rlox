
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
                class_in_else,
                ".././test-scripts/if/class_in_else.lox",
                LoxError::ParserError(String::from("class"), 2, String::from(""))
            );

            
            test_fail!(
                class_in_then,
                ".././test-scripts/if/class_in_then.lox",
                LoxError::ParserError(String::from("class"), 2, String::from(""))
            );

            
                test_succeed!(
                    dangling_else,
                    ".././test-scripts/if/dangling_else.lox",
                    "good"
                );

                
                test_succeed!(
                    else_st,
                    ".././test-scripts/if/else.lox",
                    "good","good","block"
                );

                
            test_fail!(
                fun_in_else,
                ".././test-scripts/if/fun_in_else.lox",
                LoxError::ParserError(String::from("foo"), 2, String::from(""))
            );

            
            test_fail!(
                fun_in_then,
                ".././test-scripts/if/fun_in_then.lox",
                LoxError::ParserError(String::from("foo"), 2, String::from(""))
            );

            
                test_succeed!(
                    if_st,
                    ".././test-scripts/if/if.lox",
                    "good","block",true
                );

                
                test_succeed!(
                    truth,
                    ".././test-scripts/if/truth.lox",
                    "false","nil",true,0,"empty"
                );

                
            test_fail!(
                var_in_else,
                ".././test-scripts/if/var_in_else.lox",
                LoxError::ParserError(String::from("var"), 2, String::from(""))
            );

            
            test_fail!(
                var_in_then,
                ".././test-scripts/if/var_in_then.lox",
                LoxError::ParserError(String::from("var"), 2, String::from(""))
            );

            