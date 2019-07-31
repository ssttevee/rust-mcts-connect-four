#[macro_use]
extern crate itertools;

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate wasm_bindgen;

#[cfg(not(target_arch = "wasm32"))]
extern crate termion;

pub mod game;
pub mod mcts;
pub mod hbot;
pub mod board;
pub mod common;

#[cfg(not(target_arch = "wasm32"))]
mod cli;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    cli::start()
}
