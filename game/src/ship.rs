pub struct Ship {
    x: u8,
    y: u8,
    size: u8,
    orientation: ShipOrientation,
    damage: [bool; 10],
}

pub enum ShipOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq)]
pub enum ShipState {
    Undamaged,
    Damaged
}

impl Ship {
    pub fn new(x: u8, y: u8, size: u8, orientation: ShipOrientation) -> Ship {
        Ship {
            x,
            y,
            size,
            orientation,
            damage: [false; 10],
        }
    }
    
    pub fn damage(&mut self, i: usize) {
        self.damage[i] = true;
    }

    pub fn overlaps(&self, other_ship: &Ship) -> bool {
        match self.orientation {
            ShipOrientation::Horizontal => {
                match other_ship.orientation {
                    ShipOrientation::Vertical => {
                        return self.y >= other_ship.y
                            && self.y < other_ship.y+other_ship.size
                            && self.x <= other_ship.x
                            && self.x >= other_ship.x-self.size;
                    },
                    ShipOrientation::Horizontal => {
                        return self.y == other_ship.y
                            && ((self.x >= other_ship.x && self.x < other_ship.x+other_ship.size)
                            || (other_ship.x >= self.x && other_ship.x < self.x+self.size));
                    }
                }
            },
            ShipOrientation::Vertical => {
                match other_ship.orientation {
                    ShipOrientation::Vertical => {
                        return self.x == other_ship.x
                            && ((self.y >= other_ship.y && self.y < other_ship.y+other_ship.size)
                            || (other_ship.y >= self.y && other_ship.y < self.y+self.size));
                    },
                    ShipOrientation::Horizontal => {
                        return other_ship.y >= self.y
                            && other_ship.y < self.y+self.size
                            && other_ship.x <= self.x
                            && other_ship.x >= self.x-other_ship.size;
                    }
                }
            },
        }
    }

    pub fn get_state(&self, x: u8, y: u8) -> Option<ShipState> {
        match self.orientation {
            ShipOrientation::Horizontal => {
                if y == self.y
                    && x >= self.x
                    && x < self.x+self.size {

                    if (self.damage[(x-self.x) as usize]) {
                        return Some(ShipState::Damaged);
                    } else {
                        return Some(ShipState::Undamaged);
                    }
                }
            },
            ShipOrientation::Vertical => {
                if x == self.x
                    && y >= self.y
                    && y < self.y+self.size {

                    if (self.damage[(y-self.y) as usize]) {
                        return Some(ShipState::Damaged);
                    } else {
                        return Some(ShipState::Undamaged);
                    }
                }
            },
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undamaged_ship_get_state() {
        let ship = Ship::new(0, 0, 3, ShipOrientation::Horizontal);
        assert_eq!(ship.get_state(0, 0), Some(ShipState::Undamaged));
        assert_eq!(ship.get_state(1, 0), Some(ShipState::Undamaged));
        assert_eq!(ship.get_state(2, 0), Some(ShipState::Undamaged));
        assert_eq!(ship.get_state(3, 0), None);
        assert_eq!(ship.get_state(0, 1), None);
    }

    #[test]
    fn damaged_ship_get_state() {
        let mut ship = Ship::new(0, 0, 3, ShipOrientation::Horizontal);
        ship.damage(0);

        assert_eq!(ship.get_state(0, 0).unwrap(), ShipState::Damaged);
        assert_eq!(ship.get_state(1, 0).unwrap(), ShipState::Undamaged);
        assert_eq!(ship.get_state(2, 0).unwrap(), ShipState::Undamaged);
        assert_eq!(ship.get_state(3, 0), None);
        assert_eq!(ship.get_state(0, 1), None);
    }
}