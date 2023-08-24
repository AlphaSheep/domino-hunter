use crate::turndef::Turn;

/*
Piece can be one of a corner, edge or centre. A piece is represented by an index in a specific order.
Corners have the order UBL, UFL UFR, UBR, DBL, DFL, DFR, DBR.
Edges have the order UB, UL, UF, UR, BL, FL, FR, BR, DB, DL, DF, DR
Centers have the order U, L, F, R, B, D
Pieces represent both a piece and a position interchangeably.
*/

pub trait PieceState {
    fn get_state(&self) -> &Self;
}

pub trait PiecePosition {
    fn as_index(&self) -> usize;
}

impl PiecePosition for usize {
    fn as_index(&self) -> usize {
        *self
    }
}

 #[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corner {
    UBL = 0,
    UFL = 1,
    UFR = 2,
    UBR = 3,
    DBL = 4,
    DFL = 5,
    DFR = 6,
    DBR = 7,
}

impl PiecePosition for Corner {
    fn as_index(&self) -> usize {
        *self as usize
    }
}

impl PieceState for Corner {
    fn get_state(&self) -> &Self {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    UB = 0,
    UL = 1,
    UF = 2,
    UR = 3,
    BL = 4,
    FL = 5,
    FR = 6,
    BR = 7,
    DB = 8,
    DL = 9,
    DF = 10,
    DR = 11,
}

impl PiecePosition for Edge {
    fn as_index(&self) -> usize {
        *self as usize
    }
}

impl PieceState for Edge {
    fn get_state(&self) -> &Self {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Centre {
    U = 0,
    L = 1,
    F = 2,
    R = 3,
    B = 4,
    D = 5,
}

impl PiecePosition for Centre {
    fn as_index(&self) -> usize {
        *self as usize
    }
}

impl PieceState for Centre {
    fn get_state(&self) -> &Self {
        self
    }
}

/*
Flips indicate the orientation of an edge. We define an edge's orientation as "good" relative to a particular
axis if it can be moved into the solved position with no quarter turns about that axis, and "bad" otherwise.
We arbitrarily choose to use the FB axis as the axis of reference.
*/

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flip {
    Good = 0,
    Bad = 1,
}

impl PieceState for Flip {
    fn get_state(&self) -> &Self {
        self
    }
}

impl Into<Flip> for usize {
    fn into(self) -> Flip {
        match self % 2 {
            0 => Flip::Good,
            1 => Flip::Bad,
            _ => panic!("Impossible flip value: {}", self),
        }
    }
}

impl Flip {
    pub fn flip(&self) -> Flip {
        match self {
            Flip::Good => Flip::Bad,
            Flip::Bad => Flip::Good,
        }
    }
}

/*
Twists indicate the orientation of a corner. We define a corner's orientation as "good" relative to a particular
axis if it can be moved into the solved position using half turns about any axis, and quarter turns about the
axis of reference only. We define the orientation as "clockwise" if it requires a quarter turn about another axis
to twist the corner clockwise into a good state, and "anticlockwise" otherwise.
We arbitrarily choose to use the UD axis as the axis of reference.
*/

 #[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Twist {
    None = 0,
    CW = 1,
    ACW = 2,
}

impl PieceState for Twist {
    fn get_state(&self) -> &Self {
        self
    }
}

impl Into<Twist> for usize {
    fn into(self) -> Twist {
        match self % 3 {
            0 => Twist::None,
            1 => Twist::CW,
            2 => Twist::ACW,
            _ => panic!("Impossible twist value: {}", self),
        }
    }
}

impl Twist {
    pub fn twist_by(&self, amount: &Twist) -> Self {
        let current_twist = *self.get_state() as usize;
        let twist_amount = *amount.get_state() as usize;
        (current_twist + twist_amount).into()
    }
}

/*
There are 3 base transformations that can be applied the state of the cube:
    Swapping two pieces of the same type
    Flipping an edge
    Twisting a corner
A move is a collection of these effects.
*/

pub type Swap<P: PiecePosition> = (P, P); // Positions to be swapped

pub type FlipEdge = Edge; // Position of edge to be flipped

pub type TwistCorner = (Corner, Twist); // Position of corner to be twisted, and a twist amount

/*
The state of the cube is represented by a set of lists of each of the above types.
The lists are ordered in the same order as the pieces are defined.
*/

#[derive(Clone, Debug)]
pub struct StateList<S: PieceState> {
    states: Vec<S>,
}

impl<S: PieceState + Copy> StateList<S> {
    pub fn new(states: Vec<S>) -> Self {
        StateList { states }
    }

    pub fn get<P: PiecePosition>(&self, position: &P) -> &S {
        &self.states[position.as_index()]
    }

    pub fn set<P: PiecePosition>(&mut self, position: &P, state: &S) {
        self.states[position.as_index()] = *state;
    }

    pub fn as_slice(&self) -> &[S] {
        &self.states
    }

    fn apply_swap<P: PiecePosition>(&mut self, swap: &Swap<P>) {
        let first_position = swap.0.as_index();
        let second_position = swap.1.as_index();

        let temp = self.states[first_position];

        self.states[first_position] = self.states[second_position];
        self.states[second_position] = temp;
    }

    pub fn apply_swaps<P: PiecePosition>(&mut self, swaps: &[Swap<P>]) {
        for swap in swaps {
            self.apply_swap(swap);
        }
    }
}

impl StateList<Flip> {
    fn apply_flip(&mut self, flip: FlipEdge) {
        let position = flip;
        let new_state = self.get(&position).flip();

        self.set(&position, &new_state);
    }

    pub fn apply_flips(&mut self, flips: &[FlipEdge]) {
        for flip in flips {
            self.apply_flip(*flip);
        }
    }
}

impl StateList<Twist> {
    fn apply_twist(&mut self, twist: TwistCorner) {
        let position = twist.0;
        let twist_amount = twist.1;
        let new_state = self.get(&position).twist_by(&twist_amount);

        self.set(&position, &new_state);
    }

    pub fn apply_twists(&mut self, twists: &[TwistCorner]) {
        for twist in twists {
            self.apply_twist(*twist);
        }
    }
}

#[derive(Clone, Debug)]
pub struct RawState {
    pub corners: StateList<Corner>,
    pub twists: StateList<Twist>,
    pub edges: StateList<Edge>,
    pub flips: StateList<Flip>,
    pub centers: StateList<Centre>,
}

impl RawState {
    pub fn solved() -> Self {
        RawState {
            corners: StateList { states: vec![Corner::UBL, Corner::UFL, Corner::UFR, Corner::UBR, Corner::DBL, Corner::DFL, Corner::DFR, Corner::DBR] },
            twists: StateList { states: vec![Twist::None, Twist::None, Twist::None, Twist::None, Twist::None, Twist::None, Twist::None, Twist::None] },
            edges: StateList { states: vec![Edge::UB, Edge::UL, Edge::UF, Edge::UR, Edge::BL, Edge::FL, Edge::FR, Edge::BR, Edge::DB, Edge::DL, Edge::DF, Edge::DR] },
            flips: StateList { states: vec![Flip::Good, Flip::Good, Flip::Good, Flip::Good, Flip::Good, Flip::Good, Flip::Good, Flip::Good, Flip::Good, Flip::Good, Flip::Good, Flip::Good] },
            centers: StateList { states: vec![Centre::U, Centre::L, Centre::F, Centre::R, Centre::B, Centre::D] },
        }
    }
}

/*
A move is a collection of transformations
*/

pub struct TurnEffect {
    corner_swaps: &'static [Swap<Corner>],
    corner_twists: &'static [TwistCorner],
    edge_swaps: &'static [Swap<Edge>],
    edge_flips: &'static [FlipEdge],
    center_swaps: &'static [Swap<Centre>],
}

impl TurnEffect {
    pub fn apply(&self, state: &mut RawState) {
        self.apply_corners_only(state);
        self.apply_twists_only(state);
        self.apply_edges_only(state);
        self.apply_flips_only(state);
        self.apply_centers_only(state);
    }

    pub fn apply_corners_only(&self, state: &mut RawState) {
        state.corners.apply_swaps(self.corner_swaps);
    }

    pub fn apply_twists_only(&self, state: &mut RawState) {
        state.twists.apply_swaps(self.corner_swaps);
        state.twists.apply_twists(self.corner_twists);
    }

    pub fn apply_edges_only(&self, state: &mut RawState) {
        state.edges.apply_swaps(self.edge_swaps);
    }

    pub fn apply_flips_only(&self, state: &mut RawState) {
        state.flips.apply_swaps(self.edge_swaps);
        state.flips.apply_flips(self.edge_flips);
    }

    pub fn apply_centers_only(&self, state: &mut RawState) {
        state.centers.apply_swaps(self.center_swaps);
    }

    pub fn apply_to_corners_statelist(&self, state: &mut StateList<Corner>) {
        state.apply_swaps(self.corner_swaps);
    }

    pub fn apply_to_twists_statelist(&self, state: &mut StateList<Twist>) {
        state.apply_swaps(self.corner_swaps);
        state.apply_twists(self.corner_twists);
    }

    pub fn apply_to_edges_statelist(&self, state: &mut StateList<Edge>) {
        state.apply_swaps(self.edge_swaps);
    }

    pub fn apply_to_flips_statelist(&self, state: &mut StateList<Flip>) {
        state.apply_swaps(self.edge_swaps);
        state.apply_flips(self.edge_flips);
    }

    pub fn apply_to_centers_statelist(&self, state: &mut StateList<Centre>) {
        state.apply_swaps(self.center_swaps);
    }
}

/*
We describe the effects of a single quarter turn on each layer. Moves can be described as a turning a
a combination of layers up to 3 times.
*/

const RIGHT_LAYER_EFFECT: TurnEffect = TurnEffect {
    corner_swaps: &[ (Corner::UFR, Corner::DFR), (Corner::DFR, Corner::DBR), (Corner::DBR, Corner::UBR) ] ,
    edge_swaps: &[ (Edge::UR, Edge::FR), (Edge::FR, Edge::DR), (Edge::DR, Edge::BR) ] ,
    center_swaps: &[] ,
    corner_twists: &[ (Corner::UBR, Twist::CW), (Corner::DBR, Twist::ACW), (Corner::DFR, Twist::CW), (Corner::UFR, Twist::ACW) ] ,
    edge_flips: &[] ,
};

const MIDDLE_LAYER_EFFECT: TurnEffect = TurnEffect {
    corner_swaps: &[] ,
    edge_swaps: &[ (Edge::UF, Edge::DF), (Edge::DF, Edge::DB), (Edge::DB, Edge::UB) ] ,
    center_swaps: &[ (Centre::U, Centre::F), (Centre::F, Centre::D), (Centre::D, Centre::B) ] ,
    corner_twists: &[] ,
    edge_flips: &[Edge::UF, Edge::DF, Edge::DB, Edge::UB] ,
};

const LEFT_LAYER_EFFECT: TurnEffect = TurnEffect {
    corner_swaps: &[ (Corner::UFL, Corner::DFL), (Corner::DFL, Corner::DBL), (Corner::DBL, Corner::UBL) ] ,
    edge_swaps: &[ (Edge::UL, Edge::FL), (Edge::FL, Edge::DL), (Edge::DL, Edge::BL) ] ,
    center_swaps: &[] ,
    corner_twists: &[ (Corner::UBL, Twist::ACW), (Corner::DBL, Twist::CW), (Corner::DFL, Twist::ACW), (Corner::UFL, Twist::CW) ] ,
    edge_flips: &[] ,
};

const UP_LAYER_EFFECT: TurnEffect = TurnEffect {
    corner_swaps: &[ (Corner::UBL, Corner::UFL), (Corner::UFL, Corner::UFR), (Corner::UFR, Corner::UBR) ] ,
    edge_swaps: &[ (Edge::UB, Edge::UL), (Edge::UL, Edge::UF), (Edge::UF, Edge::UR) ] ,
    center_swaps: &[] ,
    corner_twists: &[] ,
    edge_flips: &[] ,
};

const EQUATOR_LAYER_EFFECT: TurnEffect = TurnEffect {
    corner_swaps: &[] ,
    edge_swaps: &[ (Edge::BL, Edge::FL), (Edge::FL, Edge::FR), (Edge::FR, Edge::BR) ] ,
    center_swaps: &[ (Centre::L, Centre::F), (Centre::F, Centre::R), (Centre::R, Centre::B) ] ,
    corner_twists: &[] ,
    edge_flips: &[Edge::BL, Edge::FL, Edge::FR, Edge::BR] ,
};

const DOWN_LAYER_EFFECT: TurnEffect = TurnEffect {
    corner_swaps: &[ (Corner::DBL, Corner::DFL), (Corner::DFL, Corner::DFR), (Corner::DFR, Corner::DBR) ] ,
    edge_swaps: &[ (Edge::DB, Edge::DL), (Edge::DL, Edge::DF), (Edge::DF, Edge::DR) ] ,
    center_swaps: &[] ,
    corner_twists: &[] ,
    edge_flips: &[] ,
};

const FRONT_LAYER_EFFECT: TurnEffect = TurnEffect {
    corner_swaps: &[ (Corner::UFL, Corner::DFL), (Corner::DFL, Corner::DFR), (Corner::DFR, Corner::UFR) ] ,
    edge_swaps: &[ (Edge::UF, Edge::FL), (Edge::FL, Edge::DF), (Edge::DF, Edge::FR) ] ,
    center_swaps: &[] ,
    corner_twists: &[ (Corner::UFL, Twist::ACW), (Corner::DFL, Twist::CW), (Corner::DFR, Twist::ACW), (Corner::UFR, Twist::CW) ] ,
    edge_flips: &[Edge::UF, Edge::FL, Edge::DF, Edge::FR] ,
};

const SLICE_LAYER_EFFECT: TurnEffect = TurnEffect {
    corner_swaps: &[] ,
    edge_swaps: &[ (Edge::UL, Edge::DL), (Edge::DL, Edge::DR), (Edge::DR, Edge::UR) ] ,
    center_swaps: &[ (Centre::U, Centre::L), (Centre::L, Centre::D), (Centre::D, Centre::R) ] ,
    corner_twists: &[] ,
    edge_flips: &[Edge::UL, Edge::DL, Edge::DR, Edge::UR] ,
};

const BACK_LAYER_EFFECT: TurnEffect = TurnEffect {
    corner_swaps: &[ (Corner::UBL, Corner::DBL), (Corner::DBL, Corner::DBR), (Corner::DBR, Corner::UBR) ] ,
    edge_swaps: &[ (Edge::UB, Edge::BL), (Edge::BL, Edge::DB), (Edge::DB, Edge::BR) ] ,
    center_swaps: &[] ,
    corner_twists: &[ (Corner::UBL, Twist::CW), (Corner::DBL, Twist::ACW), (Corner::DBR, Twist::CW), (Corner::UBR, Twist::ACW) ] ,
    edge_flips: &[] ,
};

impl TurnEffect {
    pub fn from_turn(turn: &Turn) -> Self {
        match turn {
            &Turn::RIGHT => RIGHT_LAYER_EFFECT,
            &Turn::MIDDLE => MIDDLE_LAYER_EFFECT,
            &Turn::LEFT => LEFT_LAYER_EFFECT,
            &Turn::UP => UP_LAYER_EFFECT,
            &Turn::EQUATOR => EQUATOR_LAYER_EFFECT,
            &Turn::DOWN => DOWN_LAYER_EFFECT,
            &Turn::FRONT => FRONT_LAYER_EFFECT,
            &Turn::SLICE => SLICE_LAYER_EFFECT,
            &Turn::BACK => BACK_LAYER_EFFECT,
            _ => panic!("Raw move effects are only available for base moves of a single layer: {:?} ({:?}) is not supported. For compound moves, use move tables.", turn, turn.to_name()),
        }
    }
}