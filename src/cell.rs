use std::cmp::{Ordering, Reverse};

use crate::position::Position;
use crate::cell_group::CellGroup;

#[derive(Clone)]
pub struct Cell<'a> {
    pub time: f64,
    pub position: Position,
    pub group: &'a CellGroup,
}

impl PartialEq for Cell<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for Cell<'_> {}

impl PartialOrd for Cell<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Reverse(self.time).partial_cmp(&Reverse(other.time))
    }
}

impl Ord for Cell<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.total_cmp(&other.time).reverse()
    }
}
