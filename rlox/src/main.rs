mod environment;
mod grammar;
mod interpreter;
mod parser;
mod resolver;
mod runner;
mod scanner;
mod system_calls;
mod literal;
mod token;
mod token_type;
mod error;
mod tests;


use runner::Runner;
use std::env::args;

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
