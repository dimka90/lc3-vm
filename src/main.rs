mod memory;
mod registers;
mod opcode;
mod constants;
mod util;
mod errors;
mod vm;
use crate::memory::memory::{Memory, Memorytrait};
use crate::registers::register::{Register, Registers};
use crate::constants::constants::PC_START;
use crate::util::utils::{sign_extend};
fn main() {
    let mut memory = Memory::new();
    let mut register = Register::new();
    register.write(Registers::PC, PC_START);

    memory.store(PC_START, 50);
    memory.mem_read(Registers::PC, &mut register);
    let instr = memory.load(PC_START).unwrap();
    println!("Memory {instr}" );
    println!("Registers: {register:?}");
    let imm5: u16 = 0b11110; // -2 in 5-bit two's complement
    let signed = sign_extend(imm5, 5);
    println!("{:#X}", signed);
    println!("{:#b}", signed);
}