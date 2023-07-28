use rand::Rng;

use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32
}

impl Position {
    fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }

    pub fn random(x_bound: u32, y_bound: u32) -> Position {
        Position {
            x: rand::thread_rng().gen_range(0..x_bound),
            y: rand::thread_rng().gen_range(0..y_bound)
        }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, _rhs: Position) -> Position {
        Position {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}
