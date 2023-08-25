use crate::coordinates::Coordinate;
use crate::coordutils::{coord_to_permutation, permutation_to_coord};
use crate::rawcube::{RawState, StateList, TurnEffect, Corner};
use crate::turndef::Turn;

/// Coordinate to represent corner permutation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CornerPermCoord {
    coord: usize,
}

const NUM_CORNER_PERM_COMBINATIONS: usize = 40320;

impl CornerPermCoord {
    fn get_corners(&self) -> StateList<Corner> {
        let mut corners = Vec::with_capacity(8);
        for corner in coord_to_permutation(self.coord, 8) {
            corners.push(corner.into());
        }
        StateList::new(corners)
    }
}

impl Coordinate for CornerPermCoord {
    fn new(raw_coord: usize) -> Self {
        Self {
            coord: raw_coord
        }
    }

    fn get_size() -> usize {
        NUM_CORNER_PERM_COMBINATIONS
    }

    fn get_solved_coords() -> Vec<usize> {
        vec![0]
    }

    fn get_raw_value(&self) -> usize {
        self.coord
    }

    fn get_allowed_turns() -> Vec<Turn> {
        // All outer layer turns are allowed
        Turn::get_outer_layer_turns()
    }

    fn from_raw_state(state: RawState) -> Self {
        Self {
            coord: permutation_to_coord(state.corners.as_slice())
        }
    }

    fn to_example_raw_state(&self) -> RawState {
        let mut state = RawState::solved();
        state.corners = self.get_corners();
        state
    }

    fn apply_raw_move(&self, turn_effect: &TurnEffect) -> Self {
        let mut corners = self.get_corners();
        turn_effect.apply_to_corners_statelist(&mut corners);
        Self {
            coord: permutation_to_coord(corners.as_slice())
        }
    }
}