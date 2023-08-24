use std::ops;

/*
Moves are indicated by a 32-bit integer.
We number the bits from 0 being the least significant bit and 31 being the most significant bit.
Bits 0 and 1 are base turns, not related to any particular move.
    0b01 is a single generic move, 0b10 is a double move, and 0x11 is an inverse move.
Bits 2-7 are not used.
Bits 8-15 are used for the LR axis.
    Bits 8-9 are the right layer,
    Bits 10-11 are the middle layer,
    Bits 12-13 are the left layer.
    Bit 14 is a mirror bit, which mirrors the state across the LR axis.
Bits 16-23 are used for the UD axis.
    Bits 16-17 are the top layer,
    Bits 18-19 are the equator layer,
    Bits 20-21 are the bottom layer.
    Bit 22 is a mirror bit, which mirrors the state across the UD axis.
Bits 24-31 are used for the FB axis.
    Bits 24-25 are the front layer,
    Bits 26-27 are the inner layer,
    Bits 28-29 are the back layer.
    Bit 30 is a mirror bit, which mirrors the state across the FB axis.

This has some useful properties:
    Turns can be applied to a layer by bitshifting a base turn.
    The primary moves of a layer can be doubled or inverted by multiplying by the base turns.
    Double layer turns and cube rotations can be applied by adding turns on the same axis.
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Turn(u32);

impl ops::Mul<u32> for Turn {
    type Output = Turn;

    fn mul(self, rhs: u32) -> Turn {
        Turn(self.0 * rhs)
    }
}

impl ops::Add<Turn> for Turn {
    type Output = Turn;

    fn add(self, rhs: Turn) -> Turn {
        Turn(self.0 + rhs.0)
    }
}

impl ops::Shl<u32> for Turn {
    type Output = Turn;

    fn shl(self, rhs: u32) -> Turn {
        Turn(self.0 << rhs)
    }
}

impl ops::Shr<u32> for Turn {
    type Output = u32;

    fn shr(self, rhs: u32) -> u32 {
        self.0 >> rhs
    }
}

impl ops::BitXor<&Turn> for Turn {
    type Output = Turn;

    fn bitxor(self, rhs: &Turn) -> Turn {
        Turn(self.0 ^ rhs.0)
    }
}

const BASE_TURN: u32 = 1;
const DOUBLE_TURN: u32 = 2;
const INVERSE_TURN: u32 = 3;

const RIGHT_LAYER_SHIFT: u32 = 8;
const MIDDLE_LAYER_SHIFT: u32 = 10;
const LEFT_LAYER_SHIFT: u32 = 12;
const LR_MIRROR_SHIFT: u32 = 14;
const TOP_LAYER_SHIFT: u32 = 16;
const EQUATOR_LAYER_SHIFT: u32 = 18;
const BOTTOM_LAYER_SHIFT: u32 = 20;
const UD_MIRROR_SHIFT: u32 = 22;
const FRONT_LAYER_SHIFT: u32 = 24;
const INNER_LAYER_SHIFT: u32 = 26;
const BACK_LAYER_SHIFT: u32 = 28;
const FB_MIRROR_SHIFT: u32 = 30;

struct TurnNameMap {
    name: &'static str,
    turn: Turn,
}

impl TurnNameMap {
    pub fn new(name: &'static str, turn: Turn) -> Self {
        Self {
            name,
            turn,
        }
    }

    pub fn get_all_turn_name_maps() -> Vec<Self> {
        vec![
            Self::new("R", Turn::RIGHT),
            Self::new("R2", Turn::RIGHT * DOUBLE_TURN),
            Self::new("R'", Turn::RIGHT * INVERSE_TURN),
            Self::new("M'", Turn::MIDDLE),
            Self::new("M2", Turn::MIDDLE * DOUBLE_TURN),
            Self::new("M", Turn::MIDDLE * INVERSE_TURN),
            Self::new("L'", Turn::LEFT),
            Self::new("L2", Turn::LEFT * DOUBLE_TURN),
            Self::new("L", Turn::LEFT * INVERSE_TURN),
            Self::new("U", Turn::UP),
            Self::new("U2", Turn::UP * DOUBLE_TURN),
            Self::new("U'", Turn::UP * INVERSE_TURN),
            Self::new("E", Turn::EQUATOR),
            Self::new("E2", Turn::EQUATOR * DOUBLE_TURN),
            Self::new("E'", Turn::EQUATOR * INVERSE_TURN),
            Self::new("D'", Turn::DOWN),
            Self::new("D2", Turn::DOWN * DOUBLE_TURN),
            Self::new("D", Turn::DOWN * INVERSE_TURN),
            Self::new("F", Turn::FRONT),
            Self::new("F2", Turn::FRONT * DOUBLE_TURN),
            Self::new("F'", Turn::FRONT * INVERSE_TURN),
            Self::new("S", Turn::SLICE),
            Self::new("S2", Turn::SLICE * DOUBLE_TURN),
            Self::new("S'", Turn::SLICE * INVERSE_TURN),
            Self::new("B'", Turn::BACK),
            Self::new("B2", Turn::BACK * DOUBLE_TURN),
            Self::new("B", Turn::BACK * INVERSE_TURN),
            Self::new("x", Turn::RIGHT + Turn::MIDDLE + Turn::LEFT),
            Self::new("x2", (Turn::RIGHT + Turn::MIDDLE + Turn::LEFT) * DOUBLE_TURN),
            Self::new("x'", (Turn::RIGHT + Turn::MIDDLE + Turn::LEFT) * INVERSE_TURN),
            Self::new("y", Turn::UP + Turn::EQUATOR + Turn::DOWN),
            Self::new("y2", (Turn::UP + Turn::EQUATOR + Turn::DOWN) * DOUBLE_TURN),
            Self::new("y'", (Turn::UP + Turn::EQUATOR + Turn::DOWN) * INVERSE_TURN),
            Self::new("z", Turn::FRONT + Turn::SLICE + Turn::BACK),
            Self::new("z2", (Turn::FRONT + Turn::SLICE + Turn::BACK) * DOUBLE_TURN),
            Self::new("z'", (Turn::FRONT + Turn::SLICE + Turn::BACK) * INVERSE_TURN),
            Self::new("r", Turn::RIGHT + Turn::MIDDLE),
            Self::new("r2", (Turn::RIGHT + Turn::MIDDLE) * DOUBLE_TURN),
            Self::new("r'", (Turn::RIGHT + Turn::MIDDLE) * INVERSE_TURN),
            Self::new("l'", Turn::LEFT + Turn::MIDDLE),
            Self::new("l2", (Turn::LEFT + Turn::MIDDLE) * DOUBLE_TURN),
            Self::new("l", (Turn::LEFT + Turn::MIDDLE) * INVERSE_TURN),
            Self::new("u", Turn::UP + Turn::EQUATOR),
            Self::new("u2", (Turn::UP + Turn::EQUATOR) * DOUBLE_TURN),
            Self::new("u'", (Turn::UP + Turn::EQUATOR) * INVERSE_TURN),
            Self::new("d'", Turn::DOWN + Turn::EQUATOR),
            Self::new("d2", (Turn::DOWN + Turn::EQUATOR) * DOUBLE_TURN),
            Self::new("d", (Turn::DOWN + Turn::EQUATOR) * INVERSE_TURN),
            Self::new("f", Turn::FRONT + Turn::SLICE),
            Self::new("f2", (Turn::FRONT + Turn::SLICE) * DOUBLE_TURN),
            Self::new("f'", (Turn::FRONT + Turn::SLICE) * INVERSE_TURN),
            Self::new("b'", Turn::BACK + Turn::SLICE),
            Self::new("b2", (Turn::BACK + Turn::SLICE) * DOUBLE_TURN),
            Self::new("b", (Turn::BACK + Turn::SLICE) * INVERSE_TURN),
            Self::new("lr_mirror", Turn::LR_MIRROR),
            Self::new("ud_mirror", Turn::UD_MIRROR),
            Self::new("fb_mirror", Turn::FB_MIRROR),
        ]
    }
}

impl Into<u32> for &Turn {
    fn into(self) -> u32 {
        self.0
    }
}

impl Turn {
    pub const RIGHT: Turn = Turn(BASE_TURN << RIGHT_LAYER_SHIFT);
    pub const MIDDLE: Turn = Turn(BASE_TURN << MIDDLE_LAYER_SHIFT);
    pub const LEFT: Turn = Turn(BASE_TURN << LEFT_LAYER_SHIFT);
    pub const LR_MIRROR: Turn = Turn(BASE_TURN << LR_MIRROR_SHIFT);
    pub const UP: Turn = Turn(BASE_TURN << TOP_LAYER_SHIFT);
    pub const EQUATOR: Turn = Turn(BASE_TURN << EQUATOR_LAYER_SHIFT);
    pub const DOWN: Turn = Turn(BASE_TURN << BOTTOM_LAYER_SHIFT);
    pub const UD_MIRROR: Turn = Turn(BASE_TURN << UD_MIRROR_SHIFT);
    pub const FRONT: Turn = Turn(BASE_TURN << FRONT_LAYER_SHIFT);
    pub const SLICE: Turn = Turn(BASE_TURN << INNER_LAYER_SHIFT);
    pub const BACK: Turn = Turn(BASE_TURN << BACK_LAYER_SHIFT);
    pub const FB_MIRROR: Turn = Turn(BASE_TURN << FB_MIRROR_SHIFT);

    pub fn from_name(turn_name: &str) -> Self {
        for turn in TurnNameMap::get_all_turn_name_maps() {
            if turn.name == turn_name {
                return turn.turn;
            }
        }
        panic!("Invalid turn: {:?}", turn_name);

    }

    pub fn to_name(&self) -> String {
        for turn in TurnNameMap::get_all_turn_name_maps() {
            if turn.turn == *self {
                return turn.name.to_string();
            }
        }
        panic!("Invalid turn: {:?}", self);
    }

    pub fn to_base_turns(&self) -> Vec<Self> {
        let mut turns = Vec::new();
        let mut base = BASE_TURN << 30;
        let mut turn: u32 = self.into();
        while base > 0 {
            for _ in 0..(turn / base) {
                turns.push(Turn(base));
            }
            turn %= base;
            base >>= 2;
        }
        turns
    }

    pub fn get_all_turns() -> Vec<Self> {
        let mut turns = Vec::new();
        for turn in TurnNameMap::get_all_turn_name_maps() {
            turns.push(turn.turn);
        }
        turns
    }

    pub fn get_vec_from_alg_string(alg: &str) -> Vec<Self> {
        let mut turns = Vec::new();
        for part in alg.split(" ") {
            if part.len() == 0 {
                continue;
            }
            turns.push(Turn::from_name(part));
        }
        turns
    }

    pub fn get_base_outer_layer_turns() -> Vec<Self> {
        vec![
            Turn::RIGHT,
            Turn::LEFT,
            Turn::UP,
            Turn::DOWN,
            Turn::FRONT,
            Turn::BACK,
        ]
    }

    pub fn get_base_layer_turns() -> Vec<Self> {
        vec![
            Turn::RIGHT,
            Turn::MIDDLE,
            Turn::LEFT,
            Turn::UP,
            Turn::EQUATOR,
            Turn::DOWN,
            Turn::FRONT,
            Turn::SLICE,
            Turn::BACK,
        ]
    }

    pub fn is_base_move(&self) -> bool {
        for shift in (0..32).step_by(2) {
            if self.0 == BASE_TURN << shift {
                return true;
            }
        }
        false
    }

    pub fn get_outer_layer_turns() -> Vec<Self> {
        let mut turns = Vec::new();
        for layer in Self::get_base_outer_layer_turns() {
            for direction in [BASE_TURN, DOUBLE_TURN, INVERSE_TURN].iter() {
                turns.push(layer * *direction);
            }
        }
        turns
    }
}

pub trait TurnVec {
    fn to_base_turns(&self) -> Vec<Turn>;
    fn to_algorithm_string(&self) -> String;
}

impl TurnVec for Vec<Turn> {
    fn to_base_turns(&self) -> Vec<Turn> {
        let mut turns = Vec::new();
        for turn in self {
            turns.extend(turn.to_base_turns());
        }
        turns
    }

    fn to_algorithm_string(&self) -> String {
        let mut alg = String::new();
        for turn in self {
            alg.push_str(&turn.to_name());
            alg.push(' ');
        }
        alg.pop();
        alg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_base_turns() {
        assert_eq!(Turn::from_name("R").to_base_turns(),
            vec![Turn::RIGHT]
        );
        assert_eq!(Turn::from_name("R2").to_base_turns(),
            vec![Turn::RIGHT, Turn::RIGHT]
        );
        assert_eq!(Turn::from_name("R'").to_base_turns(),
            vec![Turn::RIGHT, Turn::RIGHT, Turn::RIGHT]
        );
        assert_eq!(Turn::from_name("M'").to_base_turns(),
            vec![Turn::MIDDLE]
        );
        assert_eq!(Turn::from_name("M2").to_base_turns(),
            vec![Turn::MIDDLE, Turn::MIDDLE]
        );
        assert_eq!(Turn::from_name("M").to_base_turns(),
            vec![Turn::MIDDLE, Turn::MIDDLE, Turn::MIDDLE]
        );
        assert_eq!(Turn::from_name("L'").to_base_turns(),
            vec![Turn::LEFT]
        );
        assert_eq!(Turn::from_name("L2").to_base_turns(),
            vec![Turn::LEFT, Turn::LEFT]
        );
        assert_eq!(Turn::from_name("L").to_base_turns(),
            vec![Turn::LEFT, Turn::LEFT, Turn::LEFT]
        );
        assert_eq!(Turn::from_name("U").to_base_turns(),
            vec![Turn::UP]
        );
        assert_eq!(Turn::from_name("U2").to_base_turns(),
            vec![Turn::UP, Turn::UP]
        );
        assert_eq!(Turn::from_name("U'").to_base_turns(),
            vec![Turn::UP, Turn::UP, Turn::UP]
        );
        assert_eq!(Turn::from_name("E").to_base_turns(),
            vec![Turn::EQUATOR]
        );
        assert_eq!(Turn::from_name("E2").to_base_turns(),
            vec![Turn::EQUATOR, Turn::EQUATOR]
        );
        assert_eq!(Turn::from_name("E'").to_base_turns(),
            vec![Turn::EQUATOR, Turn::EQUATOR, Turn::EQUATOR]
        );
        assert_eq!(Turn::from_name("D'").to_base_turns(),
            vec![Turn::DOWN]
        );
        assert_eq!(Turn::from_name("D2").to_base_turns(),
            vec![Turn::DOWN, Turn::DOWN]
        );
        assert_eq!(Turn::from_name("D").to_base_turns(),
            vec![Turn::DOWN, Turn::DOWN, Turn::DOWN]
        );
        assert_eq!(Turn::from_name("F").to_base_turns(),
            vec![Turn::FRONT]
        );
        assert_eq!(Turn::from_name("F2").to_base_turns(),
            vec![Turn::FRONT, Turn::FRONT]
        );
        assert_eq!(Turn::from_name("F'").to_base_turns(),
            vec![Turn::FRONT, Turn::FRONT, Turn::FRONT]
        );
        assert_eq!(Turn::from_name("S").to_base_turns(),
            vec![Turn::SLICE]
        );
        assert_eq!(Turn::from_name("S2").to_base_turns(),
            vec![Turn::SLICE, Turn::SLICE]
        );
        assert_eq!(Turn::from_name("S'").to_base_turns(),
            vec![Turn::SLICE, Turn::SLICE, Turn::SLICE]
        );
        assert_eq!(Turn::from_name("B'").to_base_turns(),
            vec![Turn::BACK]
        );
        assert_eq!(Turn::from_name("B2").to_base_turns(),
            vec![Turn::BACK, Turn::BACK]
        );
        assert_eq!(Turn::from_name("B").to_base_turns(),
            vec![Turn::BACK, Turn::BACK, Turn::BACK]
        );
        assert_eq!(Turn::from_name("x").to_base_turns(),
            vec![Turn::LEFT, Turn::MIDDLE, Turn::RIGHT]
        );
        assert_eq!(Turn::from_name("x2").to_base_turns(),
            vec![Turn::LEFT, Turn::LEFT, Turn::MIDDLE, Turn::MIDDLE, Turn::RIGHT, Turn::RIGHT]
        );
        assert_eq!(Turn::from_name("x'").to_base_turns(),
            vec![Turn::LEFT, Turn::LEFT, Turn::LEFT, Turn::MIDDLE, Turn::MIDDLE, Turn::MIDDLE, Turn::RIGHT, Turn::RIGHT, Turn::RIGHT]
        );
        assert_eq!(Turn::from_name("y").to_base_turns(),
            vec![Turn::DOWN, Turn::EQUATOR, Turn::UP]
        );
        assert_eq!(Turn::from_name("y2").to_base_turns(),
            vec![Turn::DOWN, Turn::DOWN, Turn::EQUATOR, Turn::EQUATOR, Turn::UP, Turn::UP]
        );
        assert_eq!(Turn::from_name("y'").to_base_turns(),
            vec![Turn::DOWN, Turn::DOWN, Turn::DOWN, Turn::EQUATOR, Turn::EQUATOR, Turn::EQUATOR, Turn::UP, Turn::UP, Turn::UP]
        );
        assert_eq!(Turn::from_name("z").to_base_turns(),
            vec![Turn::BACK, Turn::SLICE, Turn::FRONT]
        );
        assert_eq!(Turn::from_name("z2").to_base_turns(),
            vec![Turn::BACK, Turn::BACK, Turn::SLICE, Turn::SLICE, Turn::FRONT, Turn::FRONT]
        );
        assert_eq!(Turn::from_name("z'").to_base_turns(),
            vec![Turn::BACK, Turn::BACK, Turn::BACK, Turn::SLICE, Turn::SLICE, Turn::SLICE, Turn::FRONT, Turn::FRONT, Turn::FRONT]
        );
        assert_eq!(Turn::from_name("r").to_base_turns(),
            vec![Turn::MIDDLE, Turn::RIGHT]
        );
        assert_eq!(Turn::from_name("r2").to_base_turns(),
            vec![Turn::MIDDLE, Turn::MIDDLE, Turn::RIGHT, Turn::RIGHT]
        );
        assert_eq!(Turn::from_name("r'").to_base_turns(),
            vec![Turn::MIDDLE, Turn::MIDDLE, Turn::MIDDLE, Turn::RIGHT, Turn::RIGHT, Turn::RIGHT]
        );
        assert_eq!(Turn::from_name("l'").to_base_turns(),
            vec![Turn::LEFT, Turn::MIDDLE]
        );
        assert_eq!(Turn::from_name("l2").to_base_turns(),
            vec![Turn::LEFT, Turn::LEFT, Turn::MIDDLE, Turn::MIDDLE]
        );
        assert_eq!(Turn::from_name("l").to_base_turns(),
            vec![Turn::LEFT, Turn::LEFT, Turn::LEFT, Turn::MIDDLE, Turn::MIDDLE, Turn::MIDDLE]
        );
        assert_eq!(Turn::from_name("u").to_base_turns(),
            vec![Turn::EQUATOR, Turn::UP]
        );
        assert_eq!(Turn::from_name("u2").to_base_turns(),
            vec![Turn::EQUATOR, Turn::EQUATOR, Turn::UP, Turn::UP]
        );
        assert_eq!(Turn::from_name("u'").to_base_turns(),
            vec![Turn::EQUATOR, Turn::EQUATOR, Turn::EQUATOR, Turn::UP, Turn::UP, Turn::UP]
        );
        assert_eq!(Turn::from_name("d'").to_base_turns(),
            vec![Turn::DOWN, Turn::EQUATOR]
        );
        assert_eq!(Turn::from_name("d2").to_base_turns(),
            vec![Turn::DOWN, Turn::DOWN, Turn::EQUATOR, Turn::EQUATOR]
        );
        assert_eq!(Turn::from_name("d").to_base_turns(),
            vec![Turn::DOWN, Turn::DOWN, Turn::DOWN, Turn::EQUATOR, Turn::EQUATOR, Turn::EQUATOR]
        );
        assert_eq!(Turn::from_name("f").to_base_turns(),
            vec![Turn::SLICE, Turn::FRONT]
        );
        assert_eq!(Turn::from_name("f2").to_base_turns(),
            vec![Turn::SLICE, Turn::SLICE, Turn::FRONT, Turn::FRONT]
        );
        assert_eq!(Turn::from_name("f'").to_base_turns(),
            vec![Turn::SLICE, Turn::SLICE, Turn::SLICE, Turn::FRONT, Turn::FRONT, Turn::FRONT]
        );
        assert_eq!(Turn::from_name("b'").to_base_turns(),
            vec![Turn::BACK, Turn::SLICE]
        );
        assert_eq!(Turn::from_name("b2").to_base_turns(),
            vec![Turn::BACK, Turn::BACK, Turn::SLICE, Turn::SLICE]
        );
        assert_eq!(Turn::from_name("b").to_base_turns(),
            vec![Turn::BACK, Turn::BACK, Turn::BACK, Turn::SLICE, Turn::SLICE, Turn::SLICE]
        );
        assert_eq!(Turn::from_name("lr_mirror").to_base_turns(),
            vec![Turn::LR_MIRROR]
        );
        assert_eq!(Turn::from_name("ud_mirror").to_base_turns(),
            vec![Turn::UD_MIRROR]
        );
        assert_eq!(Turn::from_name("fb_mirror").to_base_turns(),
            vec![Turn::FB_MIRROR]
        );



    }
}