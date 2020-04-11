/// Player enum

#[derive(Debug, Clone, PartialEq)]
pub enum Player {
    One,
    Two,
    AI,
}

pub fn player_to_str(player: &Player) -> String {
    match player {
        Player::One => String::from("one"),
        Player::Two => String::from("two"),
        Player::AI => String::from("AI"),
    }
}
