use crate::registers::flags::{ConditionalFlag, ConditionalFlagtrait};
use crate::vm::{self, Vm};
#[repr(u16)]
#[derive(Debug, Clone)]
pub enum Registers {
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
pub struct Register {
    pub locations: [u16; Registers::RCOUNT as usize],
}
impl Register {
    pub fn new() -> Self {
        Self {
            locations: [0; Registers::RCOUNT as usize],
        }
    }

    pub fn write(&mut self, r: Registers, value: u16) {
        self.locations[r as usize] = value;
    }
    pub fn read(&self, r: Registers) -> u16 {
        self.locations[r as usize]
    }
    pub fn update_flag(&mut self, reg_num: u16) {
        let reg_enum = Self::from_register(reg_num).unwrap();
        let value = self.read(reg_enum);
        let flag = ConditionalFlag::from_value(value);
        self.locations[Registers::CC as usize] = flag as u16;
    }

    pub fn from_register(reg: u16) -> Option<Registers> {
        println!("{reg}");
        match reg {
            0 => Some(Registers::R0),
            1 => Some(Registers::R1),
            2 => Some(Registers::R1),
            3 => Some(Registers::R1),
            _=> None
        }
    }
}
