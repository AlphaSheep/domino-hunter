pub mod eo_fb;
pub mod co_ud;
pub mod e_slice_edge_sep;
pub mod cp;

use crate::rawcube::RawState;
use crate::turndef::Turn;
use std::hash::Hash;

/*
A coordinate system must implement methods to convert to and from a RawState
*/
pub trait Coordinate : Copy + Clone + PartialEq + Eq + Hash {
    fn get_size(&self) -> usize;
    fn get_solved_coords(&self) -> Vec<usize>;
    fn get_allowed_turns(&self) -> Vec<Turn>;

    fn convert_raw_state_to_coord(&self, state: RawState) -> usize;
    fn convert_coord_to_example_raw_state(&self, coord: usize) -> RawState;

    fn apply_raw_move(&self, coord: usize, turn: &Turn) -> usize;
}
