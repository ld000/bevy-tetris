use bevy::{color::Color, prelude::Component};

#[derive(Clone, Copy, Default, Debug)]
pub struct Dot {
    pub x: i8,
    pub y: i8,
}

#[derive(PartialEq, Clone, Copy)]
pub enum State {
    Zero,
    One,
    Two,
    Three,
    None,
}

#[derive(Component)]
pub enum Block {
    I {
        dots: [[Dot; 4]; 4],
        state: State,
        color: Color,
    },
    O {
        dots: [[Dot; 4]; 4],
        state: State,
        color: Color,
    },
    T {
        dots: [[Dot; 4]; 4],
        state: State,
        color: Color,
    },
    S {
        dots: [[Dot; 4]; 4],
        state: State,
        color: Color,
    },
    Z {
        dots: [[Dot; 4]; 4],
        state: State,
        color: Color,
    },
    J {
        dots: [[Dot; 4]; 4],
        state: State,
        color: Color,
    },
    L {
        dots: [[Dot; 4]; 4],
        state: State,
        color: Color,
    },
}

impl Block {
    pub fn state(&self) -> &State {
        match self {
            Self::I { state, .. } => state,
            Self::O { state, .. } => state,
            Self::T { state, .. } => state,
            Self::S { state, .. } => state,
            Self::Z { state, .. } => state,
            Self::J { state, .. } => state,
            Self::L { state, .. } => state,
        }
    }

    fn dots(&self) -> [[Dot; 4]; 4] {
        match self {
            Self::I { dots, .. } => *dots,
            Self::O { dots, .. } => *dots,
            Self::T { dots, .. } => *dots,
            Self::S { dots, .. } => *dots,
            Self::Z { dots, .. } => *dots,
            Self::J { dots, .. } => *dots,
            Self::L { dots, .. } => *dots,
        }
    }

    pub fn set_state(&mut self, state: State) {
        match self {
            Self::I { state: s, .. } => *s = state,
            Self::O { state: s, .. } => *s = state,
            Self::T { state: s, .. } => *s = state,
            Self::S { state: s, .. } => *s = state,
            Self::Z { state: s, .. } => *s = state,
            Self::J { state: s, .. } => *s = state,
            Self::L { state: s, .. } => *s = state,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::I { color, .. } => *color,
            Self::O { color, .. } => *color,
            Self::T { color, .. } => *color,
            Self::S { color, .. } => *color,
            Self::Z { color, .. } => *color,
            Self::J { color, .. } => *color,
            Self::L { color, .. } => *color,
        }
    }

    pub fn dots_by_state(&self) -> [Dot; 4] {
        match self.state() {
            State::Zero => self.dots()[0],
            State::One => self.dots()[1],
            State::Two => self.dots()[2],
            State::Three => self.dots()[3],
            State::None => unreachable!("State::None should never be used for getting dots"),
        }
    }

    pub fn new_i() -> Self {
        Self::I {
            dots: [
                [
                    Dot { x: 0, y: 1 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                    Dot { x: 3, y: 1 },
                ],
                [
                    Dot { x: 2, y: 0 },
                    Dot { x: 2, y: 1 },
                    Dot { x: 2, y: 2 },
                    Dot { x: 2, y: 3 },
                ],
                [
                    Dot { x: 0, y: 2 },
                    Dot { x: 1, y: 2 },
                    Dot { x: 2, y: 2 },
                    Dot { x: 3, y: 2 },
                ],
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 1, y: 2 },
                    Dot { x: 1, y: 3 },
                ],
            ],
            state: State::Zero,
            color: Color::srgb(0.0, 1.0, 1.0),
        }
    }

    pub fn new_o() -> Self {
        Self::O {
            dots: [
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 2, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                ],
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 2, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                ],
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 2, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                ],
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 2, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                ],
            ],
            state: State::Zero,
            color: Color::srgb(1.0, 1.0, 0.0),
        }
    }

    pub fn new_t() -> Self {
        Self::T {
            dots: [
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 0, y: 1 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                ],
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                    Dot { x: 1, y: 2 },
                ],
                [
                    Dot { x: 0, y: 1 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                    Dot { x: 1, y: 2 },
                ],
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 0, y: 1 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 1, y: 2 },
                ],
            ],
            state: State::Zero,
            color: Color::srgb(153.0 / 255.0, 0.0, 1.0),
        }
    }

    pub fn new_s() -> Self {
        Self::S {
            dots: [
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 2, y: 0 },
                    Dot { x: 0, y: 1 },
                    Dot { x: 1, y: 1 },
                ],
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                    Dot { x: 2, y: 2 },
                ],
                [
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                    Dot { x: 0, y: 2 },
                    Dot { x: 1, y: 2 },
                ],
                [
                    Dot { x: 0, y: 0 },
                    Dot { x: 0, y: 1 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 1, y: 2 },
                ],
            ],
            state: State::Zero,
            color: Color::srgb(0.0, 1.0, 0.0),
        }
    }

    pub fn new_z() -> Self {
        Self::Z {
            dots: [
                [
                    Dot { x: 0, y: 0 },
                    Dot { x: 1, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                ],
                [
                    Dot { x: 2, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                    Dot { x: 1, y: 2 },
                ],
                [
                    Dot { x: 0, y: 1 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 1, y: 2 },
                    Dot { x: 2, y: 2 },
                ],
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 0, y: 1 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 0, y: 2 },
                ],
            ],
            state: State::Zero,
            color: Color::srgb(1.0, 0.0, 0.0),
        }
    }

    pub fn new_j() -> Self {
        Self::J {
            dots: [
                [
                    Dot { x: 0, y: 0 },
                    Dot { x: 0, y: 1 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                ],
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 2, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 1, y: 2 },
                ],
                [
                    Dot { x: 0, y: 1 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                    Dot { x: 2, y: 2 },
                ],
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 0, y: 2 },
                    Dot { x: 1, y: 2 },
                ],
            ],
            state: State::Zero,
            color: Color::srgb(0.0, 0.0, 1.0),
        }
    }

    pub fn new_l() -> Self {
        Self::L {
            dots: [
                [
                    Dot { x: 2, y: 0 },
                    Dot { x: 0, y: 1 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                ],
                [
                    Dot { x: 1, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 1, y: 2 },
                    Dot { x: 2, y: 2 },
                ],
                [
                    Dot { x: 0, y: 1 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 2, y: 1 },
                    Dot { x: 0, y: 2 },
                ],
                [
                    Dot { x: 0, y: 0 },
                    Dot { x: 1, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: 1, y: 2 },
                ],
            ],
            state: State::Zero,
            color: Color::srgb(1.0, 170.0 / 255.0, 0.0),
        }
    }
}

#[derive(Component)]
pub struct Rotation;

impl Rotation {
    pub fn rotate_right(tetromino: &mut Block) -> (State, State) {
        match tetromino.state() {
            State::Zero => {
                tetromino.set_state(State::One);
                (State::Zero, State::One)
            }
            State::One => {
                tetromino.set_state(State::Two);
                (State::One, State::Two)
            }
            State::Two => {
                tetromino.set_state(State::Three);
                (State::Two, State::Three)
            }
            State::Three => {
                tetromino.set_state(State::Zero);
                (State::Three, State::Zero)
            }
            State::None => {
                tetromino.set_state(State::None);
                (State::None, State::None)
            },
        }
    }

    pub fn rotate_left(tetromino: &mut Block) -> (State, State) {
        match tetromino.state() {
            State::Zero => {
                tetromino.set_state(State::Three);
                (State::Zero, State::Three)
            }
            State::One => {
                tetromino.set_state(State::Zero);
                (State::One, State::Zero)
            }
            State::Two => {
                tetromino.set_state(State::One);
                (State::Two, State::One)
            }
            State::Three => {
                tetromino.set_state(State::Two);
                (State::Three, State::Two)
            }
            State::None => {
                tetromino.set_state(State::None);
                (State::None, State::None)
            },
        }
    }
}
