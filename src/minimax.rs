use rand::Rng;

/// App mods
use crate::connect4::Connect4;
use crate::enums::difficulty::*;
use crate::models::board::{is_successful_move, COL_NUM};

/// Type indicating minimax type
#[derive(Debug, PartialEq)]
enum Procedure {
    Maximising, // Maximising minimum gain for the player
    Minimising, // Minimising maximum gain for the opponent
}

fn switch_procedure(current: &Procedure) -> Procedure {
    if *current == Procedure::Maximising {
        Procedure::Minimising
    } else {
        Procedure::Maximising
    }
}

/// Entry point for the minimax algorithm.
pub fn play_for_ai(game: &Connect4) -> u8 {
    // We're assuming that we want to maximise for the current player!
    minimax_run(
        game,
        Procedure::Maximising,
        difficulty_to_depth(game.difficulty.clone()),
    )
    .0
}

fn minimax_run(game: &Connect4, procedure: Procedure, depth: u8) -> (u8, i8) {
    let default_res = (1, 0);

    let column_weights: Vec<(u8, i8)> = (1..(COL_NUM + 1) as u8)
        .filter_map(|col_idx| {
            let mut game_copy = game.clone();
            let success = game_copy.drop_token_in_col(col_idx);

            if is_successful_move(&success) {
                Option::Some(on_successful_token_drop(
                    &mut game_copy,
                    &procedure,
                    &depth,
                    &col_idx,
                ))
            } else {
                Option::None
            }
        })
        .collect();

    let only_weights: Vec<i8> = column_weights.clone().into_iter().map(|w| w.1).collect();

    let target_weight: Option<i8> = match procedure {
        Procedure::Maximising => only_weights.into_iter().max(),
        Procedure::Minimising => only_weights.into_iter().min(),
    };

    match target_weight {
        Option::Some(weight) => {
            let possible_moves: Vec<(u8, i8)> = column_weights
                .into_iter()
                .filter(|tpl| tpl.1 == weight)
                .collect();
            match possible_moves.as_slice() {
                [] => default_res,
                [res] => res.clone(),
                _ => {
                    let selected_idx = rand::thread_rng().gen_range(0, possible_moves.len());
                    match possible_moves.get(selected_idx) {
                        Option::Some(&val) => val,
                        Option::None => default_res,
                    }
                }
            }
        }
        Option::None => default_res,
    }
}

fn on_successful_token_drop(
    game: &mut Connect4,
    procedure: &Procedure,
    depth: &u8,
    col: &u8,
) -> (u8, i8) {
    let is_column_full = game.is_column_full(*col);
    let is_game_finished = game.is_current_player_winner();

    (
        *col,
        // If game's finished, or we can't drop any more tokens in column, or
        // we've reached the max depth for the algorithm.
        if is_column_full || is_game_finished || *depth == 0 {
            get_current_move_weight(&game, &procedure, &depth)
        // We can add more tokens in this column, and we're not at full depth
        } else {
            game.switch_players();

            // Just taking the chosen weight from the next minimax run, as this
            // will become the weight of the current column!
            minimax_run(&game, switch_procedure(&procedure), *depth - 1).1
        },
    )
}

/// Taking into account current state on the board, return value depending on
/// the maximising or minimising step!
fn get_current_move_weight(game: &Connect4, procedure: &Procedure, depth: &u8) -> i8 {
    let win = game.is_current_player_winner();
    if win {
        (*depth as i8 + 1)
            * match procedure {
                Procedure::Maximising => 1,
                Procedure::Minimising => -1,
            }
    } else {
        0
    }
}

#[test]
fn ai_should_play_col_3() {
    let mut game = Connect4::init_vs_ai(Difficulty::Test);

    game.drop_token_in_col(3);
    game.drop_token_in_col(3);
    game.drop_token_in_col(3);

    assert_eq!(play_for_ai(&game), 3);
}

#[test]
fn ai_should_play_col_1_or_5() {
    let mut game = Connect4::init_vs_ai(Difficulty::Test);

    game.drop_token_in_col(2);
    game.drop_token_in_col(3);
    game.drop_token_in_col(4);

    let played = play_for_ai(&game);
    assert_eq!(played == 1 || played == 5, true);
}

#[test]
fn ai_should_play_col_4() {
    let mut game = Connect4::init_vs_ai(Difficulty::Test);

    game.drop_token_in_col(5);
    game.drop_token_in_col(6);
    game.drop_token_in_col(7);

    assert_eq!(play_for_ai(&game), 4);
}
