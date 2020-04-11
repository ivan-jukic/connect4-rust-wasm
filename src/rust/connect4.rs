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

fn move_error_to_string(err: &PlayerMoveError) -> Option<String> {
    let to_some_str = |s| Option::Some(String::from(s));

    match err {
        PlayerMoveError::ColumnFull => to_some_str("Cannot place the token, column is full!"),
        PlayerMoveError::UnknownColumn => to_some_str("You've specified unknown column."),
        PlayerMoveError::NoError => Option::None,
    }
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

    pub fn is_column_full(&self, col_num: u8) -> bool {
        if let Option::Some(col) = num_to_col_num(col_num) {
            !self.board.can_drop_token_in_col(col)
        } else {
            false // does this make sense?
        }
    }

    pub fn print_to_console(&self) {
        self.board.print_board();
    }

    /* INIT METODS */

    /// Initialise Connect4 game for two players
    pub fn init() -> Connect4 {
        Connect4 {
            current: Player::One,
            next: Player::Two,
            board: Board::init(),
            difficulty: Difficulty::Normal,
        }
    }

    pub fn init_vs_ai(difficulty: Difficulty) -> Connect4 {
        Connect4 {
            current: Player::One,
            next: Player::AI,
            board: Board::init(),
            difficulty,
        }
    }

    /* RUNNING GAME IN TERMINAL FN's */

    fn clear_terminal() {
        print!("\x1B[2J");
    }

    /// Run game!
    pub fn run() {
        Connect4::play_moves(
            &mut Connect4::init_vs_ai(Difficulty::Normal),
            PlayerMoveError::NoError,
        );
    }

    /// Recursively play game moves until one player wins!
    fn play_moves(game: &mut Connect4, err: PlayerMoveError) {
        Connect4::clear_terminal();
        game.board.print_board();

        print!("\n\n");
        // Print any error if it happened in the previous move.
        if let Option::Some(move_err) = move_error_to_string(&err) {
            println!("{}", move_err);
        }

        match game.current {
            Player::AI => ai_round(game),
            _ => player_round(game),
        };
    }
}

fn ai_round(game: &mut Connect4) {
    game.drop_token_in_col(minimax::play_for_ai(game));
    check_win(game);
}

fn player_round(game: &mut Connect4) {
    let player_str = player_to_str(&game.current).to_uppercase();
    // Ask user for input!
    println!("Player {} playing, drop token in column > ", player_str);

    // Convert input into a column to play.
    let mut col_selected = String::new();
    io::stdin()
        .read_line(&mut col_selected)
        .expect("Failed to read line!");

    let maybe_col = str_to_col_num(col_selected.trim());

    // If column is fine
    if let Option::Some(col) = maybe_col {
        let success = game.board.drop_token(&game.current, col);

        match success {
            MoveSuccess::ColumnFull => Connect4::play_moves(game, PlayerMoveError::ColumnFull),
            MoveSuccess::UnknownColumn => {
                Connect4::play_moves(game, PlayerMoveError::UnknownColumn)
            }
            _ => check_win(game),
        };
    } else {
        Connect4::play_moves(game, PlayerMoveError::UnknownColumn)
    };
}

/// Checks if current player won, and if he has, prints it out!
fn check_win(game: &mut Connect4) {
    let player_str = player_to_str(&game.current).to_uppercase();

    // First check if current player is a winner
    if game.is_current_player_winner() {
        // If so, stop game!
        Connect4::clear_terminal();
        game.board.print_board();
        println!("\n\nPlayer {} has won!!!", player_str);
    } else {
        // Switch player and continue game!
        game.switch_players();
        Connect4::play_moves(game, PlayerMoveError::NoError);
    }
}
