
use rlox_core::frontend::parser::Parser;
use rlox_core::frontend::lexer::*;
use std::fs::read_to_string;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use rlox_core::runtime::interpreter::{Interpreter};
use rlox_core::frontend::resolver::Resolver;
// use rlox_core::runtime::system_calls::SystemInterfaceMock;
// use logos::{source::Source, Logos};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
// use rlox_core::runtime::definitions::object::Object;
use rlox_core::error::LoxError;
use super::*;

                test_succeed!(
                    arity,
                    ".././test-scripts/method/arity.lox",
                    "no args",1,3,6,10,15,21,28,36
                );

                
                test_succeed!(
                    empty_block,
                    ".././test-scripts/method/empty_block.lox",
                    Object::Nil
                );

                
            test_fail!(
                extra_arguments,
                ".././test-scripts/method/extra_arguments.lox",
                LoxError::RuntimeError(String::from("method"), 8, String::from(""))
            );

            
            test_fail!(
                missing_arguments,
                ".././test-scripts/method/missing_arguments.lox",
                LoxError::RuntimeError(String::from("method"), 5, String::from(""))
            );

            
            test_fail!(
                not_found,
                ".././test-scripts/method/not_found.lox",
                LoxError::RuntimeError(String::from("unknown"), 3, String::from(""))
            );
                
            test_fail!(
                refer_to_name,
                ".././test-scripts/method/refer_to_name.lox",
                LoxError::SemanticError(String::from("method"), 3, String::from(""))
            );

            