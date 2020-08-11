pub struct Board {
    pub width: u8,
    pub height: u8,
    pub state: BoardState,
}

#[derive(Debug, PartialEq)]
pub enum BoardState {
    INCOMPLETE,
    COMPLETE,
}

impl Board {
    pub fn new() -> Board {
        Board {
            width: 10,
            height: 10,
            state: BoardState::INCOMPLETE,
        }
    }

    pub fn to_string(&self) -> String {
        let mut display = String::new();

        display.push_str("       A  B  C  D  E  F  G  H  I  J\n     _______________________________\n");
        for h in 0..self.height {
            display.push_str(&format!("{:>2}  | ", h+1));
            for w in 0..self.width {
                display.push_str(" . ");
            }
            display.push_str("\n");
        }

        display
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_creation() {
        let board = Board::new();
        assert_eq!(board.height, 10);
        assert_eq!(board.width, 10);
        assert_eq!(board.state, BoardState::INCOMPLETE);
    }
}