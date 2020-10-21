use std::path::Path;
use std::fs::read_to_string;
use std::io::{stdin, Read, stdout, Write, self};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::scanner::{Lexer, TokenType};
use crate::parser::Parser;
// use crate::ast_printer::ASTprinter;
use crate::interpreter::{Interpreter, Object};
use crate::resolver::Resolver;
static had_error: AtomicBool = AtomicBool::new(false);
use logos::{Logos,source::Source};
use crate::system_calls::SystemInterfaceMock;
use std::rc::Rc;
use std::cell::RefCell;


fn run_script(path: &str, interpreter: &mut Interpreter) {
    let path = Path::new(path);
    let script = read_to_string(path).unwrap();
    let mut lexer = Lexer::new();
    match lexer.parse(&script) {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(mut ast) => {
                    let mut resolv = Resolver::new();
                    if let Err(err) = resolv.resolve(&mut ast) {
                        err.print_error("");
                        return;
                    }
                    if let Err(err) = interpreter.interpret(&mut ast) {
                        err.print_error("");
                    }
                },
                Err(err) => {println!("error: {:?}", err)},
            }
        },
        Err(err) => {println!("error: {:?}", err)},
    }
}

#[test]
fn class() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/class.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Str("The German chocolate cake is delicious!".to_string())));
    assert_eq!(print_cache.pop(), None);
}

#[test]
fn closures_scopes() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/closures-scopes.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Str("global".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("block".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("global".to_string())));
    assert_eq!(print_cache.pop(), None);
}


#[test]
fn closures1() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/closures1.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Num(2 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(1 as f64)));
    assert_eq!(print_cache.pop(), None);
}

#[test]
fn closures2() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/closures2.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Num(5 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(4 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(3 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(2 as f64)));
    assert_eq!(print_cache.pop(), None);
}


#[test]
fn for_test() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/for.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Num(4 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(3 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(2 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(1 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(0 as f64)));
    assert_eq!(print_cache.pop(), None);
}


#[test]
fn if_else() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/if-else.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Str("l3".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("l2".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("l1".to_string())));
    assert_eq!(print_cache.pop(), None);
}


#[test]
fn lambdas() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/lambdas.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Num(24 as f64)));
    assert_eq!(print_cache.pop(), None);
}


#[test]
fn recursion1() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/recursion1.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Num(479001600 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(362880 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(720 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(6 as f64)));
    assert_eq!(print_cache.pop(), None);
}

#[test]
fn recursion2() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/recursion2.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Num(1597 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(610 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(233 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(55 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(13 as f64)));
    assert_eq!(print_cache.pop(), None);
}

#[test]
fn while_test() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/while.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Num(89 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(55 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(34 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(21 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(13 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(8 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(5 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(3 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(2 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(1 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(1 as f64)));
    assert_eq!(print_cache.pop(), Some(Object::Num(0 as f64)));
    assert_eq!(print_cache.pop(), None);
}

#[test]
fn inheritance() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/inheritance.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Str("Pipe full of custard and coat with chocolate.".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("Fry until golden brown.".to_string())));
    assert_eq!(print_cache.pop(), None);
}

#[test]
fn scopes_variables() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/scopes_variables.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Str("global c".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("global b".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("global a".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("global c".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("outer b".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("outer a".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("global c".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("outer b".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("inner a".to_string())));
    assert_eq!(print_cache.pop(), None);
}


#[test]
fn scopes_inheritance() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/scopes_inheritance.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Str("Pipe full of custard and coat with chocolate.".to_string())));
    assert_eq!(print_cache.pop(), Some(Object::Str("Fry until golden brown.".to_string())));
    assert_eq!(print_cache.pop(), None);
}

#[test]
fn scopes_functions() {
    let print_cache = Rc::new(RefCell::new(vec![]));
    let mut interpreter = Interpreter::new(Box::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)}));
    run_script("./tests/scopes_functions.lx", &mut interpreter);
    let mut print_cache = print_cache.borrow_mut();
    assert_eq!(print_cache.pop(), Some(Object::Str("The German chocolate cake is delicious!".to_string())));
    assert_eq!(print_cache.pop(), None);
}