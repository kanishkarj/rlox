
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
use std::collections::HashMap;
                test_succeed!(
                    test_394,
                    ".././test-scripts/regression/394.lox",
                    LoxClass::new(String::from("B"), Rc::new(HashMap::new()), None)
                );

                
                test_succeed!(
                    test_40,
                    ".././test-scripts/regression/40.lox",
                    false
                );

                