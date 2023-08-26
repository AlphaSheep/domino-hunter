use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use nohash_hasher::NoHashHasher;

use crate::coordinates::{Coordinate, BasicCoordinate};
use crate::rawcube::TurnEffect;
use crate::turndef::{Turn, Algorithm};

/// MoveTable maps how a specific turn changes a coordinate
struct MoveTable {
    table: Vec<usize>,
}

impl MoveTable {
    fn generate_from_base_turn<C: BasicCoordinate>(coord_type: C, turn: &Turn) -> Self {
        let mut table = vec![usize::MAX; coord_type.get_size()];

        for coord in 0..coord_type.get_size() {
            let new_coord = coord_type.apply_raw_move(coord, &turn);
            table[coord] = new_coord;
        }
        Self {
            table,
        }
    }
}

impl MoveTable {
    fn generate_from_compound_turn(turn: &Turn, size: usize, move_tables: &MoveTables) -> Self {
        let mut table = vec![usize::MAX; size];
        let base_turns = turn.to_base_turns();

        for coord in 0..size {
            let mut new_coord = coord;
            for base_turn in &base_turns {
                new_coord = move_tables.apply_move_to_coord(new_coord, base_turn);
            }
            table[coord] = new_coord;
        }
        Self {
            table,
        }
    }
}

/// MoveTables maps how each turn from a set of turns changes a coordinate
pub struct MoveTables {
    table: HashMap<Turn, MoveTable, BuildHasherDefault<NoHashHasher<usize>>>,
    turns: Vec<Turn>,
}

impl MoveTables {
    fn empty() -> Self {
        Self {
            table: HashMap::with_hasher(BuildHasherDefault::default()),
            turns: Vec::new(),
        }
    }

    pub fn get_turns(&self) -> &[Turn] {
        &self.turns
    }

    fn generate_base_tables<C: BasicCoordinate>(&mut self, coord_type: C, move_set: &[Turn]) {
        let mut base_turns = Vec::new();
        for turn in move_set.to_base_turns() {
            if turn.is_base_move() && !base_turns.contains(&turn) {
                base_turns.push(turn);
            }
        }

        for turn in base_turns {
            self.table.insert(turn, MoveTable::generate_from_base_turn(coord_type, &turn));
            self.turns.push(turn);
        }
    }

    fn generate_compound_tables(&mut self, size: usize, move_set: &[Turn]) {
        for turn in move_set {
            if !turn.is_base_move() {
                self.table.insert(*turn, MoveTable::generate_from_compound_turn(turn, size, self));
                self.turns.push(*turn);
            }
        }
    }

    pub fn new_basic_table<C: BasicCoordinate>(coord_type: C, move_set: &[Turn]) -> Self {
        let mut tables = Self::empty();
        tables.generate_base_tables(coord_type, move_set);
        tables.generate_compound_tables(coord_type.get_size(), move_set);
        tables
    }

    pub fn apply_move_to_coord(&self, coord: usize, turn: &Turn) -> usize {
        let table = self.table.get(&turn)
        .expect("Move table not found for turn");
        let new_coord = table.table[coord];
        new_coord
    }
}
