mod memory;
mod registers;
mod  instructions;
mod constants;
mod util;
mod errors;
mod vm;
use crate::memory::memory::{Memorytrait};
use crate::registers::register::{Registers};
use crate::constants::constants::PC_START;
use crate::util::utils::{sign_extend};
use crate::instructions::opcode::{Opcode, OpcodeTrait};
use  crate::vm::{Vm, Vmtrait};
fn main() {
    let  mut lc3 = Vm::new();
    lc3.register.write(Registers::PC, PC_START);

    lc3.memory.store(PC_START, 0b0001_001_010_0_00101);
    let instr = lc3.memory.mem_read(Registers::PC, &mut lc3.register).unwrap();
    let opcode = (instr >> 12) & 0x7;

    let opcode = Opcode::from_instruction(opcode).unwrap();
    match opcode{
        Opcode::BR => println!("Branching"),
        Opcode::ADD => println!("Addition"),
        _=> println!("Unknown....")
    }

    let imm5: u16 = 0b11110; 
    let signed = sign_extend(imm5, 5);
    println!("{:#X}", signed);
    println!("{:#b}", signed);
    lc3.register.update_flag(0b1111_1111_111_11110);
}