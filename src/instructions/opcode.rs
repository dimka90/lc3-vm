use std::{io::ErrorKind};
use crate::util::utils::sign_extend;
use crate::constants::constants::BIT_COUNT;
#[repr(u16)]
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
// fn add(instr: u16) -> Result<u16, ErrorKind>;
}
impl OpcodeTrait for Opcode {
    fn from_instruction(opcode: u16) -> Option<Opcode> {
        match opcode {
            0 => return Some(Opcode::BR),
            1 => return Some(Opcode::ADD),
            _ => None,
        }
    }

    // fn add(instr: u16) -> Result<u16, ErrorKind> {
    //      let r0 = (instr >> 9) & 0x7;
    //      let r1: u16 = (instr >> 6) & 0x7;
    //      let imm_flag =(instr >> 5) & 0x1;

    //      if imm_flag == 1{
    //         let imm5 = sign_extend(instr & 0x1, BIT_COUNT);
    //      };

    // }
}
