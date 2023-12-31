use crate::coords::Coordinate;
use crate::tables::movetables::MoveTables;
use crate::tables::pruningtables::PruningTable;
use crate::turndef::Turn;

pub fn solve_optimally(mut coord: usize, movetables: MoveTables, pruningtables: PruningTable) -> Vec<Turn> {
    let mut current_distance = pruningtables.get_distance(coord);
    let mut solution = Vec::new();

    while current_distance > 0 {
        for turn in movetables.get_turns() {
            let new_coord = movetables.apply_move_to_coord(coord, turn);
            let new_distance = pruningtables.get_distance(new_coord);
            if new_distance < current_distance {
                current_distance = new_distance;
                solution.push(*turn);
                coord = new_coord;
                break;
            }
        }
    }
    solution
}