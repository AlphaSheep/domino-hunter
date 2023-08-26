use crate::coords::{Coordinate, BasicCoordinate};
use crate::utils::coordutils::{
    coord_to_permutation, permutation_to_coord,
    piece_distibution_to_coord, get_perm_for_distribution_coord
};
use crate::rawcube::{RawState, StateList, Corner, TurnEffect};
use crate::turndef::Turn;


const NUM_CORNER_DISTRIBUTIONS: usize = 70; // 8 choose 4. Number of arrangements of 4 corners from the U or D layer.
const NUM_LAYER_CORNER_PERMUTATIONS: usize = 24; // Number of permutations of 4 corners that belonmg to a layer.

const U_LAYER_CORNERS: [Corner; 4] = [Corner::UBL, Corner::UFL, Corner::UFR, Corner::UBR];
const D_LAYER_CORNERS: [Corner; 4] = [Corner::DBL, Corner::DFL, Corner::DFR, Corner::DBR];


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UCornerPermCoord {
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DCornerPermCoord {
}

impl UCornerPermCoord {
    pub fn new() -> Self {
        Self { }
    }

    fn get_corners(self, coord: usize) -> StateList<Corner> {
        get_corners(coord, &U_LAYER_CORNERS, &D_LAYER_CORNERS)
    }

    fn get_coord_for_corners(self, corners: &[Corner]) -> usize {
        get_coord_for_corners(corners, &U_LAYER_CORNERS)
    }
}

impl DCornerPermCoord {
    pub fn new() -> Self {
        Self { }
    }

    fn get_corners(self, coord: usize) -> StateList<Corner> {
        get_corners(coord, &D_LAYER_CORNERS, &U_LAYER_CORNERS)
    }

    fn get_coord_for_corners(self, corners: &[Corner]) -> usize {
        get_coord_for_corners(corners, &D_LAYER_CORNERS)
    }
}

impl Coordinate for UCornerPermCoord {

    fn get_size(&self) -> usize {
        NUM_CORNER_DISTRIBUTIONS * NUM_LAYER_CORNER_PERMUTATIONS
    }

    fn get_solved_coords(&self) -> Vec<usize> {
        vec![0]
    }

    fn get_allowed_turns(&self) -> Vec<Turn> {
        Turn::get_outer_layer_turns()
    }

    fn apply_turn(&self, coord: usize, turn: &Turn) -> usize {
        let mut corners = get_corners(coord, &U_LAYER_CORNERS, &D_LAYER_CORNERS);
        let turn_effect = TurnEffect::from_turn(turn);
        turn_effect.apply_to_corners_statelist(&mut corners);
        get_coord_for_corners(corners.as_slice(), &U_LAYER_CORNERS)
    }
}

impl BasicCoordinate for UCornerPermCoord {

    fn convert_raw_state_to_coord(&self, state: &RawState) -> usize {
        let corners = state.corners.as_slice();
        self.get_coord_for_corners(corners)
    }

    fn convert_coord_to_example_raw_state(&self, coord: usize) -> RawState {
        let mut state = RawState::solved();
        state.corners = self.get_corners(coord);
        state
    }

    fn apply_raw_turn(coord: usize, turn: &Turn) -> usize {
        let mut corners = get_corners(coord, &U_LAYER_CORNERS, &D_LAYER_CORNERS);
        let turn_effect = TurnEffect::from_turn(turn);
        turn_effect.apply_to_corners_statelist(&mut corners);
        get_coord_for_corners(corners.as_slice(), &U_LAYER_CORNERS)
    }
}

impl Coordinate for DCornerPermCoord {

    fn get_size(&self) -> usize {
        NUM_CORNER_DISTRIBUTIONS * NUM_LAYER_CORNER_PERMUTATIONS
    }

    fn get_solved_coords(&self) -> Vec<usize> {
        vec![1656]
    }

    fn get_allowed_turns(&self) -> Vec<Turn> {
        Turn::get_outer_layer_turns()
    }

    fn apply_turn(&self, coord: usize, turn: &Turn) -> usize {
        let mut corners = get_corners(coord, &D_LAYER_CORNERS, &U_LAYER_CORNERS);
        let turn_effect = TurnEffect::from_turn(turn);
        turn_effect.apply_to_corners_statelist(&mut corners);
        get_coord_for_corners(corners.as_slice(), &D_LAYER_CORNERS)
    }
}

impl BasicCoordinate for DCornerPermCoord {

    fn convert_raw_state_to_coord(&self, state: &RawState) -> usize {
        let corners = state.corners.as_slice();
        self.get_coord_for_corners(corners)
    }

    fn convert_coord_to_example_raw_state(&self, coord: usize) -> RawState {
        let mut state = RawState::solved();
        state.corners = self.get_corners(coord);
        state
    }

    fn apply_raw_turn(coord: usize, turn: &Turn) -> usize {
        let mut corners = get_corners(coord, &D_LAYER_CORNERS, &U_LAYER_CORNERS);
        let turn_effect = TurnEffect::from_turn(turn);
        turn_effect.apply_to_corners_statelist(&mut corners);
        get_coord_for_corners(corners.as_slice(), &D_LAYER_CORNERS)
    }
}

fn get_corners(coord: usize, on_layer: &[Corner], off_layer: &[Corner]) -> StateList<Corner> {
    let dist_coord = coord / NUM_LAYER_CORNER_PERMUTATIONS;
    let perm_coord = coord % NUM_LAYER_CORNER_PERMUTATIONS;

    let mut in_group_pieces = Vec::new();
    for i in coord_to_permutation(perm_coord, 4) {
        in_group_pieces.push(on_layer[i]);
    }

    let corners = get_perm_for_distribution_coord(
        dist_coord, &in_group_pieces, off_layer);
    StateList::new(corners)
}

fn get_coord_for_corners(corners: &[Corner], on_layer: &[Corner]) -> usize {
    let mut in_group = Vec::with_capacity(8);
    let mut in_group_pieces = Vec::with_capacity(4);
    for piece in corners {
        if on_layer.contains(piece) {
            in_group_pieces.push(*piece);
            in_group.push(true);
        } else {
            in_group.push(false);
        }
    }
    let dist_coord = piece_distibution_to_coord(&in_group);
    let perm_coord = permutation_to_coord(&in_group_pieces);
    dist_coord * NUM_LAYER_CORNER_PERMUTATIONS + perm_coord
}