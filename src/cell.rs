use std::cmp::{Ordering, Reverse};

use crate::position::Position;

pub struct Cell {
    pub time: f64,
    pub position: Position,
    pub cell_type: u32,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for Cell {}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Reverse(self.time).partial_cmp(&Reverse(other.time))
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.total_cmp(&other.time).reverse()
    }
}
