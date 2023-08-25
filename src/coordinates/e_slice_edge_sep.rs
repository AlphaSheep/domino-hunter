use crate::coordinates::Coordinate;
use crate::coordutils::{piece_distibution_to_coord, coord_to_piece_distribution};
use crate::rawcube::{RawState, StateList, TurnEffect, Edge, Swap};
use crate::turndef::Turn;

/// Coordinate to represent the separation of edges into E slice and UD slice
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ESliceEdgeSepCoord {
    coord: usize,
}

const NUM_EDGE_SEP_COMBINATIONS: usize = 495;

const SLICE_EDGES: [Edge; 4] = [Edge::BL, Edge::FL, Edge::FR, Edge::BR];
const UD_EDGES: [Edge; 8] = [Edge::UB, Edge::UL, Edge::UF, Edge::UR, Edge::DB, Edge::DL, Edge::DF, Edge::DR];

const E_UD_SWAPS: [Swap<Edge>; 4] = [
    (Edge::BL,Edge::UB), (Edge::FL,Edge::UL), (Edge::FR, Edge::UF), (Edge::BR, Edge::UR)
];

impl ESliceEdgeSepCoord {
    fn get_edges(&self) -> StateList<Edge> {
        let edge_sep_coord = self.coord % NUM_EDGE_SEP_COMBINATIONS;
        let is_slice_edge = coord_to_piece_distribution(
            edge_sep_coord, 12, 4);

        let mut edge_list = Vec::new();
        let mut slice_edge_index = 0;
        let mut ud_edge_index = 0;

        for i in 0..12 {
            if is_slice_edge[i] {
                edge_list.push(SLICE_EDGES[slice_edge_index]);
                slice_edge_index += 1;
            } else {
                edge_list.push(UD_EDGES[ud_edge_index]);
                ud_edge_index += 1;
            }
        }
        let mut edges = StateList::new(edge_list);
        edges.apply_swaps(&E_UD_SWAPS);

        edges
    }
}

impl Coordinate for ESliceEdgeSepCoord {
    fn new(raw_coord: usize) -> Self {
        Self {
            coord: raw_coord
        }
    }

    fn get_size() -> usize {
        NUM_EDGE_SEP_COMBINATIONS
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
        let edge_sep_coord = edge_sep_to_coord(state.edges);

        Self {
            coord: edge_sep_coord
        }
    }

    fn to_example_raw_state(&self) -> RawState {
        let mut state = RawState::solved();
        state.edges = self.get_edges();
        state
    }

    fn apply_raw_move(&self, turn_effect: &TurnEffect) -> Self {
        let mut edges = self.get_edges();
        turn_effect.apply_to_edges_statelist(&mut edges);
        let edge_sep_coord = edge_sep_to_coord(edges);

        Self {
            coord: edge_sep_coord
         }
    }

}

fn edge_sep_to_coord(edge_state: StateList<Edge>) -> usize {
    let mut edges = edge_state;
    edges.apply_swaps(&E_UD_SWAPS);
    let mut is_slice_edge = [false; 12];
    for i in 0..12 {
        is_slice_edge[i] = SLICE_EDGES.contains(edges.get(&i));
    }

    piece_distibution_to_coord(&is_slice_edge)
}
