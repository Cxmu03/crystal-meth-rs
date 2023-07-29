use image::{ImageBuffer, Rgb, RgbImage};

use std::collections::{BinaryHeap, HashMap};

use crate::position::Position;
use crate::cell_group::CellGroup;
use crate::cell::Cell;

mod position;
mod cell_group;
mod cell;

const WIDTH: u32 = 5000;
const HEIGHT: u32 = 1080;
const NUMBER_OF_STARTING_CELLS: u32 = 5000;

fn main() {
    let mut start_image = RgbImage::new(WIDTH, HEIGHT);
    let mut final_image = RgbImage::new(WIDTH, HEIGHT);

    let mut remaining_cells: BinaryHeap<Cell> = BinaryHeap::new();
    let mut occupied_cells: HashMap<Position, Cell> = HashMap::new();
    let mut cell_groups: Vec<CellGroup> = Vec::new();

    for _ in 0..NUMBER_OF_STARTING_CELLS {
        let cell_group = CellGroup::random();

        cell_groups.push(cell_group);
    }

    for cell_group in cell_groups.iter() {
        let mut position_for_new_cell = Position::random(WIDTH as i32, HEIGHT as i32);

        while occupied_cells.contains_key(&position_for_new_cell) {
            position_for_new_cell = Position::random(WIDTH as i32, HEIGHT as i32);
        }

        let new_cell = Cell {
            time: 0.0,
            position: position_for_new_cell,
            group: &cell_group
        };

        occupied_cells.insert(position_for_new_cell, new_cell.clone());
        remaining_cells.push(new_cell);
    }

    for cell in occupied_cells.values() {
        start_image.put_pixel(cell.position.x as u32, 
                        cell.position.y as u32, 
                        cell.group.color);
    }

    occupied_cells.clear();

    start_image.save("start-positions.png").unwrap();
    drop(start_image);

    while !remaining_cells.is_empty() {
        let current_cell: Cell<'_> = remaining_cells.pop().unwrap();

        if occupied_cells.contains_key(&current_cell.position) {
           continue; 
        }

        let spreads_to: Vec<Position> = vec![
            current_cell.position - Position::new(0, 1), // Above
            current_cell.position + Position::new(0, 1), // Below
            current_cell.position - Position::new(1, 0), // Left
            current_cell.position + Position::new(1, 0)  // Right
        ];

        for (i, &position) in spreads_to.iter().enumerate() {
            if position.x < 0 || position.x >= WIDTH as i32 || position.y < 0 || position.y >= HEIGHT as i32 {
                continue;     
            }

            let new_cell = Cell {
                time: current_cell.time + current_cell.group.spread_rates[i],
                position: position,
                group: current_cell.group
            };

            remaining_cells.push(new_cell);
        }

        final_image.put_pixel(current_cell.position.x as u32,
                              current_cell.position.y as u32,
                              current_cell.group.color);
        
        occupied_cells.insert(current_cell.position, current_cell);
    }
    
    final_image.save("final.png").unwrap();
}
