use std::collections::HashMap;

use crate::coordinates::Coordinate;
use crate::rawcube::TurnEffect;
use crate::turndef::{Turn, Algorithm};

/// MoveTable maps how a specific turn changes a coordinate
struct MoveTable<C: Coordinate> {
    table: Vec<usize>,
    coord_type: C,
}

impl<C: Coordinate> MoveTable<C> {
    fn generate_from_base_turn(coord_type: C, turn: &Turn) -> Self {
        let mut table = vec![usize::MAX; coord_type.get_size()];
        let turn_effect = TurnEffect::from_turn(&turn);

        for coord in 0..coord_type.get_size() {
            let new_coord = coord_type.apply_raw_move(coord, &turn);
            table[coord] = new_coord;
        }
        Self {
            table,
            coord_type,
        }
    }

    fn generate_from_compound_turn(coord_type: C, turn: &Turn, move_tables: &MoveTables<C>) -> Self {
        let mut table = vec![usize::MAX; coord_type.get_size()];
        let base_turns = turn.to_base_turns();

        for coord in 0..coord_type.get_size() {
            let mut new_coord = coord;
            for base_turn in &base_turns {
                new_coord = move_tables.apply_move_to_coord(new_coord, base_turn);
            }
            table[coord] = new_coord;
        }
        Self {
            table,
            coord_type,
        }
    }
}

/// MoveTables maps how each turn from a set of turns changes a coordinate
pub struct MoveTables<C: Coordinate> {
    table: HashMap<Turn, MoveTable<C>>,
    turns: Vec<Turn>,
    coord_type: C,
}

impl<C: Coordinate> MoveTables<C> {
    fn empty(coord_type: C) -> Self {
        Self {
            table: HashMap::new(),
            coord_type,
            turns: Vec::new(),
        }
    }

    pub fn get_size(&self) -> usize {
        self.coord_type.get_size() * self.turns.len()
    }

    pub fn get_turns(&self) -> &[Turn] {
        &self.turns
    }

    fn generate_base_tables(&mut self, move_set: &[Turn]) {
        let mut base_turns = Vec::new();
        for turn in move_set.to_base_turns() {
            if turn.is_base_move() && !base_turns.contains(&turn) {
                base_turns.push(turn);
            }
        }

        for turn in base_turns {
            self.table.insert(turn, MoveTable::generate_from_base_turn(self.coord_type, &turn));
            self.turns.push(turn);
        }
    }

    fn generate_compound_tables(&mut self, move_set: &[Turn]) {
        for turn in move_set {
            if !turn.is_base_move() {
                self.table.insert(*turn, MoveTable::generate_from_compound_turn(self.coord_type, turn, self));
                self.turns.push(*turn);
            }
        }
    }

    pub fn new(coord_type: C, move_set: &[Turn]) -> Self {
        let mut tables = Self::empty(coord_type);
        tables.generate_base_tables(move_set);
        tables.generate_compound_tables(move_set);
        tables
    }

    pub fn apply_move_to_coord(&self, coord: usize, turn: &Turn) -> usize {
        let table = self.table.get(&turn)
        .expect("Move table not found for turn");
        let new_coord = table.table[coord];
        new_coord
    }
}
