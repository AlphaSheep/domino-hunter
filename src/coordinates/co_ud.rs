use crate::coordinates::Coordinate;
use crate::coordutils::{coord_to_twist, twist_to_coord};
use crate::rawcube::{RawState, StateList, Twist, TurnEffect};
use crate::turndef::Turn;

const NUM_CORNER_TWIST_COMBINATIONS: usize = 2187;


/// Coordinate for corner orientation relative to the UD axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct COUDCoord {
}

impl COUDCoord {
    fn new() -> Self {
        Self { }
    }

    fn get_twists(&self, coord: usize) -> StateList<Twist> {
        let mut twists = coord_to_twist(coord, 7);
        let mut total_twist = 0;
        for twist in &mut twists {
            total_twist += *twist as usize;
        }
        twists.push((3 - (total_twist % 3)).into());
        StateList::new(twists)
    }
}

impl Coordinate for COUDCoord {
    fn get_size(&self) -> usize {
        NUM_CORNER_TWIST_COMBINATIONS
    }

    fn get_solved_coords(&self) -> Vec<usize> {
        vec![0]
    }

    fn get_allowed_turns(&self) -> Vec<Turn> {
        // All outer layer turns are allowed
        Turn::get_outer_layer_turns()
    }

    fn convert_raw_state_to_coord(&self, state: RawState) -> usize {
        twist_to_coord(&state.twists.as_slice()[0..7])
    }

    fn convert_coord_to_example_raw_state(&self, coord: usize) -> RawState {
        let mut state = RawState::solved();
        state.twists = self.get_twists(coord);
        state
    }

    fn apply_raw_move(&self, coord: usize, turn: &Turn) -> usize {
        let mut twists = self.get_twists(coord);
        let turn_effect = TurnEffect::from_turn(turn);
        turn_effect.apply_to_twists_statelist(&mut twists);

        twist_to_coord(&twists.as_slice()[0..7])
    }
}
