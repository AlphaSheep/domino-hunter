use std::collections::HashMap;

use crate::coordinates::Coordinate;
use crate::rawcube::TurnEffect;
use crate::turndef::Turn;

/// MoveTable maps how a specific turn changes a coordinate
struct MoveTable {
    table: Vec<usize>,
}

impl MoveTable {
    fn generate_from_base_turn<C: Coordinate>(turn: &Turn) -> Self {
        let mut table = vec![0; C::get_size()];
        let turn_effect = TurnEffect::from_turn(&turn);

        for i in 0..C::get_size() {
            let coord = C::new(i);
            let new_coord = coord.apply_raw_move(&turn_effect);
            table[i] = new_coord.get_raw_value();
        }
        Self { table }
    }

    fn generate_from_compound_turn<C: Coordinate>(turn: &Turn, move_tables: &MoveTables<C>) -> Self {
        let mut table = vec![0; C::get_size()];
        let base_turns = turn.to_base_turns();

        for i in 0..C::get_size() {
            let coord = C::new(i);
            let mut new_coord = coord;
            for base_turn in &base_turns {
                new_coord = move_tables.apply_move_to_coord(new_coord, base_turn);
            }
            table.push(new_coord.get_raw_value());
        }

        Self { table }
    }
}

/// MoveTables maps how each turn from a set of turns changes a coordinate
pub struct MoveTables<C: Coordinate> {
    initialised: bool,
    table: HashMap<Turn, MoveTable>,
    turns: Vec<Turn>,
    _base_coord: C,
}

impl<C: Coordinate> MoveTables<C> {
    fn empty() -> Self {
        Self {
            initialised: false,
            table: HashMap::new(),
            _base_coord: C::new(0),
            turns: Vec::new(),
        }
    }

    pub fn get_size(&self) -> usize {
        C::get_size() * self.turns.len()
    }

    pub fn get_turns(&self) -> &[Turn] {
        &self.turns
    }

    fn generate_base_tables(&mut self, move_set: &[Turn]) {
        for turn in move_set {
            if turn.is_base_move() {
                self.table.insert(*turn, MoveTable::generate_from_base_turn::<C>(turn));
                self.turns.push(*turn);
            }
        }
        self.initialised = true;
    }

    fn generate_compound_tables(&mut self, move_set: &[Turn]) {
        for turn in move_set {
            if !turn.is_base_move() {
                self.table.insert(*turn, MoveTable::generate_from_compound_turn::<C>(turn, self));
                self.turns.push(*turn);
            }
        }
        self.initialised = true;
    }

    pub fn new(move_set: &[Turn]) -> Self {
        let mut tables = Self::empty();
        tables.generate_base_tables(move_set);
        tables.generate_compound_tables(move_set);
        tables
    }

    pub fn apply_move_to_coord(&self, coord: C, turn: &Turn) -> C {
        let table = self.table.get(&turn).unwrap();
        let new_coord = table.table[coord.get_raw_value()];
        C::new(new_coord)
    }
}
