use crate::player::Player;
use crate::board::Board;

pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub board: Board,
}

impl Game {
    pub fn new(p1: &str, p2: &str) -> Game {
        Game {
            player1: Player::new(p1),
            player2: Player::new(p2),
            board: Board::new(),
        }
    }

    pub fn display(&self) -> String {
        let mut display = String::new();
        display.push_str("==================================\n");
        display.push_str("TURN: 1\n\n");

        display.push_str(&format!("{}:\n", &self.player1.name.as_str()));
        display.push_str(&self.player1.board.to_string());
        display.push_str("\n");
        display.push_str(&format!("{}:\n", &self.player2.name.as_str()));
        display.push_str(&self.player2.board.to_string());

        display
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_creation() {
        let game = Game::new("greyson", "julia");

        let expected = "\
==================================
TURN: 1

greyson:
       A  B  C  D  E  F  G  H  I  J
     _______________________________
 1  |  .  .  .  .  .  .  .  .  .  . 
 2  |  .  .  .  .  .  .  .  .  .  . 
 3  |  .  .  .  .  .  .  .  .  .  . 
 4  |  .  .  .  .  .  .  .  .  .  . 
 5  |  .  .  .  .  .  .  .  .  .  . 
 6  |  .  .  .  .  .  .  .  .  .  . 
 7  |  .  .  .  .  .  .  .  .  .  . 
 8  |  .  .  .  .  .  .  .  .  .  . 
 9  |  .  .  .  .  .  .  .  .  .  . 
10  |  .  .  .  .  .  .  .  .  .  . 

julia:
       A  B  C  D  E  F  G  H  I  J
     _______________________________
 1  |  .  .  .  .  .  .  .  .  .  . 
 2  |  .  .  .  .  .  .  .  .  .  . 
 3  |  .  .  .  .  .  .  .  .  .  . 
 4  |  .  .  .  .  .  .  .  .  .  . 
 5  |  .  .  .  .  .  .  .  .  .  . 
 6  |  .  .  .  .  .  .  .  .  .  . 
 7  |  .  .  .  .  .  .  .  .  .  . 
 8  |  .  .  .  .  .  .  .  .  .  . 
 9  |  .  .  .  .  .  .  .  .  .  . 
10  |  .  .  .  .  .  .  .  .  .  . 
";

        assert_eq!(game.display(), expected);
    }
}