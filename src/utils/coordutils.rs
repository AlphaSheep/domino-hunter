/*
The following are useful functions for converting to and from various
types of coordinate. Types implementing the Coordinate trait can use these
*/

use crate::utils::mathutils::{binomial, factorial};
use crate::rawcube::{Flip, Twist, PieceState};

/*
Flip is a list of flips of arbitrary length.
For 12 pieces, there are 2^12 = 4096 possible states.
If the flip of the final piece is determined by the flips of the other
pieces, then you only need pass in a slcie of the first 11 flips.
For 11 flips, there are 2^11 = 2048 possible states.
*/
pub fn flip_to_coord(flips: &[Flip]) -> usize {
    let mut coord = 0;
    for flip in flips.iter().rev() {
        coord <<= 1;
        coord += *flip as usize;
    }
    coord
}

pub fn coord_to_flip(coord: usize, num_pieces: usize) -> Vec<Flip> {
    let mut flips = Vec::with_capacity(num_pieces);
    let mut coord = coord;
    for _ in 0..num_pieces {
        flips.push(coord.into());
        coord >>= 1;
    }
    flips
}

/*
Twist is a list of twists of arbitrary length.
For 8 pieces, there are 3^8 = 6561 possible twists.
If the twist of the final piece is determined by the twists of the other
pieces, then you only need pass in a slcie of the first 7 twists.
For 7 twists, there are 3^7 = 2187 possible states.
*/

pub fn twist_to_coord(twist: &[Twist]) -> usize {
    let mut coord = 0;
    for flip in twist.iter().rev() {
        coord *= 3;
        coord += *flip as usize;
    }
    coord
}

pub fn coord_to_twist(coord: usize, num_pieces: usize) -> Vec<Twist> {
    let mut twists = Vec::with_capacity(num_pieces);
    let mut coord = coord;
    for _ in 0..num_pieces {
        twists.push(coord.into());
        coord /= 3;
    }
    twists
}

/*
It is often useful to know whether a permutation has even or odd parity.
*/
pub fn is_even_parity(perm: &[usize]) -> bool {
    // TODO - make this faster.
    // This is O(n^2) because it's quick and easy to implement.
    // It would be nice to use a O(n log n) method.
    let n = perm.len();
    let mut result = true;
    for i in 0..(n-1) {
        for j in (i+1)..n {
            result ^= perm[i] > perm[j];
        }
    }
    result
}

/*
Represent a permutation as a coordinate.
We treat this as a variable base number system where the coefficient of the nth
digit from the right is n!.
For 12 pieces, this gives 12! = 479 001 600 possible states.
For 8 pieces, this gives 8! = 40 320 possible states.
*/
pub fn permutation_to_coord<T: PartialOrd>(positions: &[T]) -> usize {
    let mut coord = 0;
    for i in (1..positions.len()).rev() {
        for j in 0..i {
            if positions[i] < positions[j] {
                coord += 1;
            }
        }
        coord *= i;
    }
    coord
}

pub fn coord_to_permutation(mut coord: usize, num_pieces: usize) -> Vec<usize> {
    let mut state = Vec::with_capacity(num_pieces);
    let mut available = Vec::with_capacity(num_pieces);
    for i in (0..num_pieces).rev() {
        state.push(0);
        available.push(i);
    }
    for i in (0..num_pieces).rev() {
        let factor = factorial(i);
        let index = coord / factor;
        state[i] = available.remove(index);
        coord %= factor;
    }
    state
}

fn get_factors(num_pieces: usize) -> Vec<usize> {
    let mut factors = Vec::with_capacity(num_pieces);
    factors.push(1);
    for i in 1..num_pieces {
        factors.push(factorial(i));
    }
    factors
}

/*
Represent a permutation as a coordinate, but ignore the permutation of the two left pieces.
Assume that the parity is even, so the permutation of the pieces in the first two positions
is determined by the permutation of the other pieces.
For 12 pieces, this gives 12!/2 = 239 500 800 possible states.
For 8 pieces, this gives 8!/2 = 20 160 possible states.
*/
pub fn permutation_to_coord_even_parity<T: PartialOrd>(positions: &[T]) -> usize {
    let mut coord = 0;
    for i in (2..positions.len()).rev() {
        for j in 0..i {
            if positions[i] < positions[j] {
                coord += 1;
            }
        }
        if i > 2 {
            coord *= i;
        }
    }
    coord
}

pub fn coord_to_permutation_even_parity(coord: usize, num_pieces: usize) -> Vec<usize> {
    let mut state = coord_to_permutation(coord*2, num_pieces);
    if !is_even_parity(state.as_slice()) {
        println!("odd parity");
        state.swap(0, 1);
    }
    state
}

/*
Represent how a particular piece type is distributed amongst other pieces.
For each possible position that is not occupied by the piece type of interest, we capture the
number of possible combinations of pieces that come before that position. The coordinate
is then the sum of all of these combinations. This is useful for encoding, for example,
the distribution of E layer edges. For a distribution of 4 pieces in 12 possible positions,
this gives binomial(12,4) = 495 possible states.
*/
pub fn piece_distibution_to_coord(state: &[bool]) -> usize {
    let mut coord = 0;
    let mut n = 0;
    let mut k = 0;
    for interesting_piece in state.iter().rev() {
        if *interesting_piece {
            k += 1;
            n += 1;
        } else {
            n += 1;
            if (n >= 1) && (k >= 1) {
                coord += binomial(n-1, k-1);
            }
        }
    }
    coord
}

pub fn coord_to_piece_distribution(mut coord: usize, num_positions: usize, num_pieces_of_interest: usize) -> Vec<bool> {
    let mut state: Vec<bool> = Vec::with_capacity(num_positions);
    for _ in 0..num_positions {
        state.push(false);
    }
    let mut num_left = num_pieces_of_interest;

    for j in 0..num_positions {
        let n = num_positions - j - 1;
        let n_choose_k = binomial(n, num_left-1);
        if coord >= n_choose_k {
            coord -= n_choose_k;
        }
        else {
            state[j] = true;
            num_left -= 1;
        }
        if num_left == 0 {
            break;
        }
    }
    state
}

/*
Given a distribution coordinate and an ordered list of pieces in and out of the group,
combines the in and out of group pieces into a single list.
*/
pub fn get_perm_for_distribution_coord<T: PieceState + Copy>(coord:usize, in_group_pieces: &[T], out_of_group_pieces: &[T]) -> Vec<T> {
    let num_in_group = in_group_pieces.len();
    let num_positions = num_in_group + out_of_group_pieces.len();

    let layer_distribution = coord_to_piece_distribution(coord, num_positions, num_in_group);
    let mut on_layer_index = 0;
    let mut off_layer_index = 0;

    let mut pieces = Vec::new();
    for belongs_in_layer in layer_distribution {
        if belongs_in_layer {
            pieces.push(in_group_pieces[on_layer_index]);
            on_layer_index += 1;
        } else {
            pieces.push(out_of_group_pieces[off_layer_index]);
            off_layer_index += 1;
        }
    }
    pieces
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_to_coord() {
        let state = &[Flip::Good, Flip::Good, Flip::Good];
        assert_eq!(flip_to_coord(state), 0);

        let state = &[Flip::Bad, Flip::Good, Flip::Good];
        assert_eq!(flip_to_coord(state), 1);

        let state = &[Flip::Good, Flip::Bad, Flip::Good];
        assert_eq!(flip_to_coord(state), 2);

        let state = &[Flip::Bad; 12];
        assert_eq!(flip_to_coord(state), 4095);
    }

    #[test]
    fn test_coord_to_flip() {
        let coord = 0;
        let num_pieces = 3;
        let expected = &[Flip::Good, Flip::Good, Flip::Good];
        assert_eq!(coord_to_flip(coord, num_pieces), expected);

        let coord = 1;
        let num_pieces = 3;
        let expected = &[Flip::Bad, Flip::Good, Flip::Good];
        assert_eq!(&coord_to_flip(coord, num_pieces), expected);

        let coord = 2;
        let num_pieces = 3;
        let expected = &[Flip::Good, Flip::Bad, Flip::Good];
        assert_eq!(&coord_to_flip(coord, num_pieces), expected);

        let coord = 4095;
        let num_pieces = 12;
        let expected = &[Flip::Bad; 12];
        assert_eq!(&coord_to_flip(coord, num_pieces), expected);
    }

    #[test]
    fn test_twist_to_coord() {
        let state = &[Twist::None, Twist::None, Twist::None];
        assert_eq!(twist_to_coord(state), 0);

        let state = &[Twist::CW, Twist::None, Twist::None];
        assert_eq!(twist_to_coord(state), 1);

        let state = &[Twist::ACW, Twist::None, Twist::None];
        assert_eq!(twist_to_coord(state), 2);

        let state = &[Twist::None, Twist::CW, Twist::None];
        assert_eq!(twist_to_coord(state), 3);

        let state = &[Twist::ACW; 8];
        assert_eq!(twist_to_coord(state), 6560);
    }

    #[test]
    fn test_coord_to_twist() {
        let coord = 0;
        let num_pieces = 3;
        let expected = &[Twist::None, Twist::None, Twist::None];
        assert_eq!(&coord_to_twist(coord, num_pieces), expected);

        let coord = 1;
        let num_pieces = 3;
        let expected = &[Twist::CW, Twist::None, Twist::None];
        assert_eq!(&coord_to_twist(coord, num_pieces), expected);

        let coord = 2;
        let num_pieces = 3;
        let expected = &[Twist::ACW, Twist::None, Twist::None];
        assert_eq!(&coord_to_twist(coord, num_pieces), expected);

        let coord = 3;
        let num_pieces = 3;
        let expected = &[Twist::None, Twist::CW, Twist::None];
        assert_eq!(&coord_to_twist(coord, num_pieces), expected);

        let coord = 6560;
        let num_pieces = 8;
        let expected = &[Twist::ACW; 8];
        assert_eq!(&coord_to_twist(coord, num_pieces), expected);
    }

    #[test]
    fn test_permutation_to_coord() {
        assert_eq!(permutation_to_coord(&[0,1,2]), 0);
        assert_eq!(permutation_to_coord(&[1,0,2]), 1);
        assert_eq!(permutation_to_coord(&[0,2,1]), 2);
        assert_eq!(permutation_to_coord(&[2,0,1]), 3);
        assert_eq!(permutation_to_coord(&[1,2,0]), 4);
        assert_eq!(permutation_to_coord(&[2,1,0]), 5);

        assert_eq!(permutation_to_coord(&[0,1,2,3,4,5,6,7]), 0);
        assert_eq!(permutation_to_coord(&[1,0,2,3,4,5,6,7]), 1);
        assert_eq!(permutation_to_coord(&[0,1,2,3,4,5,7,6]), 5_040);
        assert_eq!(permutation_to_coord(&[7,6,5,4,3,2,1,0]), 40_319);

        assert_eq!(permutation_to_coord(&[0,1,2,3,4,5,6,7,8,9,10,11]), 0);
        assert_eq!(permutation_to_coord(&[1,0,2,3,4,5,6,7,8,9,10,11]), 1);
        assert_eq!(permutation_to_coord(&[0,1,2,3,4,5,6,7,8,9,11,10]), 39_916_800);
        assert_eq!(permutation_to_coord(&[11,10,9,8,7,6,5,4,3,2,1,0]), 479_001_599);
    }

    #[test]
    fn test_coord_to_permutation() {
        let expected = &[0,1,2];
        assert_eq!(&coord_to_permutation(0, 3), expected);
        let expected = &[1,0,2];
        assert_eq!(&coord_to_permutation(1, 3), expected);
        let expected = &[0,2,1];
        assert_eq!(&coord_to_permutation(2, 3), expected);
        let expected = &[2,0,1];
        assert_eq!(&coord_to_permutation(3, 3), expected);
        let expected = &[1,2,0];
        assert_eq!(&coord_to_permutation(4, 3), expected);
        let expected = &[2,1,0];
        assert_eq!(&coord_to_permutation(5, 3), expected);

        let expected = &[0,1,2,3,4,5,6,7];
        assert_eq!(&coord_to_permutation(0, 8), expected);
        let expected = &[7,6,5,4,3,2,1,0];
        assert_eq!(&coord_to_permutation(40_319, 8), expected);

        let expected = &[0,1,2,3,4,5,6,7,8,9,10,11];
        assert_eq!(&coord_to_permutation(0, 12), expected);
        let expected = &[11,10,9,8,7,6,5,4,3,2,1,0];
        assert_eq!(&coord_to_permutation(479_001_599, 12), expected);
    }

    #[test]
    fn test_permutation_to_coord_even_parity() {
        assert_eq!(permutation_to_coord_even_parity(&[0,1,2]), 0);
        assert_eq!(permutation_to_coord_even_parity(&[1,0,2]), 0);
        assert_eq!(permutation_to_coord_even_parity(&[0,2,1]), 1);
        assert_eq!(permutation_to_coord_even_parity(&[2,0,1]), 1);
        assert_eq!(permutation_to_coord_even_parity(&[1,2,0]), 2);
        assert_eq!(permutation_to_coord_even_parity(&[2,1,0]), 2);

        assert_eq!(permutation_to_coord_even_parity(&[0,1,2,3,4,5,6,7]), 0);
        assert_eq!(permutation_to_coord_even_parity(&[1,0,2,3,4,5,6,7]), 0);
        assert_eq!(permutation_to_coord_even_parity(&[0,1,2,3,4,5,7,6]), 2_520);
        assert_eq!(permutation_to_coord_even_parity(&[7,6,5,4,3,2,1,0]), 20_159);

        assert_eq!(permutation_to_coord_even_parity(&[0,1,2,3,4,5,6,7,8,9,10,11]), 0);
        assert_eq!(permutation_to_coord_even_parity(&[1,0,2,3,4,5,6,7,8,9,10,11]), 0);
        assert_eq!(permutation_to_coord_even_parity(&[0,1,2,3,4,5,6,7,8,9,11,10]), 19_958_400);
        assert_eq!(permutation_to_coord_even_parity(&[11,10,9,8,7,6,5,4,3,2,1,0]), 239_500_799);
    }

    #[test]
    fn test_coord_to_permutation_even_parity() {
        let expected = &[0,1,2];
        assert_eq!(&coord_to_permutation_even_parity(0, 3), expected);
        let expected = &[2,0,1];
        assert_eq!(&coord_to_permutation_even_parity(1, 3), expected);
        let expected = &[1,2,0];
        assert_eq!(&coord_to_permutation_even_parity(2, 3), expected);

        let expected = &[0,1,2,3,4,5,6,7];
        assert_eq!(&coord_to_permutation_even_parity(0, 8), expected);
        let expected = &[7,6,5,4,3,2,1,0];
        assert_eq!(&coord_to_permutation_even_parity(20_159, 8), expected);

        let expected = &[0,1,2,3,4,5,6,7,8,9,10,11];
        assert_eq!(&coord_to_permutation_even_parity(0, 12), expected);
        let expected = &[11,10,9,8,7,6,5,4,3,2,1,0];
        assert_eq!(&coord_to_permutation_even_parity(239_500_799, 12), expected);
    }

    #[test]
    fn test_piece_distibution_to_coord() {
        assert_eq!(piece_distibution_to_coord(&[true, false, false]), 0);
        assert_eq!(piece_distibution_to_coord(&[false, true, false]), 1);
        assert_eq!(piece_distibution_to_coord(&[false, false, true]), 2);

        assert_eq!(piece_distibution_to_coord(&[true, true, false, false]), 0);
        assert_eq!(piece_distibution_to_coord(&[true, false, true, false]), 1);
        assert_eq!(piece_distibution_to_coord(&[true, false, false, true]), 2);
        assert_eq!(piece_distibution_to_coord(&[false, true, true, false]), 3);
        assert_eq!(piece_distibution_to_coord(&[false, true, false, true]), 4);
        assert_eq!(piece_distibution_to_coord(&[false, false, true, true]), 5);

        assert_eq!(piece_distibution_to_coord(
            &[true, true, true, true, false, false, false, false, false, false, false, false]
        ), 0);
        assert_eq!(piece_distibution_to_coord(
            &[false, false, false, false, false, false, false, false, true, true, true, true]
        ), 494);
    }

    #[test]
    fn test_coord_to_piece_distribution() {
        assert_eq!(coord_to_piece_distribution(0, 3, 1), &[true, false, false]);
        assert_eq!(coord_to_piece_distribution(1, 3, 1), &[false, true, false]);
        assert_eq!(coord_to_piece_distribution(2, 3, 1), &[false, false, true]);

        assert_eq!(coord_to_piece_distribution(0, 4, 2), &[true, true, false, false]);
        assert_eq!(coord_to_piece_distribution(1, 4, 2), &[true, false, true, false]);
        assert_eq!(coord_to_piece_distribution(2, 4, 2), &[true, false, false, true]);
        assert_eq!(coord_to_piece_distribution(3, 4, 2), &[false, true, true, false]);
        assert_eq!(coord_to_piece_distribution(4, 4, 2), &[false, true, false, true]);
        assert_eq!(coord_to_piece_distribution(5, 4, 2), &[false, false, true, true]);

        assert_eq!(coord_to_piece_distribution(0, 12, 4),
            &[true, true, true, true, false, false, false, false, false, false, false, false]
        );
        assert_eq!(coord_to_piece_distribution(494, 12, 4),
            &[false, false, false, false, false, false, false, false, true, true, true, true]
        );
    }

    #[test]
    fn test_is_even_parity() {
        assert_eq!(is_even_parity(&[0, 1, 2]), true);
        assert_eq!(is_even_parity(&[0, 2, 1]), false);
        assert_eq!(is_even_parity(&[0,1,2,3,4,5]), true);
        assert_eq!(is_even_parity(&[5,4,3,2,1,0]), false);
        assert_eq!(is_even_parity(&[0,1,2,3,4,5,6,7]), true);
        assert_eq!(is_even_parity(&[0,1,2,3,4,5,7,6]), false);
        assert_eq!(is_even_parity(&[0,1,2,3,4,5,6,7,8,9,10,11]), true);
        assert_eq!(is_even_parity(&[11,10,9,8,7,6,5,4,3,2,1,0]), true);
    }

}