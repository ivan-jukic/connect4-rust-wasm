#![allow(dead_code)]
#![allow(unused_imports)]

/// Using Connect4 module in the main only!
use crate::connect4::Connect4;
use crate::models::masks::get_win_masks;

/// List of top level modules!
mod connect4;
mod enums;
mod minimax;
mod models;

/// Main!
fn main() {
    println!("You're playing Connect4!");

    // Run game!
    Connect4::run();
}
