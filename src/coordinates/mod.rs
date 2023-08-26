pub mod basic_coordinates;

use std::fmt::Debug;
use std::hash::Hash;
use std::marker::Sync;

use crate::rawcube::RawState;
use crate::turndef::Turn;

/*
A coordinate system must implement methods to convert to and from a RawState
*/
pub trait Coordinate : Copy + Clone + PartialEq + Eq + Hash + Debug + Sync {
    fn get_size(&self) -> usize;
    fn get_solved_coords(&self) -> Vec<usize>;
    fn get_allowed_turns(&self) -> Vec<Turn>;

    fn apply_turn(&self, coord: usize, turn: &Turn) -> usize;
}

pub trait BasicCoordinate : Coordinate {
    fn convert_raw_state_to_coord(&self, state: &RawState) -> usize;
    fn convert_coord_to_example_raw_state(&self, coord: usize) -> RawState;

    // Basic coordinates need to implement a static method to apply a turn to a coordinate
    // This makes it easier to parallelize move table generation
    fn apply_raw_turn(coord: usize, turn: &Turn) -> usize;
}

