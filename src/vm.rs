use crate::memory::memory::Memory;
use crate::registers::register::Register;
pub struct VM{
    memory: Memory,
    register: Register
}