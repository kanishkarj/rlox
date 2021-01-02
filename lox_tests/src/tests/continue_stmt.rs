
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


            test_fail!(
                class,
                ".././test-scripts/continue/class.lox",
                LoxError::ParserError(String::from("continue"), 2, String::from(""))
            );

            
            test_fail!(
                function,
                ".././test-scripts/continue/function.lox",
                LoxError::Continue(2)
            );

            
            test_fail!(
                global_scope,
                ".././test-scripts/continue/global_scope.lox",
                LoxError::Continue(1)
            );

            
            test_fail!(
                local_scope,
                ".././test-scripts/continue/local_scope.lox",
                LoxError::Continue(2)
            );

            
                test_succeed!(
                    loops,
                    ".././test-scripts/continue/loop.lox",
                    5,5
                );

                