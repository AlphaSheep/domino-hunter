use std::collections::HashMap;

use crate::coords::Coordinate;
use crate::turndef::Turn;
use crate::tables::movetables::MoveTables;

const MAX_SEARCH_DISTANCE: u8 = 25;

/// PruningTable stores the distance of each coordinate from the solved state
pub struct PruningTable {
    table: Vec<u8>,
    turn_set: Vec<Turn>,
    num_filled: usize,
    max_size: usize,
    solved_coords: Vec<usize>,
}

impl PruningTable {
    fn empty<C: Coordinate>(coord_type: C, turn_sets: &[Turn]) -> Self {
        Self {
            table: Vec::new(),
            turn_set: turn_sets.to_vec(),
            num_filled: 0,
            max_size: coord_type.get_size(),
            solved_coords: coord_type.get_solved_coords(),
        }
    }

    fn init_table(&mut self) {
        self.table = vec![u8::MAX; self.max_size];
        for solved_coord in &self.solved_coords {
            self.table[*solved_coord] = 0;
            self.num_filled += 1;
        }
    }

    fn forward_fill_single_pass(&mut self, distance: u8, movetables: &MoveTables) {
        for coord in 0..self.max_size {
            if self.table[coord] == distance {
                for turn in &self.turn_set {
                    let new_coord = movetables.apply_move_to_coord(coord, turn);
                    if self.table[new_coord] == u8::MAX {
                        self.table[new_coord] = distance + 1;
                        self.num_filled += 1;
                    }
                }
            }
        }
    }

    fn backward_fill_single_pass(&mut self, distance: u8, movetables: &MoveTables) {
        for coord in 0..self.max_size {
            if self.table[coord] > distance {
                for turn in &self.turn_set {
                    let new_coord = movetables.apply_move_to_coord(coord, turn);
                    if self.table[new_coord] == distance {
                        self.table[coord] = distance + 1;
                        self.num_filled += 1;
                        break;
                    }
                }
            }
        }
    }

    fn generate(&mut self, move_tables: &MoveTables) {
        self.init_table();
        let mut distance = 0;
        let size = self.max_size;
        let forward_threshold = size / 2;
        while self.num_filled < size {
            if self.num_filled < forward_threshold {
                self.forward_fill_single_pass(distance, move_tables);
            } else {
                self.backward_fill_single_pass(distance, move_tables);
            }
            distance += 1;

            if distance > MAX_SEARCH_DISTANCE {
                panic!("Pruning table generation failed. Not all coordinates were filled. ({})", self.num_filled);
            }
        }
    }

    pub fn new<C: Coordinate>(coord_type: C, move_tables: &MoveTables) -> Self {
        let mut tables = Self::empty(
            coord_type,
            move_tables.get_turns(),
        );
        tables.generate(move_tables);
        tables
    }

    pub fn get_distance(&self, coord: usize) -> u8 {
        self.table[coord]
    }

}