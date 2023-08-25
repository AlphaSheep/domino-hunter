use crate::coordinates::Coordinate;
use crate::coordutils::{
    coord_to_permutation, permutation_to_coord,
    piece_distibution_to_coord, get_perm_for_distribution_coord
};
use crate::rawcube::{RawState, StateList, Edge, TurnEffect};
use crate::turndef::Turn;

const NUM_EDGE_DISTRIBUTIONS: usize = 495; // 12 choose 4. Number of arrangements of 4 edges from a slice.
const NUM_SLICE_EDGE_PERMUTATIONS: usize = 24; // Number of permutations of 4 edges that belong to a slice.

const E_SLICE_EDGES: [Edge; 4] = [Edge::BL, Edge::FL, Edge::FR, Edge::BR];
const M_SLICE_EDGES: [Edge; 4] = [Edge::UB, Edge::UF, Edge::DB, Edge::DF];
const S_SLICE_EDGES: [Edge; 4] = [Edge::UL, Edge::UR, Edge::DL, Edge::DR];

const E_SLICE_SOLVED_COORD: usize = 10200;
const M_SLICE_SOLVED_COORD: usize = 1824;
const S_SLICE_SOLVED_COORD: usize = 5448;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ESliceEdgePermCoord {
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MSliceEdgePermCoord {
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SSliceEdgePermCoord {
}

impl ESliceEdgePermCoord {
    pub fn new() -> Self {
        Self { }
    }

    fn get_edges(&self, coord: usize) -> StateList<Edge> {
        let off_slice_edges = [M_SLICE_EDGES, S_SLICE_EDGES].concat();
        get_edges(coord, &E_SLICE_EDGES, &off_slice_edges)
    }

    fn get_coord_for_edges(&self, edges: &[Edge]) -> usize {
        get_coord_for_edges(edges, &E_SLICE_EDGES)
    }
}

impl MSliceEdgePermCoord {
    pub fn new() -> Self {
        Self { }
    }

    fn get_edges(&self, coord: usize) -> StateList<Edge> {
        let off_slice_edges = [E_SLICE_EDGES, S_SLICE_EDGES].concat();
        get_edges(coord, &M_SLICE_EDGES, &off_slice_edges)
    }

    fn get_coord_for_edges(&self, edges: &[Edge]) -> usize {
        get_coord_for_edges(edges, &M_SLICE_EDGES)
    }
}

impl SSliceEdgePermCoord {
    pub fn new() -> Self {
        Self { }
    }

    fn get_edges(&self, coord: usize) -> StateList<Edge> {
        let off_slice_edges = [E_SLICE_EDGES, M_SLICE_EDGES].concat();
        get_edges(coord, &S_SLICE_EDGES, &off_slice_edges)
    }

    fn get_coord_for_edges(&self, edges: &[Edge]) -> usize {
        get_coord_for_edges(edges, &S_SLICE_EDGES)
    }
}

impl Coordinate for ESliceEdgePermCoord {

    fn get_size(&self) -> usize {
        NUM_EDGE_DISTRIBUTIONS * NUM_SLICE_EDGE_PERMUTATIONS
    }

    fn get_solved_coords(&self) -> Vec<usize> {
        vec![E_SLICE_SOLVED_COORD]
    }

    fn get_allowed_turns(&self) -> Vec<Turn> {
        // All outer layer turns are allowed
        Turn::get_outer_layer_turns()
    }

    fn convert_raw_state_to_coord(&self, state: &RawState) -> usize {
        let edges = state.edges.as_slice();
        self.get_coord_for_edges(edges)
    }

    fn convert_coord_to_example_raw_state(&self, coord: usize) -> RawState {
        let mut state = RawState::solved();
        state.edges = self.get_edges(coord);
        state
    }

    fn apply_raw_move(&self, coord: usize, turn: &Turn) -> usize {
        let mut edges = self.get_edges(coord);
        let turn_effect = TurnEffect::from_turn(turn);
        turn_effect.apply_to_edges_statelist(&mut edges);
        self.get_coord_for_edges(&edges.as_slice())
    }
}

impl Coordinate for MSliceEdgePermCoord {

    fn get_size(&self) -> usize {
        NUM_EDGE_DISTRIBUTIONS * NUM_SLICE_EDGE_PERMUTATIONS
    }

    fn get_solved_coords(&self) -> Vec<usize> {
        vec![M_SLICE_SOLVED_COORD]
    }

    fn get_allowed_turns(&self) -> Vec<Turn> {
        // All outer layer turns are allowed
        Turn::get_outer_layer_turns()
    }

    fn convert_raw_state_to_coord(&self, state: &RawState) -> usize {
        let edges = state.edges.as_slice();
        self.get_coord_for_edges(edges)
    }

    fn convert_coord_to_example_raw_state(&self, coord: usize) -> RawState {
        let mut state = RawState::solved();
        state.edges = self.get_edges(coord);
        state
    }

    fn apply_raw_move(&self, coord: usize, turn: &Turn) -> usize {
        let mut edges = self.get_edges(coord);
        let turn_effect = TurnEffect::from_turn(turn);
        turn_effect.apply_to_edges_statelist(&mut edges);
        self.get_coord_for_edges(&edges.as_slice())
    }
}

impl Coordinate for SSliceEdgePermCoord {

    fn get_size(&self) -> usize {
        NUM_EDGE_DISTRIBUTIONS * NUM_SLICE_EDGE_PERMUTATIONS
    }

    fn get_solved_coords(&self) -> Vec<usize> {
        vec![S_SLICE_SOLVED_COORD]
    }

    fn get_allowed_turns(&self) -> Vec<Turn> {
        // All outer layer turns are allowed
        Turn::get_outer_layer_turns()
    }

    fn convert_raw_state_to_coord(&self, state: &RawState) -> usize {
        let edges = state.edges.as_slice();
        self.get_coord_for_edges(edges)
    }

    fn convert_coord_to_example_raw_state(&self, coord: usize) -> RawState {
        let mut state = RawState::solved();
        state.edges = self.get_edges(coord);
        state
    }

    fn apply_raw_move(&self, coord: usize, turn: &Turn) -> usize {
        let mut edges = self.get_edges(coord);
        let turn_effect = TurnEffect::from_turn(turn);
        turn_effect.apply_to_edges_statelist(&mut edges);
        self.get_coord_for_edges(&edges.as_slice())
    }
}

fn get_edges(coord: usize, slice_edges: &[Edge], off_slice_edges: &[Edge]) -> StateList<Edge> {
    let dist_coord = coord / NUM_SLICE_EDGE_PERMUTATIONS;
    let perm_coord = coord % NUM_SLICE_EDGE_PERMUTATIONS;

    let mut in_group_pieces = Vec::new();
    for i in coord_to_permutation(perm_coord, 4) {
        in_group_pieces.push(slice_edges[i]);
    }

    let edges = get_perm_for_distribution_coord(
        dist_coord, &in_group_pieces, off_slice_edges);
    StateList::new(edges)
}

fn get_coord_for_edges(edges: &[Edge], on_layer: &[Edge]) -> usize {
    let mut in_group = Vec::with_capacity(12);
    let mut in_group_pieces = Vec::with_capacity(4);

    for piece in edges {
        if on_layer.contains(piece) {
            in_group_pieces.push(*piece);
            in_group.push(true);
        } else {
            in_group.push(false);
        }
    }
    let dist_coord = piece_distibution_to_coord(&in_group);
    let perm_coord = permutation_to_coord(&in_group_pieces);
    dist_coord * NUM_SLICE_EDGE_PERMUTATIONS + perm_coord
}