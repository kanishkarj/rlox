mod chunk;
mod commons;
mod debug;
mod compiler;
mod resolver;

use chunk::{OpCode, VM};
use debug::disassemble_chunk;
use crate::compiler::run_file;
use std::env::args;
fn main() {
//     let mut vm = VM::new();
    // let mut const_pool:Vec<Value>;
    
    // let const_pos = vm.push_const(1.3);
    // vm.push_inst(OpCode::Constant(2, const_pos));
    // vm.push_inst(OpCode::Return(1));
    // vm.run(true);
    let cli_args: Vec<String> = args().collect();
    let ln = cli_args.len();
    if ln == 2 {
        run_file(&cli_args[1]);
    } else if ln < 2 {
    } else {
        println!("rlox [script]");
    }
    // disassemble_chunk(&mut vm, "test chunk");
}
