
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
                    and,
                    ".././test-scripts/logical_operator/and.lox",
                    false,1,false,true,3,true,false
                );

                
                test_succeed!(
                    and_truth,
                    ".././test-scripts/logical_operator/and_truth.lox",
                    false,Object::Nil,"ok","ok","ok"
                );

                
                test_succeed!(
                    or,
                    ".././test-scripts/logical_operator/or.lox",
                    1,1,true,false,false,false,true
                );

                
                test_succeed!(
                    or_truth,
                    ".././test-scripts/logical_operator/or_truth.lox",
                    "ok","ok",true,0,"s"
                );

                