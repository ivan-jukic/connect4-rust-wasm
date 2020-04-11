// TODO remove this when integration is completed
#![allow(dead_code)]
#![allow(unused_imports)]

/// Using Connect4 module in the main only!
use crate::connect4::Connect4;

/// WASM
use wasm_bindgen::prelude::*;

/// List of top level modules!
mod connect4;
mod enums;
mod minimax;
mod models;

/*
/// Main!
fn main() {
    // Run game!
    Connect4::run();
}
*/

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, {{project-name}}!");
}
