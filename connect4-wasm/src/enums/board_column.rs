// Enum to count available column to drop tokens in!

#[derive(Debug)]
pub enum BoardColumn {
    Col1,
    Col2,
    Col3,
    Col4,
    Col5,
    Col6,
    Col7,
}

/// Get a number from board column
pub fn move_to_col_num(board_move: BoardColumn) -> usize {
    match board_move {
        BoardColumn::Col1 => 0,
        BoardColumn::Col2 => 1,
        BoardColumn::Col3 => 2,
        BoardColumn::Col4 => 3,
        BoardColumn::Col5 => 4,
        BoardColumn::Col6 => 5,
        BoardColumn::Col7 => 6,
    }
}

/// Convert a string to column
pub fn str_to_col_num(str_num: &str) -> Option<BoardColumn> {
    match str_num {
        "1" => Some(BoardColumn::Col1),
        "2" => Some(BoardColumn::Col2),
        "3" => Some(BoardColumn::Col3),
        "4" => Some(BoardColumn::Col4),
        "5" => Some(BoardColumn::Col5),
        "6" => Some(BoardColumn::Col6),
        "7" => Some(BoardColumn::Col7),
        _ => None,
    }
}

/// Convert from a number to board column
pub fn num_to_col_num(num: u8) -> Option<BoardColumn> {
    match num {
        1 => Some(BoardColumn::Col1),
        2 => Some(BoardColumn::Col2),
        3 => Some(BoardColumn::Col3),
        4 => Some(BoardColumn::Col4),
        5 => Some(BoardColumn::Col5),
        6 => Some(BoardColumn::Col6),
        7 => Some(BoardColumn::Col7),
        _ => None,
    }
}
