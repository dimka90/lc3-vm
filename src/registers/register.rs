use crate::registers::flags::{ConditionalFlag, ConditionalFlagtrait};
// use crate::vm::{self, Vm};
#[repr(u16)]
#[derive(Debug, Clone)]
pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    CC,
    RCOUNT,
}

#[derive(Debug, Clone)]
pub struct RegisterFile {
    pub locations: [u16; Register::RCOUNT as usize],
}
impl RegisterFile {
    pub fn new() -> Self {
        Self {
            locations: [0; Register::RCOUNT as usize],
        }
    }

    pub fn write(&mut self, r: Register, value: u16) {
        self.locations[r as usize] = value;
    }
    pub fn read(&self, r: Register) -> u16 {
        self.locations[r as usize]
    }
    pub fn update_flag(&mut self, reg_num: u16) {
        let reg_enum = Self::from_register(reg_num).unwrap();
        let value = self.read(reg_enum);
        let flag = ConditionalFlag::from_value(value);
     
        self.locations[Register::CC as usize] = flag as u16;
    }

    pub fn from_register(reg: u16) -> Option<Register> {
        println!("{reg}");
        match reg {
            0 => Some(Register::R0),
            1 => Some(Register::R1),
            2 => Some(Register::R1),
            3 => Some(Register::R1),
            _=> None
        }
    }
}
