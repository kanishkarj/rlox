#![allow(unused_imports)]

use rlox_core::frontend::lexer::*;
use rlox_core::frontend::parser::Parser;
use rlox_core::runtime::interpreter::Interpreter;
use rlox_vm::resolver::Resolver;
use std::fs::read_to_string;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
// // use logos::{source::Source, Logos};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
// // use rlox_core::runtime::definitions::object::Object;
use rlox_core::error::LoxError;
use rlox_vm::{chunk::VM, compiler::run_file};

fn run_script(path: &str, interpreter: &mut Interpreter) -> Result<(), LoxError> {
    let path = Path::new(path);
    let script = read_to_string(path).unwrap();
    let mut ast = Parser::new(Lexer::new().parse(&script)?).parse()?;
    Resolver::new().resolve(&mut ast)?;
    interpreter.interpret(&mut ast)?;
    Ok(())
}

macro_rules! test_line {
    ($res_vec:ident, $expected_val:expr, $($expected_vals:expr),+) => {
        // we are not doing tail recursion here as pop will return the values in reverse order, and on reversing the order in which we are checking we can ensure the input args can be in correct order.
        test_line!($res_vec, $($expected_vals),+);
        test_line!($res_vec, $expected_val);
    };
    ($res_vec:ident, $expected_val:expr) => {

let val = $res_vec.pop();
        if let Some(f) = (&$expected_val as &dyn Any).downcast_ref::<&str>() {
            assert_eq!(val, Some(Object::Str(f.to_string())));
        } else if let Some(f) = (&$expected_val as &dyn Any).downcast_ref::<f64>() {
            assert_eq!(val, Some(Object::Num(*f)));
        } else if let Some(f) = (&$expected_val as &dyn Any).downcast_ref::<i32>() {
            assert_eq!(val, Some(Object::Num(*f as f64)));
        } else if let Some(f) = (&$expected_val as &dyn Any).downcast_ref::<bool>() {
            assert_eq!(val, Some(Object::Bool(*f)));
        } else if let Some(_f) = (&$expected_val as &dyn Any).downcast_ref::<Object>() {
            if let Some(Object::Nil) = val {

            }
            else {
                panic!("[from script(class): {:?}][tested for: {:?}]",val.unwrap(),$expected_val);
            }
        } else if let Some(f) = (&$expected_val as &dyn Any).downcast_ref::<Class>() {
            if let Some(Object::Str(cl)) = val {
                assert_eq!(cl, f.name);
            }
            else {
                panic!("[from script(class): {:?}][tested for: {:?}]",val.unwrap(),$expected_val);
            }
        // } else if let Some(f) = (&$expected_val as &dyn Any).downcast_ref::<LoxInstance>() {
        //     if let Some(Object::Instance(cl)) = val {
        //         assert_eq!(cl.klass.name, f.klass.name);
        //     } else {
        //         panic!("[from script(instance): {}][tested for: {:?}]",val.unwrap(),$expected_val);
        //     }
        } else {
            // NOTE: this can happen if the type of param is not correctly passed
            panic!("[from script: {:?}][tested for: {:?}]",val,$expected_val);
        }
        // assert_eq!($res_vec.pop(), Some($expected_val));
    };
}

macro_rules! test_succeed {
    ($test_name:ident, $file_path: literal, $($expected_vals:expr),+) => {
        #[test]
        fn $test_name() {
            use rlox_vm::system_calls::SystemInterfaceMock;
            use rlox_vm::chunk::Object;
            use rlox_vm::class::Class;
            use rlox_vm::instance::Instance;
            let print_cache = Rc::new(RefCell::new(vec![]));
            // let mut interpreter = Interpreter::new(Rc::new(RefCell::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)})));
            run_file($file_path, SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}).unwrap();
            let mut print_cache = print_cache.borrow_mut();
            test_line!(print_cache, $($expected_vals),+);
            assert_eq!(print_cache.pop(), None);
        }
    };
    ($test_name:ident, $file_path: literal) => {
        #[test]
        fn $test_name() {
            use rlox_vm::system_calls::SystemInterfaceMock;
            let print_cache = Rc::new(RefCell::new(vec![]));
            // let mut interpreter = Interpreter::new(Rc::new(RefCell::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)})));
            run_file($file_path, SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}).unwrap();
            let mut print_cache = print_cache.borrow_mut();
            assert_eq!(print_cache.pop(), None);
        }
    };
}

macro_rules! test_fail {
    ($test_name:ident, $file_path: literal, $err_val: expr) => {
        #[test]
        fn $test_name() {
            use rlox_vm::system_calls::SystemInterfaceMock;
            let print_cache = Rc::new(RefCell::new(vec![]));
            // let mut interpreter = Interpreter::new(Rc::new(RefCell::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)})));
            if let Err(err) = run_file(
                $file_path,
                SystemInterfaceMock {
                    print_cache: Rc::clone(&print_cache),
                },
            ) {
                use LoxError::*;
                match (err.clone(), $err_val) {
                    (ScannerError(_, _, _), ScannerError(_, _, _)) => {}
                    (ParserError(_, _, _), ParserError(_, _, _)) => {}
                    (RuntimeError(_, _, _), RuntimeError(_, _, _)) => {}
                    (SemanticError(_, _, _), SemanticError(_, _, _)) => {}
                    (Break(line1), Break(line2)) if line1 == line2 => {}
                    (Continue(line1), Continue(line2)) if line1 == line2 => {}
                    (ReturnVal(_, line1), ReturnVal(_, line2)) if line1 == line2 => {}
                    _ => {
                        panic!("unhandled error {:?}", err)
                    }
                }
                return;
            }
            panic!("test did not err");
        }
    };
}

// SampleTest generated
// #[test]
// fn class() {
//     let print_cache = Rc::new(RefCell::new(vec![]));
//     let mut interpreter = Interpreter::new(Rc::new(RefCell::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)})));
//     run_script("../tests-scripts/class.lx", &mut interpreter);
//     let mut print_cache = print_cache.borrow_mut();
//     assert_eq!(print_cache.pop(), Some(Object::Str("The German chocolate cake is delicious!".to_string())));
//     assert_eq!(print_cache.pop(), None);
// }

mod assignment;
mod block;
mod bool;
mod break_stmt;
mod call;
mod class;
mod closure;
mod comments;
mod constructor;
mod continue_stmt;
mod field;
mod for_stmt;
mod function;
mod if_stmt;
mod inheritance;
mod lambda;
mod logical_operator;
mod method;
mod miscellaneous;
mod nil;
mod number;
mod operator;
mod print;
mod regression;
mod return_stmt;
mod string;
mod super_stmt;
mod this;
mod variable;
mod while_stmt;
