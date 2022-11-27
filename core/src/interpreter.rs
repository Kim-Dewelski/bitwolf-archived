pub mod arm9;

use crate::{Core, Engine};

pub struct Interpreter;

impl Engine for Interpreter {
    type GlobalData = ();
    type Arm9Data = ();
}

#[inline]
pub fn run(core: &mut Core<Interpreter>, cycles: u64) {
    arm9::run(core, cycles);
}
