// mod runner;
// mod error;
// mod tests;
// mod frontend;
// mod runtime;
use rlox_core::runtime::runner::Runner;
// use runner::Runner;
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
