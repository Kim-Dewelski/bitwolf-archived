// Temporary!
#![allow(dead_code)]
//

#[allow(unused_imports)]
#[macro_use]
extern crate util;

mod cli;
mod common;
mod config;
mod debug_views;
mod emu;
mod gui;
mod log;

fn main() {
    env_logger::init();
    let logger = log::new();
    let cli_args = cli::parse();
    gui::run(logger, cli_args);
}
