
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
                body_must_be_block,
                "../test-scripts/function/body_must_be_block.lox",
                LoxError::ParserError(String::from("123"), 3, String::from(""))
            );

            
                test_succeed!(
                    empty_body,
                    "../test-scripts/function/empty_body.lox",
                    Object::Nil
                );

                
            test_fail!(
                extra_arguments,
                "../test-scripts/function/extra_arguments.lox",
                LoxError::RuntimeError(String::from("f"), 6, String::from(""))
            );

            
            test_fail!(
                local_mutual_recursion,
                "../test-scripts/function/local_mutual_recursion.lox",
                LoxError::RuntimeError(String::from("isOdd"), 4, String::from(""))
            );

            
                test_succeed!(
                    local_recursion,
                    "../test-scripts/function/local_recursion.lox",
                    21
                );

                
            test_fail!(
                missing_arguments,
                "../test-scripts/function/missing_arguments.lox",
                LoxError::RuntimeError(String::from("f"), 3, String::from(""))
            );

            
            test_fail!(
                missing_comma_in_parameters,
                "../test-scripts/function/missing_comma_in_parameters.lox",
                LoxError::ParserError(String::from("c"), 3, String::from(""))
            );

            
                test_succeed!(
                    mutual_recursion,
                    "../test-scripts/function/mutual_recursion.lox",
                    true,true
                );

                
                test_succeed!(
                    parameters,
                    "../test-scripts/function/parameters.lox",
                    0,1,3,6,10,15,21,28,36
                );

                
                test_succeed!(
                    recursion,
                    "../test-scripts/function/recursion.lox",
                    21
                );

                