/// Using Connect4 module in the main only!
use crate::connect4::Connect4;

/// List of top level modules!
mod connect4;
mod enums;
mod minimax;
mod models;

/// Main!
fn main() {
    // Run game!
    Connect4::run();
}
