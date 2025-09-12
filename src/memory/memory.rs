use crate::constants::constants::{MEMORY_MAX, PC_INCREMENT};
use crate::errors::errors::AddressError;
use crate::registers::register::{RegisterFile, Register};
use crate::util::utils::convert_to_usize;
#[derive(Debug)]
pub struct Memory {
    pub locations: [u16; MEMORY_MAX],
}

pub trait Memorytrait {
    fn new() -> Self;
    fn store(&mut self, mem_addr: u16, instr: u16) -> Option<u16>;
    fn load(&self, mem_addr: u16) -> Result<u16, AddressError>;
    fn mem_read(&self, pc_addr: Register, register: &mut RegisterFile) -> Option<u16>;
}
impl Memorytrait for Memory {
    fn new() -> Self {
        Self {
            locations: [0; MEMORY_MAX],
        }
    }
    fn store(&mut self, mem_addr: u16, instr: u16) -> Option<u16> {
        self.locations[convert_to_usize(mem_addr)] = instr;
        Some(instr)
    }
    fn load(&self, mem_addr: u16) -> Result<u16, AddressError> {
        if convert_to_usize(mem_addr) >= MEMORY_MAX {
            return Err(AddressError::OutofBounds);
        }
        let instr = self.locations[mem_addr as usize];
        Ok(instr)
    }

    fn mem_read(&self, pc_addr: Register, register: &mut RegisterFile) -> Option<u16> {
        let pc_value = register.locations[convert_to_usize(pc_addr.clone() as u16)];
        let instr = self.load(pc_value).unwrap();
        let pc_addr_to_usize = convert_to_usize(pc_addr as u16);
        //Update PC value by 1
        register.locations[pc_addr_to_usize] = pc_value + PC_INCREMENT;
        Some(instr)
    }
}
