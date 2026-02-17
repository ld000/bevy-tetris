use bevy::{color::Color, prelude::Component};

#[derive(Clone, Copy, Default, Debug)]
pub struct Dot {
    pub x: i8,
    pub y: i8,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum State {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Component, Clone)]
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

    pub fn reset_rotation(&mut self) {
        self.set_state(State::Zero);
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_blocks() -> Vec<Block> {
        vec![
            Block::new_i(),
            Block::new_o(),
            Block::new_t(),
            Block::new_s(),
            Block::new_z(),
            Block::new_j(),
            Block::new_l(),
        ]
    }

    #[test]
    fn initial_state_is_zero() {
        for block in all_blocks() {
            assert_eq!(*block.state(), State::Zero);
        }
    }

    #[test]
    fn rotate_right_cycles() {
        let mut block = Block::new_t();
        assert_eq!(*block.state(), State::Zero);
        Rotation::rotate_right(&mut block);
        assert_eq!(*block.state(), State::One);
        Rotation::rotate_right(&mut block);
        assert_eq!(*block.state(), State::Two);
        Rotation::rotate_right(&mut block);
        assert_eq!(*block.state(), State::Three);
        Rotation::rotate_right(&mut block);
        assert_eq!(*block.state(), State::Zero);
    }

    #[test]
    fn rotate_left_cycles() {
        let mut block = Block::new_t();
        assert_eq!(*block.state(), State::Zero);
        Rotation::rotate_left(&mut block);
        assert_eq!(*block.state(), State::Three);
        Rotation::rotate_left(&mut block);
        assert_eq!(*block.state(), State::Two);
        Rotation::rotate_left(&mut block);
        assert_eq!(*block.state(), State::One);
        Rotation::rotate_left(&mut block);
        assert_eq!(*block.state(), State::Zero);
    }

    #[test]
    fn dots_by_state_returns_4_dots() {
        for block in all_blocks() {
            assert_eq!(block.dots_by_state().len(), 4);
        }
    }

    #[test]
    fn o_piece_same_dots_all_states() {
        let mut block = Block::new_o();
        let dots_zero = block.dots_by_state();
        Rotation::rotate_right(&mut block);
        let dots_one = block.dots_by_state();
        Rotation::rotate_right(&mut block);
        let dots_two = block.dots_by_state();
        Rotation::rotate_right(&mut block);
        let dots_three = block.dots_by_state();

        for i in 0..4 {
            assert_eq!(dots_zero[i].x, dots_one[i].x);
            assert_eq!(dots_zero[i].y, dots_one[i].y);
            assert_eq!(dots_zero[i].x, dots_two[i].x);
            assert_eq!(dots_zero[i].y, dots_two[i].y);
            assert_eq!(dots_zero[i].x, dots_three[i].x);
            assert_eq!(dots_zero[i].y, dots_three[i].y);
        }
    }

    #[test]
    fn reset_rotation_returns_to_zero() {
        let mut block = Block::new_s();
        Rotation::rotate_right(&mut block);
        Rotation::rotate_right(&mut block);
        assert_eq!(*block.state(), State::Two);
        block.reset_rotation();
        assert_eq!(*block.state(), State::Zero);
    }
}
