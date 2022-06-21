use crate::engines::interpreter::Interpreter;
use cpu::{
    cycles::Cycles,
    instrutions::decode::CC,
    registers::{Flag, R16, R8},
};

impl Interpreter {
    #[inline(always)]
    pub fn r8_get(&self, r: R8) -> u8 {
        self.cpu.regs().read_r8(r)
    }

    #[inline(always)]
    pub fn r8_set(&mut self, r: R8, v: u8) {
        self.cpu.regs_mut().write_r8(r, v);
    }

    #[inline(always)]
    pub fn r16_get(&self, r: R16) -> u16 {
        self.cpu.regs().read_r16(r)
    }

    #[inline(always)]
    pub fn r16_set(&mut self, r: R16, v: u16) {
        self.cpu.regs_mut().write_r16(r, v);
    }

    #[inline(always)]
    pub fn pc_get(&self) -> u16 {
        self.cpu.regs().read_pc()
    }

    #[inline(always)]
    pub fn pc_set(&mut self, val: u16) {
        self.cpu.regs_mut().write_pc(val);
    }

    #[inline(always)]
    pub fn sp_get(&self) -> u16 {
        self.cpu.regs().read_sp()
    }

    #[inline(always)]
    pub fn sp_set(&mut self, val: u16) {
        self.cpu.regs_mut().write_sp(val);
    }

    #[inline(always)]
    pub fn flag_get(&self, f: Flag) -> bool {
        self.cpu.regs().get_flag(f)
    }

    #[inline(always)]
    pub fn flag_set(&mut self, f: Flag, v: bool) {
        self.cpu.regs_mut().set_flag(f, v);
    }

    #[inline(always)]
    pub fn check_cond(&self, cc: CC) -> bool {
        match cc {
            CC::NZ => !self.flag_get(Flag::N),
            CC::Z => self.flag_get(Flag::Z),
            CC::NC => !self.flag_get(Flag::C),
            CC::C => self.flag_get(Flag::C),
        }
    }

    #[inline(always)]
    pub fn mem_read(&mut self, adr: u16) -> u8 {
        todo!()
    }

    #[inline(always)]
    pub fn mem_write(&mut self, adr: u16, val: u8) {
        todo!()
    }

    #[inline(always)]
    pub fn fetch(&mut self) -> u8 {
        let pc = self.pc_get();
        self.pc_set(pc + 1);
        self.mem_read(pc)
    }

    #[inline(always)]
    pub fn fetch16(&mut self) -> u16 {
        let lo = self.fetch() as u16;
        let hi = self.fetch() as u16;
        (hi << 8) | lo
    }

    #[inline(always)]
    pub fn push(&mut self, val: u16) {
        let sp = self.sp_get();
        self.sp_set(sp.wrapping_sub(2));
        self.mem_write(sp.wrapping_sub(1), (val >> 8) as u8);
        self.mem_write(sp.wrapping_sub(2), val as u8);
    }

    #[inline(always)]
    pub fn pop(&mut self) -> u16 {
        let mut val = 0 as u16;
        let sp = self.sp_get();
        self.sp_set(sp.wrapping_add(2));
        val |= self.mem_read(sp) as u16;
        val |= self.mem_read(sp.wrapping_add(1)) as u16;
        val
    }

    #[inline(always)]
    pub fn phl_get(&mut self) -> u8 {
        self.mem_read(self.r16_get(R16::HL))
    }

    #[inline(always)]
    pub fn phl_set(&mut self, val: u8) {
        self.mem_write(self.r16_get(R16::HL), val)
    }

    #[inline(always)]
    pub fn tick(&mut self, cycles: Cycles) {
        todo!()
    }
}
