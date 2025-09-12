mod memory;
mod registers;
mod  instructions;
mod constants;
mod util;
mod errors;
mod vm;
use crate::memory::memory::{Memorytrait};
use crate::registers::register::{Registers};
use crate::constants::constants::PC_START;
use crate::instructions::opcode::{Opcode, OpcodeTrait};
use  crate::vm::{Vm, Vmtrait};
fn main() {
    test_add_instruction();
}
fn test_add_instruction() {
    println!("=== ADD Instruction Tests ===\n");
    
    // Test 1: Positive result
    println!("Test 1: ADD - Positive result");
    let mut lc3 = Vm::new();
    lc3.register.write(Registers::PC, PC_START);
    lc3.register.write(Registers::R2, 10);
    lc3.register.write(Registers::R3, 5);
    // ADD R1, R2, R3 (R1 = R2 + R3 = 10 + 5 = 15)
    lc3.memory.store(PC_START, 0b0001_001_010_0_00011);
    
    let instr = lc3.memory.mem_read(Registers::PC, &mut lc3.register).unwrap();
    let opcode = Opcode::from_instruction(instr >> 12).unwrap();
    println!("Opcode: {:?}", opcode);
    match opcode { 
        Opcode::ADD => Opcode::add(instr, &mut lc3), 
        _ => panic!("Expected ADD opcode") 
    }
    
    println!("R2: {}, R3: {}", lc3.register.locations[2], lc3.register.locations[3]);
    println!("R1 result: {}", lc3.register.locations[1]);
    println!("Condition Code: {}", lc3.register.locations[Registers::CC as usize]);
    assert!(lc3.register.locations[1] == 15, "Expected positive result 15");
    assert!(lc3.register.locations[Registers::CC as usize] > 0, "Expected positive flag");
    println!("✓ Positive result test passed\n");
    
    // Test 2: Zero result
    println!("Test 2: ADD - Zero result");
    let mut lc3 = Vm::new();
    lc3.register.write(Registers::PC, PC_START);
    lc3.register.write(Registers::R2, 5);
    lc3.register.write(Registers::R3, 65531); // -5 in 16-bit two's complement (0xFFFB)
    // ADD R1, R2, R3 (R1 = R2 + R3 = 5 + (-5) = 0)
    lc3.memory.store(PC_START, 0b0001_001_010_0_00011);
    
    let instr = lc3.memory.mem_read(Registers::PC, &mut lc3.register).unwrap();
    let opcode = Opcode::from_instruction(instr >> 12).unwrap();
    println!("Opcode: {:?}", opcode);
    match opcode { 
        Opcode::ADD => Opcode::add(instr, &mut lc3), 
        _ => panic!("Expected ADD opcode") 
    }
    
    println!("R2: {}, R3: {} (-5)", lc3.register.locations[2], lc3.register.locations[3]);
    println!("R1 result: {}", lc3.register.locations[1]);
    println!("Condition Code: {}", lc3.register.locations[Registers::CC as usize]);
    assert!(lc3.register.locations[1] == 0, "Expected zero result");
    assert!(lc3.register.locations[Registers::CC as usize] == 2, "Expected zero flag");
    println!("✓ Zero result test passed\n");
    
    // Test 3: Negative result
    println!("Test 3: ADD - Negative result");
    let mut lc3 = Vm::new();
    lc3.register.write(Registers::PC, PC_START);
    lc3.register.write(Registers::R2, 5);
    lc3.register.write(Registers::R3, 65526); // -10 in 16-bit two's complement (0xFFF6)
    // ADD R1, R2, R3 (R1 = R2 + R3 = 5 + (-10) = -5)
    lc3.memory.store(PC_START, 0b0001_001_010_0_00011);
    
    let instr = lc3.memory.mem_read(Registers::PC, &mut lc3.register).unwrap();
    let opcode = Opcode::from_instruction(instr >> 12).unwrap();
    println!("Opcode: {:?}", opcode);
    match opcode { 
        Opcode::ADD => Opcode::add(instr, &mut lc3), 
        _ => panic!("Expected ADD opcode") 
    }
    
    println!("R2: {}, R3: {} (-10)", lc3.register.locations[2], lc3.register.locations[3]);
    println!("R1 result: {} (-5)", lc3.register.locations[1]);
    println!("Condition Code: {}", lc3.register.locations[Registers::CC as usize]);
    // The result should be 65531 (which is -5 in 16-bit two's complement)
    assert!(lc3.register.locations[1] == 65531, "Expected negative result (-5 as 65531)");
    assert!(lc3.register.locations[Registers::CC as usize] == 4, "Expected negative flag (4)");
    println!("✓ Negative result test passed\n");
    
    println!("=== All ADD Instruction Tests Completed Successfully ===");
}