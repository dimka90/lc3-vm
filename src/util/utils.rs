pub fn convert_to_usize(value: u16) -> usize{
    let u16_to_usize =value as usize;
    u16_to_usize
}
pub fn sign_extend(instr : u16,bit_count: usize) -> u16 {
    if instr >> ( bit_count - 1) & 1 == 1 {
        println!("Mask:::: {bit_count}");
        let result =instr | 0xFFFF << bit_count;
        return result;
    } else {
        return instr;
    }
}
