use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Query, Res, ResMut, Transform, With};

use crate::board::board_check_block_position;
use crate::common_component::{ActiveBlock, GameData, DOT_SIZE, MAX_LOCK_RESETS};
use crate::tetromino;

pub(crate) fn block_movement_system(
    mut query: Query<(&tetromino::Block, &mut Transform), With<ActiveBlock>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
) {
    for (block, mut transform) in query.iter_mut() {
        let mut transform_x: f32 = 0.0;

        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            transform_x = -DOT_SIZE;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            transform_x = DOT_SIZE;
        }

        if transform_x == 0.0 {
            continue;
        }

        let in_board = board_check_block_position(
            &game_data.board_matrix,
            transform.translation.x + transform_x,
            transform.translation.y,
            block,
        );

        if in_board {
            transform.translation.x += transform_x;

            // Reset lock delay on successful move (up to 15 resets)
            if game_data.lock_delay_active {
                game_data.lock_move_count += 1;
                if game_data.lock_move_count < MAX_LOCK_RESETS {
                    game_data.lock_delay_timer.reset();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::board_check_block_position;
    use crate::common_component::{BOARD_COLS, BOARD_ROWS, DOT_SIZE};
    use crate::tetromino::Block;

    #[test]
    fn move_left_blocked_at_wall() {
        let board = [[0i8; BOARD_COLS]; BOARD_ROWS];
        let block = Block::new_t();
        // T-piece at left wall (spawn_x = -37.5)
        // Moving left by DOT_SIZE should fail
        let at_left_wall_x = -112.5; // leftmost valid x for T-piece
        assert!(!board_check_block_position(
            &board,
            at_left_wall_x - DOT_SIZE,
            200.0,
            &block
        ));
    }

    #[test]
    fn move_right_blocked_at_wall() {
        let board = [[0i8; BOARD_COLS]; BOARD_ROWS];
        let block = Block::new_t();
        // T-piece at right wall
        let at_right_wall_x = 87.5; // rightmost valid x for T-piece
        assert!(!board_check_block_position(
            &board,
            at_right_wall_x + DOT_SIZE,
            200.0,
            &block
        ));
    }

    #[test]
    fn move_blocked_by_placed_piece() {
        let mut board = [[0i8; BOARD_COLS]; BOARD_ROWS];
        // Fill column 5 entirely
        for row in 0..BOARD_ROWS {
            board[row][5] = 1;
        }
        let block = Block::new_o();
        // O-piece dots at (1,0),(2,0),(1,1),(2,1) relative to base
        // At x=-37.5, base board_x=3, so dots at columns 4,5 — column 5 is filled
        assert!(!board_check_block_position(&board, -37.5, 200.0, &block));
    }
}
