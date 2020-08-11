use std::fmt;
use std::cmp::{max, min};

use crate::board::CoordinateState::{OpenWater, UndamagedShip};
use crate::board::ShipState::Undamaged;
use crate::ship::{
    Ship,
    ShipOrientation,
    ShipState
};

pub struct Board {
    pub width: u8,
    pub height: u8,
    pub state: BoardState,
    pub ships: Vec<Ship>,
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
            ships: Vec::new(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut display = String::new();

        display.push_str("       A  B  C  D  E  F  G  H  I  J\n     _______________________________\n");
        for h in 0..self.height {
            display.push_str(&format!("{:>2}  | ", h+1));
            for w in 0..self.width {
                display.push_str(&self.coord_status(w, h).to_string());
            }
            display.push_str("\n");
        }

        display
    }
    
    pub fn with_ship(&mut self, new_ship: Ship) -> &mut Board {
        for existing_ship in self.ships.iter() {
            if new_ship.overlaps(existing_ship) {
                panic!("Ships cannot overlap!");
            }
        }
        self.ships.push(new_ship);
        self
    }
    
    fn coord_status(&self, x: u8, y: u8) -> CoordinateState {
        // possible status:
        //   Empty Water:      .  
        //   Undamaged Ship:  [ ]
        //   Damaged Ship:    [X]
        //   Miss:             x
        
        // ask each ship if the status is undamaged/damaged
        for ship in self.ships.iter() {
            match ship.get_state(x, y) {
                Some(ShipState::Undamaged) => {
                    return CoordinateState::UndamagedShip;
                },
                Some(ShipState::Damaged) => {
                    return CoordinateState::DamagedShip;
                },
                None => {}
            }
        }
        
        // ask the list of previous guesses if it's a miss
        
        // else return open water
        return OpenWater;
    }
}

pub enum CoordinateState {
    OpenWater,
    UndamagedShip,
    DamagedShip,
    Miss,
}

impl fmt::Display for CoordinateState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoordinateState::OpenWater => write!(f, " . "),
            CoordinateState::UndamagedShip => write!(f, "[ ]"),
            CoordinateState::DamagedShip => write!(f, "[X]"),
            CoordinateState::Miss => write!(f, " x "),
        }
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

    #[test]
    fn board_with_nonoverlapping_ships() {
        let mut board = Board::new();
        board.with_ship(Ship::new(2, 2, 3, ShipOrientation::Horizontal));
        board.with_ship(Ship::new(5, 2, 3, ShipOrientation::Horizontal));
        board.with_ship(Ship::new(9, 1, 3, ShipOrientation::Vertical));
        board.with_ship(Ship::new(9, 4, 3, ShipOrientation::Vertical));
        
        assert_eq!(board.ships.len(), 4);
    }

    #[test]
    #[should_panic]
    fn board_with_overlapping_ships_vert_horiz_throws_error() {
        let mut board = Board::new();
        board.with_ship(Ship::new(2, 2, 4, ShipOrientation::Horizontal));
        board.with_ship(Ship::new(3, 1, 4, ShipOrientation::Vertical));
    }

    #[test]
    #[should_panic]
    fn board_with_overlapping_ships_horiz_vert_throws_error() {
        let mut board = Board::new();

        board.with_ship(Ship::new(3, 1, 4, ShipOrientation::Vertical));
        board.with_ship(Ship::new(2, 2, 4, ShipOrientation::Horizontal));
    }

    #[test]
    #[should_panic]
    fn board_with_overlapping_ships_horiz_horiz_throws_error() {
        let mut board = Board::new();

        board.with_ship(Ship::new(3, 1, 4, ShipOrientation::Horizontal));
        board.with_ship(Ship::new(5, 1, 4, ShipOrientation::Horizontal));
    }

    #[test]
    #[should_panic]
    fn board_with_overlapping_ships_vert_vert_throws_error() {
        let mut board = Board::new();

        board.with_ship(Ship::new(3, 2, 4, ShipOrientation::Vertical));
        board.with_ship(Ship::new(3, 4, 4, ShipOrientation::Vertical));
    }

    #[test]
    fn empty_board_to_string() {
        let board = Board::new();
        
        let expected = 
"       A  B  C  D  E  F  G  H  I  J
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

        assert_eq!(expected, board.to_string());
    }

    #[test]
    fn one_horizontal_ship_board_to_string() {
        let mut board = Board::new();
        board.with_ship(Ship::new(1, 2, 4, ShipOrientation::Horizontal));

        let expected = 
"       A  B  C  D  E  F  G  H  I  J
     _______________________________
 1  |  .  .  .  .  .  .  .  .  .  . 
 2  |  .  .  .  .  .  .  .  .  .  . 
 3  |  . [ ][ ][ ][ ] .  .  .  .  . 
 4  |  .  .  .  .  .  .  .  .  .  . 
 5  |  .  .  .  .  .  .  .  .  .  . 
 6  |  .  .  .  .  .  .  .  .  .  . 
 7  |  .  .  .  .  .  .  .  .  .  . 
 8  |  .  .  .  .  .  .  .  .  .  . 
 9  |  .  .  .  .  .  .  .  .  .  . 
10  |  .  .  .  .  .  .  .  .  .  . 
";

        assert_eq!(board.to_string(), expected);
    }

    #[test]
    fn one_vertical_ship_board_to_string() {
        let mut board = Board::new();
        board.with_ship(Ship::new(3, 2, 5, ShipOrientation::Vertical));

        let expected =
"       A  B  C  D  E  F  G  H  I  J
     _______________________________
 1  |  .  .  .  .  .  .  .  .  .  . 
 2  |  .  .  .  .  .  .  .  .  .  . 
 3  |  .  .  . [ ] .  .  .  .  .  . 
 4  |  .  .  . [ ] .  .  .  .  .  . 
 5  |  .  .  . [ ] .  .  .  .  .  . 
 6  |  .  .  . [ ] .  .  .  .  .  . 
 7  |  .  .  . [ ] .  .  .  .  .  . 
 8  |  .  .  .  .  .  .  .  .  .  . 
 9  |  .  .  .  .  .  .  .  .  .  . 
10  |  .  .  .  .  .  .  .  .  .  . 
";

        assert_eq!(board.to_string(), expected);
    }

    #[test]
    fn one_vertical_damaged_ship_board_to_string() {
        let mut board = Board::new();
        let mut ship = Ship::new( 3, 2, 5, ShipOrientation::Vertical);
        ship.damage(1);
        ship.damage(3);
        ship.damage(4);
        board.with_ship(ship);

        let expected =
"       A  B  C  D  E  F  G  H  I  J
     _______________________________
 1  |  .  .  .  .  .  .  .  .  .  . 
 2  |  .  .  .  .  .  .  .  .  .  . 
 3  |  .  .  . [ ] .  .  .  .  .  . 
 4  |  .  .  . [X] .  .  .  .  .  . 
 5  |  .  .  . [ ] .  .  .  .  .  . 
 6  |  .  .  . [X] .  .  .  .  .  . 
 7  |  .  .  . [X] .  .  .  .  .  . 
 8  |  .  .  .  .  .  .  .  .  .  . 
 9  |  .  .  .  .  .  .  .  .  .  . 
10  |  .  .  .  .  .  .  .  .  .  . 
";

        assert_eq!(board.to_string(), expected);
    }
}