use super::common::*;

pub fn blx<const IMM: bool>(adr: u32, instr: u32) -> String {
    format!(
        "blx {}",
        if IMM {
            let offset = (((instr as i32 & 0xFFFFFF) << 8) >> 8) * 4;
            let adr = adr.wrapping_add_signed(offset).wrapping_add(8);
            format!("#0x{adr:08X}")
        } else {
            reg(instr & 0xF)
        }
    )
}

pub fn bx(_: u32, instr: u32) -> String {
    let rm = reg(instr & 0xF);
    let cond = cond(instr);
    format!("bx{cond} {rm}")
}

pub fn b<const ARG: arm_decode::B>(adr: u32, instr: u32) -> String {
    let offset = (((instr as i32 & 0xFFFFFF) << 8) >> 8) * 4;
    let adr = adr.wrapping_add_signed(offset).wrapping_add(8);
    let cond = cond(instr);
    format!("b{cond}{} #0x{adr:08X}", if ARG.link { "l" } else { "" })
}