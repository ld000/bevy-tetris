use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Query, Res, ResMut, Transform, With};

use crate::board::board_check_block_position;
use crate::common_component::{ActiveBlock, GameData};
use crate::tetromino;

pub(crate) fn block_movement_system(
    mut query: Query<(&tetromino::Block, &mut Transform), With<ActiveBlock>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
) {
    for (block, mut transform) in query.iter_mut() {
        let mut transform_x: f32 = 0.0;

        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            transform_x = -25.0;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            transform_x = 25.0;
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
                if game_data.lock_move_count < 15 {
                    game_data.lock_delay_timer.reset();
                }
            }
        }
    }
}
