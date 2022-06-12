mod nintendo;

use super::DrawContext;
use crate::backends::{Backend, BackendType};

pub fn draw(draw_ctx: &mut DrawContext, backend: &mut Backend) {
    match backend.get() {
        BackendType::None => {}
        backend => draw_ctx.ui().menu("Debug", || match backend {
            BackendType::NintendoGBC(gbc) => nintendo::gbc::menu(draw_ctx, gbc),
            BackendType::None => {
                logger::fatal!("Attempted to spawn 'Debug' drop down menu from 'None' backend",)
            }
        }),
    }
}
