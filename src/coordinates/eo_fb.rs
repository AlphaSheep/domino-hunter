use crate::coordinates::Coordinate;
use crate::coordutils::{coord_to_flip, flip_to_coord};
use crate::rawcube::{RawState, StateList, Flip, TurnEffect};
use crate::turndef::Turn;

/// Coordinate for edge orientation relative to the FB axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EOFBCoord {

}

impl EOFBCoord {
    fn new() -> Self {
        Self { }
    }

    fn get_flips(&self, coord: usize) -> StateList<Flip> {
        let mut flips = coord_to_flip(coord, 11);
        let mut flip_last = false;
        for flip in &flips {
            flip_last ^= flip == &Flip::Bad;
        }
        flips.push(if flip_last { Flip::Bad } else { Flip::Good });
        StateList::new(flips)
    }
}

impl Coordinate for EOFBCoord {

    fn get_size(&self) -> usize {
        2048
    }

    fn get_solved_coords(&self) -> Vec<usize> {
        vec![0]
    }

    fn get_allowed_turns(&self) -> Vec<Turn> {
        // All outer layer turns are allowed
        Turn::get_outer_layer_turns()
    }

    fn convert_raw_state_to_coord(&self, state: RawState) -> usize {
        let flips = state.flips.as_slice();
        // We only care about the first 11 flips
        // The last flip is determined by the first 11
        // as there are always an even number bad of edges
        flip_to_coord(&flips[0..11])
    }

    fn convert_coord_to_example_raw_state(&self, coord: usize) -> RawState {
        let mut state = RawState::solved();
        state.flips = self.get_flips(coord);
        state
    }

    fn apply_raw_move(&self, coord: usize, turn: &Turn) -> usize {
        let mut flips = self.get_flips(coord);
        let turn_effect = TurnEffect::from_turn(turn);
        turn_effect.apply_to_flips_statelist(&mut flips);
        flip_to_coord(&flips.as_slice()[0..11])
    }
}



