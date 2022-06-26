use crate::{emu::event_slots::Slot, Emu};

impl Emu {
    pub fn handle_event(&mut self, slot: Slot) {
        match slot {
            Slot::TIMER => self.timer_event(),
            Slot::EI => self.ime_set(true),
        }
    }
}