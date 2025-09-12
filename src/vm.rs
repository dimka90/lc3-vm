use crate::memory::memory::{Memory, Memorytrait};
use crate::registers::register::RegisterFile;
pub struct Vm {
    pub memory: Memory,
    pub register: RegisterFile,
}
pub trait Vmtrait {
    fn new() -> Self;
}
impl Vmtrait for Vm {
    fn new() -> Self {
        Self {
            memory: Memory::new(),
            register: RegisterFile::new(),
        }
    }
}
