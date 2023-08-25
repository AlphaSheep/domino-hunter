use std::collections::HashMap;

use crate::coordinates::Coordinate;
use crate::turndef::Turn;
use crate::movetables::MoveTables;


/// PruningTable stores the distance of each coordinate from the solved state
pub struct PruningTable<C: Coordinate> {
    table: Vec<u8>,
    turn_set: Vec<Turn>,
    num_filled: usize,
    coord_type: C,
}

impl<C: Coordinate> PruningTable<C> {
    fn empty(turn_sets: &[Turn], coord_type: C) -> Self {
        Self {
            table: Vec::new(),
            turn_set: turn_sets.to_vec(),
            num_filled: 0,
            coord_type,
        }
    }

    fn init_table(&mut self) {
        self.table = vec![u8::MAX; self.coord_type.get_size()];
        for solved_coord in self.coord_type.get_solved_coords() {
            self.table[solved_coord] = 0;
            self.num_filled += 1;
        }
    }

    fn forward_fill_single_pass(&mut self, distance: u8, movetables: &MoveTables<C>) {
        for coord in 0..self.coord_type.get_size() {
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

    fn backward_fill_single_pass(&mut self, distance: u8, movetables: &MoveTables<C>) {
        for coord in 0..self.coord_type.get_size() {
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

    fn generate(&mut self, move_tables: &MoveTables<C>) {
        self.init_table();
        let mut distance = 0;
        let size = self.coord_type.get_size();
        let forward_threshold = size / 2;
        while self.num_filled < size {
            if self.num_filled < forward_threshold {
                self.forward_fill_single_pass(distance, move_tables);
            } else {
                self.backward_fill_single_pass(distance, move_tables);
            }
            distance += 1;

            if distance > 25 {
                for coord in 0..size {
                    if self.table[coord] == u8::MAX {
                        println!("Unfilled coord: {}", coord);
                    }
                }
                panic!("Pruning table generation failed. Not all coordinates were filled. ({})", self.num_filled);

            }
        }
    }

    pub fn new(coord_type: C, move_tables: &MoveTables<C>) -> Self {
        let mut tables = Self::empty(
            move_tables.get_turns(),
            coord_type
        );
        tables.generate(move_tables);
        tables
    }

    pub fn get_distance(&self, coord: usize) -> u8 {
        self.table[coord]
    }

}