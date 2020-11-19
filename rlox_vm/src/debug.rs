use crate::VM;
use crate::OpCode;

pub fn disassemble_chunk(vm: &VM, chunk_name: &str) {
    println!("===== {} =====", chunk_name);
    let mut pc = 0;
    while pc < vm.len_inst() {
        pc = disassemble_inst(vm, pc);
    }
}

pub fn disassemble_inst(vm: &VM, offset: usize) -> usize {
    use OpCode::*;

    // println!("{} ", offset);
    let inst = vm.get_inst(offset).unwrap();
    // return match inst {
    //     Return(line) => simple_instruction("OP_RETURN",*line, offset),
    //     Constant(line, pos) => const_instruction("OP_CONST",*line, vm, *pos, offset),
    // }
    return 0;
}

pub fn simple_instruction(name: &str,line: u32, offset: usize) -> usize {
    println!("L{}: {} ",line , name);
    return offset + 1;
}

pub fn const_instruction(name: &str,line: u32, vm: &VM, pos: usize, offset: usize) -> usize {
    println!("L{}: {} {:?} ",line , name, vm.get_const(pos));
    return offset + 1;
}