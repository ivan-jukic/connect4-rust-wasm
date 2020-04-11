use crate::models::board::{COL_NUM, ROW_NUM};

/// Board point type!
pub type BoardPoint = (i8, i8);

/// List of board points (in 1D) that define one win state!
pub type BoardPoints = Vec<i8>;

/// Win mask, contains 1's on positions that determine winning state of the board
pub type WinMask = u64;

/// Pre-calculated mask values, using the get_win_masks function! Tests below
/// validate that the function is working properly.
pub fn get_cached_win_masks() -> Vec<WinMask> {
    vec![
        15,
        30,
        60,
        120,
        1920,
        3840,
        7680,
        15360,
        245760,
        491520,
        983040,
        1966080,
        2130440,
        2113665,
        4227330,
        4260880,
        8454660,
        8521760,
        16843009,
        16909320,
        17043520,
        31457280,
        33686018,
        33818640,
        62914560,
        67372036,
        67637280,
        125829120,
        134744072,
        135274560,
        251658240,
        270549120,
        272696320,
        541098240,
        545392640,
        1082196480,
        1090785280,
        2155905152,
        2181570560,
        2164392960,
        4026531840,
        4311810304,
        4328785920,
        8053063680,
        8623620608,
        8657571840,
        16106127360,
        17247241216,
        17315143680,
        32212254720,
        34630287360,
        34905128960,
        69260574720,
        69810257920,
        138521149440,
        139620515840,
        275955859456,
        277042298880,
        279241031680,
        515396075520,
        551911718912,
        554084597760,
        1030792151040,
        1103823437824,
        1108169195520,
        2061584302080,
        2207646875648,
        2216338391040,
        4123168604160,
    ]
}

/// Calculate win masks, that will be used to determine the win state.
pub fn get_win_masks() -> Vec<WinMask> {
    get_board_points()
        .iter()
        .map(|bp| get_win_points(*bp))
        .flatten()
        .filter(|wp| wp.len() > 0)
        .map(get_mask)
        .collect()
}

/// Function which takes in a vec of numbers and transforms them into a single
/// number, by summing poweres of two of all numbers, thus creating a binary mask.
pub fn get_mask(wp: BoardPoints) -> WinMask {
    wp.iter().clone().map(|n| (2 as u64).pow(*n as u32)).sum()
}

/***** MODULE PRIVATE FUNCTIONS *****/

/// A simple enum to add additional context to the number which should define the
/// total number of columns.
enum Cols {
    Cols(i8),
}

/// Extract actual number of cols from a type!
fn get_cols_val(Cols::Cols(val): &Cols) -> i8 {
    *val // De-reference!
}

/// Return a vector of all points on the connect4 playing board!
fn get_board_points() -> Vec<BoardPoint> {
    (0..COL_NUM)
        .map(|x| (0..ROW_NUM).map(move |y| (x as i8, y as i8)))
        .flatten()
        .collect()
}

/// For a specific board point, gets all relevant win point combinations, looking
/// from left to right. For each point, right to left points would be covered by
/// finding win points of a board point that came before.
fn get_win_points((col, row): BoardPoint) -> Vec<BoardPoints> {
    vec![
        // Check up
        get_up_win_points(col, row),
        // Check right
        get_right_win_points(col, row),
        // Check diagonals, if there's enough space to the right
        get_diagonal_win_points(col, row),
    ]
}

/// From the current col / row, looking up, what are the winning points.
fn get_up_win_points(col: i8, row: i8) -> BoardPoints {
    if row < ROW_NUM - 3 {
        calc_range(Cols::Cols(COL_NUM), |x| (col, row + x))
    } else {
        vec![]
    }
}

/// Looking from the current point, to the right, what are the winning points.
fn get_right_win_points(col: i8, row: i8) -> BoardPoints {
    if col < COL_NUM - 3 {
        calc_range(Cols::Cols(COL_NUM), |y| (col + y, row))
    } else {
        vec![]
    }
}

/// Looking from the current point, if we go diagonally up, what are the winning
/// points, or if we cannot go diagonally up, what are the winning points
/// diagonally down.
fn get_diagonal_win_points(col: i8, row: i8) -> BoardPoints {
    if col < COL_NUM - 3 {
        if row < ROW_NUM - 3 {
            // up diagonal
            calc_range(Cols::Cols(COL_NUM), |xy| (col + xy, row + xy))
        } else {
            // down diagonal
            calc_range(Cols::Cols(COL_NUM), |xy| (col + xy, row - xy))
        }
    } else {
        vec![]
    }
}

/// Creates a range of four 1D points, that indicate a win state.
fn calc_range<F: FnMut(i8) -> BoardPoint>(cols: Cols, func: F) -> Vec<i8> {
    (0..4)
        .map(func)
        .map(|(c, r)| (r * get_cols_val(&cols)) + c) // transform 2d point to 1d
        .collect()
}

/* Board points!

(5) | 35 | 36 | 37 | 38 | 39 | 40 | 41 |
(4) | 28 | 29 | 30 | 31 | 32 | 33 | 34 |
(3) | 21 | 22 | 23 | 24 | 25 | 26 | 27 |
(2) | 14 | 15 | 16 | 17 | 18 | 19 | 20 |
(1) |  7 |  8 |  9 | 10 | 11 | 12 | 13 |
(0) |  0 |  1 |  2 |  3 |  4 |  5 |  6 |
      (0)  (1)  (2)  (3)  (4)  (5)  (6)

*/

// Module tests!!

#[test]
fn can_get_correct_col_num() {
    assert_eq!(get_cols_val(&Cols::Cols(4)), 4);
    assert_eq!(get_cols_val(&Cols::Cols(5)), 5);
    assert_eq!(get_cols_val(&Cols::Cols(6)), 6);
    assert_ne!(get_cols_val(&Cols::Cols(7)), -7);
    assert_ne!(get_cols_val(&Cols::Cols(8)), -8);
}

#[test]
fn theres_42_board_points() {
    assert_eq!(get_board_points().len(), 42);
}

#[test]
fn theres_these_points_on_board() {
    let points = get_board_points();

    assert_eq!(points.contains(&(0, 0)), true);
    assert_eq!(points.contains(&(4, 5)), true);
    assert_eq!(points.contains(&(6, 5)), true);
    assert_eq!(points.contains(&(1, 1)), true);
    assert_eq!(points.contains(&(2, 5)), true);
    assert_eq!(points.contains(&(6, 4)), true);
}

#[test]
fn but_theres_no_these_points() {
    let points = get_board_points();

    assert_eq!(points.contains(&(-1, 0)), false);
    assert_eq!(points.contains(&(6, 6)), false);
    assert_eq!(points.contains(&(7, -6)), false);
    assert_eq!(points.contains(&(50, 20)), false);
}

#[test]
fn these_are_some_win_points() {
    // We calc win points for all board points, here we're just testing two!
    let win_points = get_win_points((0, 0));
    let win_points_2 = get_win_points((2, 2));

    assert_eq!(
        win_points,
        vec![[0, 7, 14, 21], [0, 1, 2, 3], [0, 8, 16, 24]]
    );

    assert_eq!(
        win_points_2,
        vec![[16, 23, 30, 37], [16, 17, 18, 19], [16, 24, 32, 40]]
    );
}

#[test]
fn these_are_some_win_numbers() {
    assert_eq!(get_mask(vec![0, 1, 2, 3]), 15);
    assert_eq!(get_mask(vec![0, 8, 16, 24]), 16843009);
    assert_eq!(get_mask(vec![0, 7, 14, 21]), 2113665);
}
