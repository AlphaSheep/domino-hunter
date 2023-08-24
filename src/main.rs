mod turndef;
mod rawcube;
mod coord;
mod coordutils;
mod mathutils;
mod movetables;
mod pruningtables;
mod solver;

mod eo_fb;

use std::time::Instant;

use crate::rawcube::{TurnEffect, RawState};
use crate::turndef::{Turn, TurnVec};
use crate::coord::Coordinate;
use crate::eo_fb::coord::EOFBCoord;
use crate::movetables::MoveTables;
use crate::pruningtables::PruningTable;
use crate::solver::solve_optimally;


fn main() {
    // Sanity check raw state and turn effects - do a bunch of moves and compare to a real cube
    let turns = Turn::get_vec_from_alg_string("F U' R B2 L2 M' U R' D2 S E2 L U").to_base_turns();
    let mut cube = RawState::solved();
    for turn in turns {
        let effect = TurnEffect::from_turn(&turn);
        effect.apply(&mut cube);
    }
    println!("{:?}", cube);

    // sanity check EO coord
    let coord = EOFBCoord::new(0);
    let turn = Turn::from_name("F");
    let new_coord = coord.apply_raw_move(&TurnEffect::from_turn(&turn));
    println!("Using raw effect on coord. {:?} {} -> new coord: {:?}", coord, turn.to_name(), new_coord);

    // Generate move tables for EO
    println!("Generating move tables");
    let now = Instant::now();
    let eo_move_tables = MoveTables::<EOFBCoord>::new(&Turn::get_base_outer_layer_turns());
    println!("Total time taken: {} seconds", (now.elapsed().as_micros() as f64 / 1_000_000.0));

    // Sanity check move tables
    let new_coord = eo_move_tables.apply_move_to_coord(coord, &turn);
    println!("Using move tables. {:?} {} -> new coord: {:?}", coord, turn.to_name(), new_coord);

    // Generate pruning tables for EO
    println!("Generating pruning tables");
    let now = Instant::now();
    let eo_pruning_table = PruningTable::<EOFBCoord>::new(&eo_move_tables);
    println!("Total time taken: {} seconds", (now.elapsed().as_micros() as f64 / 1_000_000.0));

    // Sanity check pruning tables
    println!("distance: {:?}", eo_pruning_table.get_distance(&new_coord));

    let solution = solve_optimally(new_coord, eo_move_tables, eo_pruning_table);
    println!("Solution: {:?}", solution.to_algorithm_string());

}
