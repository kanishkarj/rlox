use std::path::Path;
use std::fs::read_to_string;
use std::io::{stdin, Read, stdout, Write, self};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::scanner::*;
use crate::parser::Parser;
// use crate::ast_printer::ASTprinter;
use crate::interpreter::{Interpreter, Object};
use crate::resolver::Resolver;
static had_error: AtomicBool = AtomicBool::new(false);
use logos::{Logos,source::Source};
use crate::system_calls::SystemInterfaceMock;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;


fn run_script(path: &str, interpreter: &mut Interpreter) -> Result<(), LoxError>  {
    let path = Path::new(path);
    let script = read_to_string(path).unwrap();
    let mut ast = Parser::new(Lexer::new().parse(&script)?).parse()?;
        Resolver::new().resolve(&mut ast)?;
        interpreter.interpret(&mut ast)?;
        Ok(())
}


macro_rules! test_line {
    ($res_vec:ident, $expected_val:expr, $($expected_vals:expr),+) => {
        test_line!($res_vec, $expected_val);
        test_line!($res_vec, $($expected_vals),+)
    };
    ($res_vec:ident, $expected_val:expr) => {
        if let Some(f) = (&$expected_val as &Any).downcast_ref::<&str>() {
            assert_eq!($res_vec.pop(), Some(Object::Str(f.to_string())));
        } else if let Some(f) = (&$expected_val as &Any).downcast_ref::<f64>() {
            assert_eq!($res_vec.pop(), Some(Object::Num(*f)));
        } else if let Some(f) = (&$expected_val as &Any).downcast_ref::<i32>() {
            assert_eq!($res_vec.pop(), Some(Object::Num(*f as f64)));
        } else {
            // NOTE: this can happen if the type of param is not correctly passed
            panic!();
        }
        // assert_eq!($res_vec.pop(), Some($expected_val));
    };
}

macro_rules! create_test {
    ($test_name:ident, $file_path: literal, $($expected_vals:expr),+) => {
        #[test]
        fn $test_name() {
            let print_cache = Rc::new(RefCell::new(vec![]));
            let mut interpreter = Interpreter::new(Rc::new(RefCell::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)})));
            run_script($file_path, &mut interpreter);
            let mut print_cache = print_cache.borrow_mut();
            test_line!(print_cache, $($expected_vals),+);
            assert_eq!(print_cache.pop(), None);
        }
    };
}


// SampleTest generated
// #[test]
// fn class() {
//     let print_cache = Rc::new(RefCell::new(vec![]));
//     let mut interpreter = Interpreter::new(Rc::new(RefCell::new(SystemInterfaceMock{print_cache: Rc::clone(&print_cache)})));
//     run_script("./tests/class.lx", &mut interpreter);
//     let mut print_cache = print_cache.borrow_mut();
//     assert_eq!(print_cache.pop(), Some(Object::Str("The German chocolate cake is delicious!".to_string())));
//     assert_eq!(print_cache.pop(), None);
// }

create_test!(class, "./tests/class.lx", "The German chocolate cake is delicious!");
create_test!(closures_scopes, "./tests/closures-scopes.lx", "global","block","global");
create_test!(closures1,"./tests/closures1.lx",2,1);
create_test!(closures2,"./tests/closures2.lx",5,4,3,2);
create_test!(for_test,"./tests/for.lx",4,3,2,1,0);
create_test!(if_else,"./tests/if-else.lx","l3","l2","l1");
create_test!(lambdas,"./tests/lambdas.lx",24);
create_test!(recursion1,"./tests/recursion1.lx",479001600,362880,720,6);
create_test!(recursion2,"./tests/recursion2.lx",1597,610,233,55,13);
create_test!(while_test,"./tests/while.lx",89,55,34,21,13,8,5,3,2,1,1,0);
create_test!(inheritance,"./tests/inheritance.lx","Pipe full of custard and coat with chocolate.","Fry until golden brown.");
create_test!(scopes_variables,"./tests/scopes_variables.lx","global c","global b","global a","global c","outer b","outer a","global c","outer b","inner a");
create_test!(scopes_inheritance,"./tests/scopes_inheritance.lx","Pipe full of custard and coat with chocolate.","Fry until golden brown.");
create_test!(scopes_functions,"./tests/scopes_functions.lx","The German chocolate cake is delicious!");