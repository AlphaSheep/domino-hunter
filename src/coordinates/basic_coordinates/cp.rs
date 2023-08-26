use crate::coordinates::{Coordinate, BasicCoordinate};
use crate::coordutils::{coord_to_permutation, permutation_to_coord};
use crate::rawcube::{RawState, StateList, TurnEffect, Corner};
use crate::turndef::Turn;

/// Coordinate to represent corner permutation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CornerPermCoord {
}

const NUM_CORNER_PERM_COMBINATIONS: usize = 40320;

impl CornerPermCoord {
    pub fn new() -> Self {
        Self { }
    }
}

impl Coordinate for CornerPermCoord {
    fn get_size(&self) -> usize {
        NUM_CORNER_PERM_COMBINATIONS
    }

    fn get_solved_coords(&self) -> Vec<usize> {
        vec![0]
    }

    fn get_allowed_turns(&self) -> Vec<Turn> {
        // All outer layer turns are allowed
        Turn::get_outer_layer_turns()
    }

    fn apply_turn(&self, coord: usize, turn: &Turn) -> usize {
        let mut corners = get_corners(coord);
        let turn_effect = TurnEffect::from_turn(turn);
        turn_effect.apply_to_corners_statelist(&mut corners);

        permutation_to_coord(corners.as_slice())
    }
}

impl BasicCoordinate for CornerPermCoord {

    fn convert_raw_state_to_coord(&self, state: &RawState) -> usize {
        permutation_to_coord(state.corners.as_slice())
    }

    fn convert_coord_to_example_raw_state(&self, coord: usize) -> RawState {
        let mut state = RawState::solved();
        state.corners = get_corners(coord);
        state
    }

    fn apply_raw_turn(coord: usize, turn: &Turn) -> usize {
        let mut corners = get_corners(coord);
        let turn_effect = TurnEffect::from_turn(turn);
        turn_effect.apply_to_corners_statelist(&mut corners);

        permutation_to_coord(corners.as_slice())
    }
}

fn get_corners(coord: usize) -> StateList<Corner> {
    let mut corners = Vec::with_capacity(8);
    for corner in coord_to_permutation(coord, 8) {
        corners.push(corner.into());
    }
    StateList::new(corners)
}
