use crate::registers::flags::{ConditionalFlag, ConditionalFlagtrait};
#[repr(u16)]
#[derive(Debug, Clone)]
pub enum Registers{
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
    RCOUNT

}

#[derive(Debug, Clone)]
pub struct Register{
    pub locations:[u16; Registers::RCOUNT as usize]
}
impl  Register {
    pub fn new() -> Self{
        Self { locations: [0; Registers::RCOUNT as usize] }
    }

    pub fn write(&mut self, r: Registers, value: u16) {
        self.locations[r as usize] = value;
    }
    pub fn read(self, r: Registers) -> u16{
        self.locations[r as usize]
    }
    pub fn update_flag(&mut self, value: u16){
        let flag = ConditionalFlag::from_value(value);
        println!("Flag to update");
        self.locations[Registers::CC as usize] = flag as u16;
        println!("Value at Reg 9: {}", self.locations[Registers::CC as usize]  )
    }
}