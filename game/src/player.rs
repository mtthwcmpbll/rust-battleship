use crate::board::Board;

pub struct Player {
    pub name: String,
    pub board: Board,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: String::from(name),
            board: Board::new(),
        }
    }
}