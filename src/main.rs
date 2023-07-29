use image::{ImageBuffer, Rgb, RgbImage};

use std::collections::{BinaryHeap, HashMap};

use crate::position::Position;
use crate::cell_group::CellGroup;
use crate::cell::Cell;

mod position;
mod cell_group;
mod cell;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const NUMBER_OF_STARTING_CELLS: u32 = 200;

fn main() {
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    let mut remaining_cells: BinaryHeap<Cell> = BinaryHeap::new();
    let mut occupied_cells: HashMap<Position, Cell> = HashMap::new();
    let mut cell_groups: Vec<CellGroup> = Vec::new();

    for _ in 0..NUMBER_OF_STARTING_CELLS {
        let cell_group = CellGroup::random();

        cell_groups.push(cell_group);
    }

    for cell_group in cell_groups.iter() {
        let mut position_for_new_cell = Position::random(WIDTH, HEIGHT);

        while occupied_cells.contains_key(&position_for_new_cell) {
            position_for_new_cell = Position::random(WIDTH, HEIGHT);
        }

        let new_cell = Cell {
            time: 0.0,
            position: position_for_new_cell,
            group: &cell_group
        };

        occupied_cells.insert(position_for_new_cell, new_cell);

    }
    
    for cell in occupied_cells.values() {
        image.put_pixel(cell.position.x, 
                        cell.position.y, 
                        cell.group.color);
    }

    image.save("test.png").unwrap();
}
