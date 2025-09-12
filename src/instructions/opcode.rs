
use crate::registers::register::{Register, Registers};
use crate::util::utils::sign_extend;
use crate::constants::constants::BITS_COUNT;
use crate::vm::{ Vm};
#[repr(u16)]
#[derive(Debug)]
pub enum Opcode {
    BR = 0, /* branch */
    ADD,    /* add  */
    LD,     /* load */
    ST,     /* store */
    JSR,    /* jump register */
    AND,    /* bitwise and */
    LDR,    /* load register */
    STR,    /* store register */
    RTI,    /* unused */
    NOT,    /* bitwise not */
    LDI,    /* load indirect */
    STI,    /* store indirect */
    JMP,    /* jump */
    RES,    /* reserved (unused) */
    LEA,    /* load effective address */
    TRAP,   /* execute trap */
}

pub trait OpcodeTrait {
fn from_instruction(opcode: u16) -> Option<Opcode>;
fn add(instr: u16, vm: &mut Vm);
}
impl OpcodeTrait for Opcode {
    fn from_instruction(opcode: u16) -> Option<Opcode> {
        match opcode {
            0 => return Some(Opcode::BR),
            1 => return Some(Opcode::ADD),
            _ => None,
        }
    }

    fn add(instr: u16, vm: &mut Vm){
         let dr = (instr >> 9) & 0x7;
         let sr1: u16 = (instr >> 6) & 0x7;
         let imm_flag =(instr >> 5) & 0x1;

         let src1_value = vm.register.locations[sr1 as usize];
         if imm_flag == 1{
            let imm5 = sign_extend(instr & 0x1F, BITS_COUNT);
            vm.register.locations[dr as usize] = src1_value.wrapping_add(imm5);         }
         else{
             let src2 = instr & 0x7;
             let src2_value = vm.register.locations[src2 as usize];
             vm.register.locations[dr as usize] = src1_value.wrapping_add(src2_value);
         }
         vm.register.update_flag(dr);

    }
}
