use rand::Rng;

use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn random(x_bound: i32, y_bound: i32) -> Position {
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

impl ops::Sub<Position> for Position {
    type Output = Position;
    
    fn sub(self, _rhs: Position) -> Position {
        Position {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y
        }
    }

}
