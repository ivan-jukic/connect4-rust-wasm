// TODO remove this when integration is completed
#![allow(dead_code)]
#![allow(unused_imports)]

/// List of top level modules!
mod connect4;
mod enums;
mod minimax;
mod models;

/// Using Connect4 module in the main only!
use crate::connect4::Connect4;
use crate::enums::player::Player;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

/// WASM
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn connect4_ai_move(c: u8, w: bool);
    pub fn connect4_player_win(w: bool);
    pub fn connect4_err(err: &str);
}

#[derive(Serialize, Deserialize)]
struct Input {
    difficulty: String,
    player: String,
    board: Vec<Vec<String>>,
}

#[wasm_bindgen]
pub fn process_data_for_ai(data: &str) {
    match serde_json::from_str::<Input>(data) {
        Ok(inpt) => {
            let mut game: Connect4 =
                Connect4::init_from_input_for(Player::AI, Player::One, inpt.difficulty, inpt.board);
            let (col, is_winner) = game.get_ai_move();

            // Report data...
            connect4_ai_move(col, is_winner);
        }
        Err(err) => connect4_err(err.to_string().as_str()),
    };
}

#[wasm_bindgen]
pub fn process_data_for_player(data: &str) {
    match serde_json::from_str::<Input>(data) {
        Ok(inpt) => {
            let game: Connect4 =
                Connect4::init_from_input_for(Player::One, Player::AI, inpt.difficulty, inpt.board);

            // Check if player won
            connect4_player_win(game.is_current_player_winner());
        }
        Err(err) => connect4_err(err.to_string().as_str()),
    };
}
