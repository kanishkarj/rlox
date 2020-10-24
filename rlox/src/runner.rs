use crate::parser::Parser;
use crate::scanner::*;
use std::fs::read_to_string;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::interpreter::Interpreter;
use crate::resolver::Resolver;
static had_error: AtomicBool = AtomicBool::new(false);
use crate::system_calls::{SystemCalls, SystemInterface};
use logos::{source::Source, Logos};
use std::rc::Rc;
use std::cell::RefCell;

/**
 * TODO: 
 * extend and compose errors such that return and break come in another enum.
 * env.getat is to be only used with env, and .get with globals, ensure this for the others too! can eb done using traits.
 * refactor/rename to follow rust naming
 * macros in tests
 * add tests that check for error cases too
 * */
pub struct Runner {
    sys_interface: Rc<RefCell<dyn SystemCalls>>,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            sys_interface: Rc::new(RefCell::new(SystemInterface {})),
        }
    }

    pub fn run_file(&mut self, path: &String) {
        let path = Path::new(path);
        let script = read_to_string(path).unwrap();
        if let Err(err) = self.run(&script) {
            self.sys_interface.borrow_mut().print_error(err);
        }
    }
    pub fn run_prompt(&mut self) {
        let mut buff = String::new();
        let inp = stdin();
        loop {
            print!("> ");
            io::stdout().flush();
            buff.clear();
            inp.read_line(&mut buff).unwrap();
            if let Err(err) = self.run(&buff) {
                self.sys_interface.borrow_mut().print_error(err);
            }
        }
    }
    fn run(&mut self, script: &String) -> Result<(), LoxError> {
        let mut ast = Parser::new(Lexer::new().parse(script)?).parse()?;
        Resolver::new().resolve(&mut ast)?;
        Interpreter::new(Rc::clone(&mut self.sys_interface)).interpret(&mut ast)?;
        Ok(())
    }
}
