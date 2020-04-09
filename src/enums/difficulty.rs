/// Type for determining difficulty

/**
 * Different difficulties of the game! Each difficulty corresponds to the
 * depth to which the minimax algorithm will look ahead for possible outcomes.
 */
#[derive(Debug, Clone)]
pub enum Difficulty {
    Test,
    Easy,
    Normal,
    Hard,
    VeryHard,
    NoChance,
}

pub fn difficulty_to_depth(difficulty: Difficulty) -> u8 {
    match difficulty {
        Difficulty::Test => 1,
        Difficulty::Easy => 2,
        Difficulty::Normal => 3,
        Difficulty::Hard => 5,
        Difficulty::VeryHard => 6,
        Difficulty::NoChance => 10,
    }
}
