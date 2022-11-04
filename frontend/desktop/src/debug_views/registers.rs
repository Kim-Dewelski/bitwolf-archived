use super::GlobalStateData;

#[derive(Default, Debug, Clone)]
pub struct State {
    pub pc: u32,
}

#[derive(Default, Debug)]
pub struct Registers {
    state: State,
    pc_changed: bool,
}

impl Registers {
    #[inline]
    pub fn pc(&self) -> u32 {
        self.state.pc
    }
}

impl GlobalStateData for Registers {
    type State = State;

    #[inline]
    fn on_change(&mut self, old: Self::State) {
        let new = &self.state;
        self.pc_changed = new.pc != old.pc;
    }

    #[inline]
    fn set_state(&mut self, new: Self::State) {
        self.state = new;
    }

    #[inline]
    fn get_state(&self) -> Self::State {
        self.state.clone()
    }
}