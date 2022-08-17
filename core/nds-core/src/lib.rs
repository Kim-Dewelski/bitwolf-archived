pub mod engine;

mod arm9;
#[cfg(test)]
mod test;

use arm9::ARM9;
use engine::Engine;

pub struct Core<E: Engine> {
    arm9: ARM9<E>,
}
