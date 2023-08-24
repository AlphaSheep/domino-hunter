use crate::coord::Coordinate;
use crate::coordutils::{coord_to_flip, flip_to_coord};
use crate::rawcube::{RawState, StateList, Flip, TurnEffect};
use crate::turndef::Turn;

/// Coordinate for edge orientation relative to the FB axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EOFBCoord {
    coord: usize,
}

impl EOFBCoord {
    fn get_flips(&self) -> StateList<Flip> {
        let mut flips = coord_to_flip(self.coord, 11);
        let mut flip_last = false;
        for flip in &flips {
            flip_last ^= flip == &Flip::Bad;
        }
        flips.push(if flip_last { Flip::Bad } else { Flip::Good });
        StateList::new(flips)
    }
}

impl Coordinate for EOFBCoord {
    fn new(raw_coord: usize) -> Self {
        EOFBCoord {
            coord: raw_coord
        }
    }

    fn get_size() -> usize {
        2048
    }

    fn get_solved_coords() -> Vec<usize> {
        vec![0]
    }

    fn get_raw_value(&self) -> usize {
        self.coord
    }

    fn get_allowed_turns(&self) -> Vec<Turn> {
        // All outer layer turns are allowed
        Turn::get_outer_layer_turns()
    }

    fn from_raw_state(state: RawState) -> Self {
        let flips = state.flips.as_slice();
        // We only care about the first 11 flips
        // The last flip is determined by the first 11
        // as there are always an even number bad of edges
        EOFBCoord {
            coord: flip_to_coord(&flips[0..11])
        }
    }

    fn to_example_raw_state(&self) -> RawState {
        let mut state = RawState::solved();
        state.flips = self.get_flips();
        state
    }

    fn apply_raw_move(&self, turn_effect: &TurnEffect) -> Self {
        let mut flips = self.get_flips();
        turn_effect.apply_to_flips_statelist(&mut flips);
        EOFBCoord {
            coord: flip_to_coord(&flips.as_slice()[0..11])
        }
    }
}



