use std::collections::HashMap;

use crate::coord::Coordinate;
use crate::turndef::Turn;
use crate::movetables::MoveTables;


/// PruningTable stores the distance of each coordinate from the solved state
pub struct PruningTable<C: Coordinate> {
    table: Vec<u8>,
    turn_set: Vec<Turn>,
    num_filled: usize,
    base_coord: C,
}

impl<C: Coordinate> PruningTable<C> {
    fn empty(turn_sets: &[Turn]) -> Self {
        Self {
            table: Vec::new(),
            turn_set: turn_sets.to_vec(),
            num_filled: 0,
            base_coord: C::new(0),
        }
    }

    fn init_table(&mut self) {
        self.table = vec![u8::MAX; C::get_size()];
        for solved_coord in C::get_solved_coords() {
            self.table[solved_coord] = 0;
            self.num_filled += 1;
        }
    }

    fn forward_fill_single_pass(&mut self, distance: u8, movetables: &MoveTables<C>) {
        for i in 0..C::get_size() {
            if self.table[i] == distance {
                let coord = C::new(i);
                for turn in &self.turn_set {
                    let new_coord = movetables.apply_move_to_coord(coord, turn);
                    let j = new_coord.get_raw_value();
                    if self.table[j] == u8::MAX {
                        self.table[j] = distance + 1;
                        self.num_filled += 1;
                    }
                }
            }
        }
    }

    fn backward_fill_single_pass(&mut self, distance: u8, movetables: &MoveTables<C>) {
        for i in 0..C::get_size() {
            if self.table[i] > distance {
                let coord = C::new(i);
                for turn in &self.turn_set {
                    let new_coord = movetables.apply_move_to_coord(coord, turn);
                    let j = new_coord.get_raw_value();
                    if self.table[j] == distance {
                        self.table[i] = distance + 1;
                        self.num_filled += 1;
                        break;
                    }
                }
            }
        }
    }

    fn generate(&mut self, move_tables: &MoveTables<C>) {
        self.init_table();
        let mut distance = 0;
        let size = C::get_size();
        let forward_threshold = size / 2;
        while self.num_filled < size {
            if self.num_filled < forward_threshold {
                self.forward_fill_single_pass(distance, move_tables);
            } else {
                self.backward_fill_single_pass(distance, move_tables);
            }
            distance += 1;

            if distance > 25 {
                panic!("Pruning table generation failed. Not all coordinates were filled. ({})", self.num_filled);
            }
        }
    }

    pub fn new(move_tables: &MoveTables<C>) -> Self {
        let mut tables = Self::empty(move_tables.get_turns());
        tables.generate(move_tables);
        tables
    }

    pub fn get_distance(&self, coord: &C) -> u8 {
        self.table[coord.get_raw_value()]
    }

}