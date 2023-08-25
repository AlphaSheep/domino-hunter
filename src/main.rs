mod turndef;
mod rawcube;
mod coordinates;
mod coordutils;
mod mathutils;
mod movetables;
mod pruningtables;
mod solver;

use std::time::Instant;

use crate::rawcube::{TurnEffect, RawState};
use crate::turndef::{Turn, Algorithm};
use crate::coordinates::Coordinate;
use crate::movetables::MoveTables;
use crate::pruningtables::PruningTable;
use crate::solver::solve_optimally;

use crate::coordinates::eo_fb::EOFBCoord;
use crate::coordinates::co_ud::COUDCoord;
use crate::coordinates::e_slice_edge_sep::ESliceEdgeSepCoord;
use crate::coordinates::cp::CornerPermCoord;


fn main() {
    // Sanity check raw state and turn effects - do a bunch of moves and compare to a real cube
    let turns = Turn::get_vec_from_alg_string("F U' R B2 L2 M' U R' D2 S E2 L U").to_base_turns();
    let mut cube = RawState::solved();
    for turn in turns {
        let effect = TurnEffect::from_turn(&turn);
        effect.apply(&mut cube);
    }
    println!("{:?}", cube);

    // Get coordinate types
    let eo = EOFBCoord{};
    let co = COUDCoord{};
    let e_slice = ESliceEdgeSepCoord{};
    let cp = CornerPermCoord{};

    // sanity check EO coord
    let coord = 0;
    let turn = Turn::from_name("F");
    let new_coord = eo.apply_raw_move(coord, &turn);
    println!("Using raw effect on coord. {:?} {} -> new coord: {:?}", coord, turn.to_name(), new_coord);

    // Generate move tables for EO
    println!("Generating move tables");
    let now = Instant::now();
    let eo_move_tables = MoveTables::new(eo, &eo.get_allowed_turns());
    let co_move_tables = MoveTables::new(co, &co.get_allowed_turns());
    let e_slice_move_tables = MoveTables::new(e_slice, &e_slice.get_allowed_turns());
    let cp_move_tables = MoveTables::new(cp, &cp.get_allowed_turns());
    println!("Total time taken: {} seconds", (now.elapsed().as_micros() as f64 / 1_000_000.0));

    // Sanity check move tables
    let cp_coord = 6313;
    let cp_turn = Turn::from_name("F");
    let cp_new_coord = cp_move_tables.apply_move_to_coord(cp_coord, &cp_turn);
    println!("Using CP move tables. {:?} {} -> new coord: {:?}", cp_coord, cp_turn.to_name(), cp_new_coord);

    let cp_coord = 6653;
    let cp_new_coord = cp_move_tables.apply_move_to_coord(cp_coord, &cp_turn);
    println!("Using CP move tables. {:?} {} -> new coord: {:?}", cp_coord, cp_turn.to_name(), cp_new_coord);

    let cp_coord = 9676;
    let cp_new_coord = cp_move_tables.apply_move_to_coord(cp_coord, &cp_turn);
    println!("Using CP move tables. {:?} {} -> new coord: {:?}", cp_coord, cp_turn.to_name(), cp_new_coord);

    let cp_coord = 9336;
    let cp_new_coord = cp_move_tables.apply_move_to_coord(cp_coord, &cp_turn);
    println!("Using CP move tables. {:?} {} -> new coord: {:?}", cp_coord, cp_turn.to_name(), cp_new_coord);

    let cp_turn = Turn::from_name("F'");

    let cp_coord = 6313;
    let cp_new_coord = cp_move_tables.apply_move_to_coord(cp_coord, &cp_turn);
    println!("Using CP move tables. {:?} {} -> new coord: {:?}", cp_coord, cp_turn.to_name(), cp_new_coord);

    let cp_coord = 9336;
    let cp_new_coord = cp_move_tables.apply_move_to_coord(cp_coord, &cp_turn);
    println!("Using CP move tables. {:?} {} -> new coord: {:?}", cp_coord, cp_turn.to_name(), cp_new_coord);

    let cp_coord = 9676;
    let cp_new_coord = cp_move_tables.apply_move_to_coord(cp_coord, &cp_turn);
    println!("Using CP move tables. {:?} {} -> new coord: {:?}", cp_coord, cp_turn.to_name(), cp_new_coord);

    let cp_coord = 6653;
    let cp_new_coord = cp_move_tables.apply_move_to_coord(cp_coord, &cp_turn);
    println!("Using CP move tables. {:?} {} -> new coord: {:?}", cp_coord, cp_turn.to_name(), cp_new_coord);

    // Generate pruning tables for EO
    println!("Generating pruning tables");
    let now = Instant::now();
    let eo_pruning_table = PruningTable::new(eo, &eo_move_tables);
    let co_pruning_table = PruningTable::new(co, &co_move_tables);
    let e_slice_pruning_table = PruningTable::new(e_slice, &e_slice_move_tables);
    let cp_pruning_table = PruningTable::new(cp, &cp_move_tables);
    println!("Total time taken: {} seconds", (now.elapsed().as_micros() as f64 / 1_000_000.0));

    // Sanity check pruning tables
    println!("EO distance: {:?}", eo_pruning_table.get_distance(1));
    println!("CO distance: {:?}", co_pruning_table.get_distance(1));
    println!("E slice distance: {:?}", e_slice_pruning_table.get_distance(1));
    println!("CP distance: {:?}", cp_pruning_table.get_distance(1));

    // let solution = solve_optimally(new_coord, eo_move_tables, eo_pruning_table);
    // println!("Solution: {:?}", solution.to_algorithm_string());

}
