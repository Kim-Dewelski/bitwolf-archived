use crate::gui::window::Window;

use super::{DynamicDV, GlobalState, Ui};
use bitwolf_core::debug::cartridge_info::Header;
use imgui::Io;

#[derive(Default)]
pub struct DVCartridge;

#[derive(Debug, Default)]
pub struct State {
    pub cartridge_header: Header,
}

impl DynamicDV for DVCartridge {
    type Local = State;
    type Emu = ();

    #[inline]
    fn draw(
        &mut self,
        state: &mut State,
        _global_state: &GlobalState,
        _window: &mut Window,
        ui: &Ui,
        _io: &Io,
    ) {
        let cart = &state.cartridge_header;
        ui.text(format!("arm9 rom offset: 0x{:08X}", cart.arm9_rom_adr()));
        ui.text(format!("arm9 entry: 0x{:08X}", cart.arm9_entry()));
        ui.text(format!("arm9 load address: 0x{:08X}", cart.arm9_load_adr()));
        ui.text(format!("arm9 size: 0x{:08X}", cart.arm9_size()));
    }

    #[inline]
    fn has_menu_bar(&self) -> bool {
        false
    }

    #[inline]
    fn on_change(&mut self, _old: Self::Local, _new: &mut Self::Local) {}

    #[inline]
    fn emu_update(&self) -> Option<Self::Emu> {
        None
    }
}