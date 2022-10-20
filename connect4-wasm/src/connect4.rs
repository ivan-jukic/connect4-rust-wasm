use std::io;

// Use enums
use crate::enums::board_column::*;
use crate::enums::difficulty::Difficulty;
use crate::enums::player::{player_to_str, Player};

// Use models
use crate::models::board::{is_successful_move, Board, MoveSuccess};
use crate::models::masks::get_cached_win_masks;

use crate::minimax;

/// Enum to describe possible move errors!
/// TODO reuse exising type in the board model
enum PlayerMoveError {
    NoError,
    UnknownColumn,
    ColumnFull,
}

// Game struct
#[derive(Debug, Clone)]
pub struct Connect4 {
    current: Player,
    next: Player,
    board: Board,
    pub difficulty: Difficulty,
}

impl Connect4 {
    /// Checks if the current player won the game!
    pub fn is_current_player_winner(&self) -> bool {
        let player_status = self.board.get_status_num_for_player(&self.current);

        get_cached_win_masks()
            .iter()
            .fold(false, |acc, &m| acc || player_status & m == m)
    }

    /// Switches players
    pub fn switch_players(&mut self) {
        let new_next = self.current.clone();
        self.current = self.next.clone();
        self.next = new_next;
    }

    /// Drop a token in column
    pub fn drop_token_in_col(&mut self, col_num: u8) -> MoveSuccess {
        if let Option::Some(col) = num_to_col_num(col_num) {
            self.board.drop_token(&self.current, col)
        } else {
            MoveSuccess::UnknownColumn
        }
    }

    /// ...
    pub fn is_column_full(&self, col_num: u8) -> bool {
        if let Option::Some(col) = num_to_col_num(col_num) {
            !self.board.can_drop_token_in_col(col)
        } else {
            false // does this make sense?
        }
    }

    pub fn get_ai_move(&mut self) -> (u8, bool) {
        let col: u8 = minimax::play_for_ai(&self);
        self.drop_token_in_col(col);
        (col, self.is_current_player_winner())
    }

    pub fn get_board(self) -> Board {
        self.board
    }

    // Initialising based on the input from the "outside"
    pub fn init_from_input_for(
        current: Player,
        next: Player,
        difficulty: String,
        str_board: Vec<Vec<String>>,
    ) -> Connect4 {
        let board = Board::init_from(str_board);
        Connect4 {
            current,
            next,
            board,
            difficulty: match difficulty.as_str() {
                "easy" => Difficulty::Easy,
                "medium" => Difficulty::Normal,
                "hard" => Difficulty::Hard,
                _ => Difficulty::Test,
            },
        }
    }

    /// For testing...
    pub fn init_vs_ai(difficulty: Difficulty) -> Connect4 {
        Connect4 {
            current: Player::One,
            next: Player::AI,
            board: Board::init(),
            difficulty,
        }
    }
}
