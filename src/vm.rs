use crate::memory::memory::{Memory, Memorytrait};
use crate::registers::register::Register;
pub struct Vm {
    pub memory: Memory,
    pub register: Register,
}
pub trait Vmtrait {
    fn new() -> Self;
}
impl Vmtrait for Vm {
    fn new() -> Self {
        Self {
            memory: Memory::new(),
            register: Register::new(),
        }
    }
}
