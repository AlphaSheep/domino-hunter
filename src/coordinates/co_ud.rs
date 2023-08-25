use crate::coordinates::Coordinate;
use crate::coordutils::{coord_to_twist, twist_to_coord};
use crate::rawcube::{RawState, StateList, Twist, TurnEffect};
use crate::turndef::Turn;

const NUM_CORNER_TWIST_COMBINATIONS: usize = 2187;


/// Coordinate for corner orientation relative to the UD axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct COUDCoord {
    coord: usize,
}

impl COUDCoord {
    fn get_twists(&self) -> StateList<Twist> {
        let twist_coord = self.coord;
        let mut twists = coord_to_twist(twist_coord, 7);
        let mut total_twist = 0;
        for twist in &mut twists {
            total_twist += *twist as usize;
        }
        twists.push((3 - (total_twist % 3)).into());
        StateList::new(twists)
    }
}

impl Coordinate for COUDCoord {
    fn new(raw_coord: usize) -> Self {
        Self {
            coord: raw_coord
        }
    }

    fn get_size() -> usize {
        NUM_CORNER_TWIST_COMBINATIONS
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
        let twist_coord = twist_to_coord(&state.twists.as_slice()[0..7]);
        Self {
            coord: twist_coord
        }
    }

    fn to_example_raw_state(&self) -> RawState {
        let mut state = RawState::solved();
        state.twists = self.get_twists();
        state
    }

    fn apply_raw_move(&self, turn_effect: &TurnEffect) -> Self {
        let mut twists = self.get_twists();

        turn_effect.apply_to_twists_statelist(&mut twists);

        let twist_coord = twist_to_coord(&twists.as_slice()[0..7]);

        Self {
            coord: twist_coord
         }
    }
}
