use crate::{rawcube::{RawState, TurnEffect}, turndef::Turn};
use std::{hash::Hash, usize};

/*
A coordinate system must implement methods to convert to and from a RawState
*/
pub trait Coordinate : Copy + Clone + PartialEq + Eq + Hash {
    fn new(raw_coord: usize) -> Self;
    fn get_size() -> usize;
    fn get_solved_coords() -> Vec<usize>;
    fn get_allowed_turns(&self) -> Vec<Turn>;
    fn get_raw_value(&self) -> usize;
    fn from_raw_state(state: RawState) -> Self;
    fn to_example_raw_state(&self) -> RawState;
    fn apply_raw_move(&self, turn_effect: &TurnEffect) -> Self;
}
