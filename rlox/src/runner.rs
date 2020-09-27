use std::path::Path;
use std::fs::read_to_string;
use std::io::{stdin, Read};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::scanner::Lexer;
use crate::parser::Parser;
// use crate::ast_printer::ASTprinter;
use crate::interpreter::Interpreter;

static had_error: AtomicBool = AtomicBool::new(false);

pub struct Runner {
    lexer: Lexer,
    interpreter: Interpreter
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            lexer: Lexer::new(),
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, path: &String) {
        let path = Path::new(path);
        let script = read_to_string(path).unwrap();
        self.run(&script);
    }
    
    pub fn run_prompt(&mut self) {
        let mut buff = String::new();
        let mut inp = stdin();
    
        while true {
            buff.clear();
            inp.read_to_string(&mut buff).unwrap();
            self.run(&buff);
            if had_error.compare_and_swap(true, true, Ordering::Release) {
                std::process::exit(65)
            }
        }
    }
    
    fn run(&mut self, script: &String) {
        match self.lexer.parse(script) {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(mut ast) => {
                        // match ast.accept(&mut ASTprinter{}) {
                        //     Ok(mut val) => {
                        //         println!("{}", val);
                        //     },
                        //     Err(err) => {println!("error: {:?}", err)},
                        // }
                        self.interpreter.interpret(&mut ast);
                    },
                    Err(err) => {println!("error: {:?}", err)},
                }
            },
            Err(err) => {println!("error: {:?}", err)},
        }
    }
    
    pub fn error(line: u32, location: &String, message: &String) {
        Self::report(line, location, message)
    }
    
    fn report(line: u32, location: &String, message: &String) {
        println!("[line {}] Error {}: {}", line, location, message);
        had_error.store(true, Ordering::Release);
    }
}
