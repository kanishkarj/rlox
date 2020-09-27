mod runner;
mod scanner;
mod interpreter;
mod grammar;
mod ast_printer;
mod parser;
mod environment;

use std::env::args;
use runner::Runner;

fn main() {
    let cli_args: Vec<String> = args().collect();
    let ln = cli_args.len();
    let mut runner = Runner::new();
    if ln == 2 {
        runner.run_file(&cli_args[1]);
    } else if ln < 2 {
        runner.run_prompt();
    } else {
        println!("rlox [script]");
    }
    // ast_printer::test();
}
