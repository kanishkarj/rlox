
use std::collections::HashMap;
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
                    arguments,
                    ".././test-scripts/constructor/arguments.lox",
                    "init",1,2
                );

                
                test_succeed!(
                    call_init_early_return,
                    ".././test-scripts/constructor/call_init_early_return.lox",
                    "init","init",LoxInstance::new(LoxClass::new(String::from("Foo"), Rc::new(HashMap::new()), None))
                );

                
                test_succeed!(
                    call_init_explicitly,
                    ".././test-scripts/constructor/call_init_explicitly.lox",
                    "Foo.init(one)","Foo.init(two)",LoxInstance::new(LoxClass::new(String::from("Foo"), Rc::new(HashMap::new()), None)),"init"
                );

                
                test_succeed!(
                    default,
                    ".././test-scripts/constructor/default.lox",
                    LoxInstance::new(LoxClass::new(String::from("Foo"), Rc::new(HashMap::new()), None))
                );

                
            test_fail!(
                default_arguments,
                ".././test-scripts/constructor/default_arguments.lox",
                LoxError::RuntimeError(String::from("Foo"), 3, String::from(""))
            );

            
                test_succeed!(
                    early_return,
                    ".././test-scripts/constructor/early_return.lox",
                    "init",LoxInstance::new(LoxClass::new(String::from("Foo"), Rc::new(HashMap::new()), None))
                );

                
            test_fail!(
                extra_arguments,
                ".././test-scripts/constructor/extra_arguments.lox",
                LoxError::RuntimeError(String::from("Foo"), 8, String::from(""))
            );

            
                test_succeed!(
                    init_not_method,
                    ".././test-scripts/constructor/init_not_method.lox",
                    "not initializer"
                );

                
            test_fail!(
                missing_arguments,
                ".././test-scripts/constructor/missing_arguments.lox",
                LoxError::RuntimeError(String::from("Foo"), 5, String::from(""))
            );

            
                test_succeed!(
                    return_in_nested_function,
                    ".././test-scripts/constructor/return_in_nested_function.lox",
                    "bar",LoxInstance::new(LoxClass::new(String::from("Foo"), Rc::new(HashMap::new()), None))
                );

                
            test_fail!(
                return_value,
                ".././test-scripts/constructor/return_value.lox",
                LoxError::SemanticError(String::from("return"), 3, String::from(""))
            );

            