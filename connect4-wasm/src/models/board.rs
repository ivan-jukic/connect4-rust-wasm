#![allow(unused_variables)]

/// Playing board!
use crate::enums::board_column::{move_to_col_num, BoardColumn};
use crate::enums::player::Player;

/// Consts
pub const ROW_NUM: i8 = 6;
pub const COL_NUM: i8 = 7;

/// Define if a position on the board was played or not, and by whom
#[derive(Debug, Clone, PartialEq)]
pub enum MovePlayed {
    Played(Player),
    NotPlayed,
}

/// Indicates success or failure when we try to drop a token
#[derive(Debug, PartialEq)]
pub enum MoveSuccess {
    Placed(usize),
    ColumnFull,
    UnknownColumn,
}

pub fn is_successful_move(m: &MoveSuccess) -> bool {
    match m {
        MoveSuccess::Placed(_) => true,
        _ => false,
    }
}

type BoardState = Vec<Vec<MovePlayed>>;

#[derive(Debug, Clone)]
pub struct Board {
    board_state: BoardState,
}

impl Board {
    // Static init method!
    pub fn init() -> Board {
        Board {
            board_state: vec![vec![MovePlayed::NotPlayed; ROW_NUM as usize]; COL_NUM as usize],
        }
    }

    /// Initialises board from an input! TODO add a test!
    pub fn init_from(str_board: Vec<Vec<String>>) -> Board {
        str_board
            .iter()
            .enumerate()
            .fold(Board::init(), |board, (i, col)| {
                col.iter().enumerate().fold(board, |mut curr, (j, val)| {
                    curr.board_state[i][j] = match val.as_str() {
                        "player" => MovePlayed::Played(Player::One),
                        "ai" => MovePlayed::Played(Player::AI),
                        _ => MovePlayed::NotPlayed,
                    };
                    curr
                })
            })
    }

    pub fn get_board_state(self) -> BoardState {
        self.board_state
    }

    /// Method to check if a token can be dropped in a column.
    pub fn can_drop_token_in_col(&self, column: BoardColumn) -> bool {
        let col_num: usize = move_to_col_num(column);

        if col_num >= COL_NUM as usize {
            false
        } else {
            self.board_state
                .get(col_num)
                .unwrap_or(&vec![])
                .contains(&MovePlayed::NotPlayed)
        }
    }

    /// Returns i64 representation of the current board state for a player. We
    /// can then use bitwise operations to check if the player has won.
    pub fn get_status_num_for_player(&self, player: &Player) -> u64 {
        let total_cols: usize = COL_NUM as usize;
        let palyed_move = MovePlayed::Played(player.clone());

        self.board_state
            .iter()
            .enumerate()
            .fold(0, |acc, (col_idx, col)| {
                acc + col.iter().enumerate().fold(0, |acc2, (row_idx, val)| {
                    if let MovePlayed::Played(p) = val {
                        if p == player {
                            acc2 + (2 as u64).pow((row_idx * total_cols + col_idx) as u32)
                        } else {
                            acc2
                        }
                    } else {
                        acc2
                    }
                })
            })
    }

    /// Method that "drops" a token in the column, and marks it as played by player.
    pub fn drop_token(&mut self, player: &Player, column: BoardColumn) -> MoveSuccess {
        // Num of the colum to play!
        let col_num: usize = move_to_col_num(column);

        match self.board_state.get_mut(col_num) {
            Option::Some(col_played) => {
                // Find the index where we need to put the token to!
                let not_played_idx: Option<usize> =
                    col_played.iter().position(|m| *m == MovePlayed::NotPlayed);

                match not_played_idx {
                    Option::Some(idx) => {
                        // "Drop" token!
                        self.board_state[col_num] = col_played
                            .iter()
                            .enumerate()
                            .map(|(i, val)| {
                                if i == idx {
                                    MovePlayed::Played(player.clone())
                                } else {
                                    val.clone()
                                }
                            })
                            .collect();

                        // Return info about successful token drop with row idx.
                        MoveSuccess::Placed(idx)
                    }
                    Option::None => MoveSuccess::ColumnFull,
                }
            }
            Option::None => MoveSuccess::UnknownColumn,
        }
    }
}

// TEST!!

#[test]
fn can_drop_tokens() {
    let mut board = Board::init();
    let success = board.drop_token(&Player::One, BoardColumn::Col1);

    // Test placing first token by player one, on row index 0
    assert_eq!(success, MoveSuccess::Placed(0));
    assert_eq!(board.can_drop_token_in_col(BoardColumn::Col1), true);
    assert_eq!(
        *board.board_state.get(0).unwrap(),
        vec![
            MovePlayed::Played(Player::One),
            MovePlayed::NotPlayed,
            MovePlayed::NotPlayed,
            MovePlayed::NotPlayed,
            MovePlayed::NotPlayed,
            MovePlayed::NotPlayed,
        ]
    );

    // Test placing second token by player two, on row index 1
    let success = board.drop_token(&Player::Two, BoardColumn::Col1);
    assert_eq!(success, MoveSuccess::Placed(1));
    assert_eq!(board.can_drop_token_in_col(BoardColumn::Col1), true);
    assert_eq!(
        *board.board_state.get(0).unwrap(),
        vec![
            MovePlayed::Played(Player::One),
            MovePlayed::Played(Player::Two),
            MovePlayed::NotPlayed,
            MovePlayed::NotPlayed,
            MovePlayed::NotPlayed,
            MovePlayed::NotPlayed,
        ]
    );

    // Drop more tokens to fill the first column
    board.drop_token(&Player::One, BoardColumn::Col1);
    board.drop_token(&Player::Two, BoardColumn::Col1);
    board.drop_token(&Player::One, BoardColumn::Col1);
    board.drop_token(&Player::Two, BoardColumn::Col1);

    // Try to drop one more!
    let success = board.drop_token(&Player::One, BoardColumn::Col1);
    assert_eq!(success, MoveSuccess::ColumnFull);
    assert_eq!(board.can_drop_token_in_col(BoardColumn::Col1), false);
}

#[test]
fn can_calc_correct_player_status() {
    let mut board = Board::init();
    board.drop_token(&Player::One, BoardColumn::Col1);
    board.drop_token(&Player::One, BoardColumn::Col1);
    board.drop_token(&Player::One, BoardColumn::Col1);
    board.drop_token(&Player::One, BoardColumn::Col1);

    // First column should be a win for player one
    assert_eq!(board.get_status_num_for_player(&Player::One), 2113665);

    let mut board = Board::init();
    board.drop_token(&Player::One, BoardColumn::Col1);
    board.drop_token(&Player::Two, BoardColumn::Col2);
    board.drop_token(&Player::One, BoardColumn::Col2);
    board.drop_token(&Player::Two, BoardColumn::Col3);
    board.drop_token(&Player::One, BoardColumn::Col4);
    board.drop_token(&Player::Two, BoardColumn::Col3);
    board.drop_token(&Player::One, BoardColumn::Col3);
    board.drop_token(&Player::Two, BoardColumn::Col4);
    board.drop_token(&Player::One, BoardColumn::Col5);
    board.drop_token(&Player::Two, BoardColumn::Col4);
    board.drop_token(&Player::One, BoardColumn::Col4);

    assert_eq!(board.get_status_num_for_player(&Player::One), 16843033);
}
