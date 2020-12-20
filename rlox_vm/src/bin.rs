
use rlox_vm::chunk::{OpCode, VM};
// use debug::disassemble_chunk;
use rlox_vm::compiler::run_file;
use std::env::args;
use rlox_vm::system_calls::SystemInterface;
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
        println!("{:?}", run_file(&cli_args[1], SystemInterface{}));
    } else if ln < 2 {
    } else {
        println!("rlox [script]");
    }
    // disassemble_chunk(&mut vm, "test chunk");
}
