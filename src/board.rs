use bevy::prelude::Component;

use crate::common_component::{BOARD_COLS, BOARD_ROWS, DOT_SIZE};
use crate::tetromino;

#[derive(Component, Debug)]
pub(crate) struct BoardDot {
    pub board_x: i8,
    pub board_y: i8,
}

pub(crate) fn get_object_position_in_board(x: f32, y: f32) -> (i8, i8) {
    let board_x: i8 = if x < 0.0 {
        (4.0 - (x.abs() / DOT_SIZE - 0.5)) as i8
    } else {
        (5.0 + (x / DOT_SIZE - 0.5)) as i8
    };

    let board_y: i8 = if y < 0.0 {
        (10.0 + (y.abs() / DOT_SIZE - 0.5)) as i8
    } else {
        (9.0 - (y / DOT_SIZE - 0.5)) as i8
    };

    (board_x, board_y)
}

pub(crate) fn get_dot_position_in_board(x: f32, y: f32, dot_x: i8, dot_y: i8) -> (i8, i8) {
    let (mut board_x, mut board_y) = get_object_position_in_board(x, y);

    board_x += dot_x;
    board_y += dot_y;

    (board_x, board_y)
}

/// check if the block is in the board
/// -----------------------------------
/// | 0, 0 | 0, 1 | 0, 2 | ... | 0, 9 |
/// | 1, 0 | 1, 1 | 1, 2 | ... | 1, 9 |
/// | 2, 0 | 2, 1 | 2, 2 | ... | 2, 9 |
/// | ...  | ...  | ...  | ... | ...  |
/// | 19,0 | 19,1 | 19,2 | ... | 19,9 |
/// -----------------------------------
pub(crate) fn board_check_block_position(
    board: &[[i8; BOARD_COLS]; BOARD_ROWS],
    x: f32,
    y: f32,
    block: &tetromino::Block,
) -> bool {
    let dots = block.dots_by_state();
    for dot in dots.iter() {
        let (board_x, board_y) = get_dot_position_in_board(x, y, dot.x, dot.y);

        if !(0..BOARD_COLS as i8).contains(&board_x) || !(0..BOARD_ROWS as i8).contains(&board_y) {
            return false;
        }

        if board[board_y as usize][board_x as usize] == 1 {
            return false;
        }
    }

    true
}

pub(crate) fn place_dot_on_board(board_x: i8, board_y: i8, board: &mut [[i8; BOARD_COLS]; BOARD_ROWS]) {
    board[board_y as usize][board_x as usize] = 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tetromino::Block;
    use crate::common_component::{BOARD_COLS, BOARD_ROWS};

    #[test]
    fn object_position_center() {
        // World origin (0,0) maps to board center
        let (bx, by) = get_object_position_in_board(0.0, 0.0);
        // x=0 → 5 + (0/25 - 0.5) = 4, y=0 → 9 - (0/25 - 0.5) = 9
        assert_eq!(bx, 4);
        assert_eq!(by, 9);
    }

    #[test]
    fn object_position_top_left() {
        // Top-left of board: x = -112.5, y = 237.5
        let (bx, by) = get_object_position_in_board(-112.5, 237.5);
        assert_eq!(bx, 0);
        assert_eq!(by, 0);
    }

    #[test]
    fn object_position_bottom_right() {
        // Bottom-right of board: x = 112.5, y = -237.5
        let (bx, by) = get_object_position_in_board(112.5, -237.5);
        assert_eq!(bx, 9);
        assert_eq!(by, 19);
    }

    #[test]
    fn dot_position_adds_offset() {
        let (bx, by) = get_dot_position_in_board(0.0, 0.0, 2, 3);
        let (base_x, base_y) = get_object_position_in_board(0.0, 0.0);
        assert_eq!(bx, base_x + 2);
        assert_eq!(by, base_y + 3);
    }

    #[test]
    fn check_block_position_empty_board_valid() {
        let board = [[0i8; BOARD_COLS]; BOARD_ROWS];
        let block = Block::new_t();
        // Default spawn position
        assert!(board_check_block_position(&board, -37.5, 237.5, &block));
    }

    #[test]
    fn check_block_position_out_of_bounds() {
        let board = [[0i8; BOARD_COLS]; BOARD_ROWS];
        let block = Block::new_i();
        // Way off the left side
        assert!(!board_check_block_position(&board, -500.0, 0.0, &block));
    }

    #[test]
    fn check_block_position_collision() {
        let mut board = [[0i8; BOARD_COLS]; BOARD_ROWS];
        // Fill bottom row
        board[19] = [1; BOARD_COLS];
        let block = Block::new_o();
        // Try to place at bottom — should collide
        assert!(!board_check_block_position(&board, -37.5, -237.5, &block));
    }

    #[test]
    fn place_dot_sets_cell() {
        let mut board = [[0i8; BOARD_COLS]; BOARD_ROWS];
        place_dot_on_board(3, 5, &mut board);
        assert_eq!(board[5][3], 1);
    }
}
