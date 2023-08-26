mod turndef;
mod rawcube;
mod coords;
mod utils;
mod tables;
mod solver;

use std::time::Instant;

use crate::coords::{Coordinate, BasicCoordinate};
use crate::tables::movetables::MoveTables;
use crate::tables::pruningtables::PruningTable;
use crate::rawcube::RawState;
use crate::solver::solve_optimally;

use crate::coords::basic_coords::eo_fb::EOFBCoord;
use crate::coords::basic_coords::co_ud::COUDCoord;
// use crate::coordinates::basic_coordinates::e_slice_edge_sep::ESliceEdgeSepCoord;
use crate::coords::basic_coords::cp::CornerPermCoord;
use crate::coords::basic_coords::u_d_corner_perms::{UCornerPermCoord, DCornerPermCoord};
use crate::coords::basic_coords::e_m_s_edges::{ESliceEdgePermCoord, MSliceEdgePermCoord, SSliceEdgePermCoord};


fn main() {
    // Get coordinate types
    let eo = EOFBCoord{};
    let co = COUDCoord{};
    // let e_slice = ESliceEdgeSepCoord{};
    let cp = CornerPermCoord{};
    let u_corners = UCornerPermCoord{};
    let d_corners = DCornerPermCoord{};
    let e_slice_edges = ESliceEdgePermCoord{};
    let m_slice_edges = MSliceEdgePermCoord{};
    let s_slice_edges = SSliceEdgePermCoord{};

    // Check solved position
    let cube = RawState::solved();
    println!("EO solved: {:?}", eo.convert_raw_state_to_coord(&cube));
    println!("CO solved: {:?}", co.convert_raw_state_to_coord(&cube));
    // println!("E slice: {:?}", e_slice.convert_raw_state_to_coord(&cube));
    println!("CP solved: {:?}", cp.convert_raw_state_to_coord(&cube));
    // println!("U corners: {:?}", u_corners.convert_raw_state_to_coord(&cube));
    // println!("D corners: {:?}", d_corners.convert_raw_state_to_coord(&cube));
    println!("E slice edges solved: {:?}", e_slice_edges.convert_raw_state_to_coord(&cube));
    println!("M slice edges solved: {:?}", m_slice_edges.convert_raw_state_to_coord(&cube));
    println!("S slice edges solved: {:?}", s_slice_edges.convert_raw_state_to_coord(&cube));

    // Generate move tables for EO
    println!("Generating move tables");
    let now = Instant::now();
    let eo_move_tables = MoveTables::new_basic_table(eo, &eo.get_allowed_turns());
    let co_move_tables = MoveTables::new_basic_table(co, &co.get_allowed_turns());
    // let e_slice_move_tables = MoveTables::new_basic_table(e_slice, &e_slice.get_allowed_turns());
    let cp_move_tables = MoveTables::new_basic_table(cp, &cp.get_allowed_turns());
    // let u_corners_move_tables = MoveTables::new_basic_table(u_corners, &u_corners.get_allowed_turns());
    // let d_corners_move_tables = MoveTables::new_basic_table(d_corners, &d_corners.get_allowed_turns());
    let e_slice_edges_move_tables = MoveTables::new_basic_table(e_slice_edges, &e_slice_edges.get_allowed_turns());
    let m_slice_edges_move_tables = MoveTables::new_basic_table(m_slice_edges, &m_slice_edges.get_allowed_turns());
    let s_slice_edges_move_tables = MoveTables::new_basic_table(s_slice_edges, &s_slice_edges.get_allowed_turns());
    println!("Total time taken: {} seconds", (now.elapsed().as_micros() as f64 / 1_000_000.0));

    // Generate pruning tables for EO
    println!("Generating pruning tables");
    let now = Instant::now();
    let eo_pruning_table = PruningTable::new(eo, &eo_move_tables);
    let co_pruning_table = PruningTable::new(co, &co_move_tables);
    // let e_slice_pruning_table = PruningTable::new(e_slice, &e_slice_move_tables);
    let cp_pruning_table = PruningTable::new(cp, &cp_move_tables);
    // let u_corners_pruning_table = PruningTable::new(u_corners, &u_corners_move_tables);
    // let d_corners_pruning_table = PruningTable::new(d_corners, &d_corners_move_tables);
    let e_slice_edges_pruning_table = PruningTable::new(e_slice_edges, &e_slice_edges_move_tables);
    let m_slice_edges_pruning_table = PruningTable::new(m_slice_edges, &m_slice_edges_move_tables);
    let s_slice_edges_pruning_table = PruningTable::new(s_slice_edges, &s_slice_edges_move_tables);
    println!("Total time taken: {} seconds", (now.elapsed().as_micros() as f64 / 1_000_000.0));

    // Sanity check pruning tables
    println!("EO distance: {:?}", eo_pruning_table.get_distance(1));
    println!("CO distance: {:?}", co_pruning_table.get_distance(1));
    // println!("E slice distance: {:?}", e_slice_pruning_table.get_distance(1));
    println!("CP distance: {:?}", cp_pruning_table.get_distance(1));
    // println!("U corners distance: {:?}", u_corners_pruning_table.get_distance(1));
    // println!("D corners distance: {:?}", d_corners_pruning_table.get_distance(1));
    println!("E slice edges distance: {:?}", e_slice_edges_pruning_table.get_distance(1));
    println!("M slice edges distance: {:?}", m_slice_edges_pruning_table.get_distance(1));
    println!("S slice edges distance: {:?}", s_slice_edges_pruning_table.get_distance(1));

    // let solution = solve_optimally(new_coord, eo_move_tables, eo_pruning_table);
    // println!("Solution: {:?}", solution.to_algorithm_string());

}
