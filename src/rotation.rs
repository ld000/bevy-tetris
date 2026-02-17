use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{
    Children, Commands, Entity, KeyCode, Query, Res, ResMut, Transform, With,
};

use crate::board::board_check_block_position;
use crate::common_component::{ActiveDot, GameData};
use crate::tetromino;

/// Returns kick offset table for SRS (Super Rotation System)
/// Returns array of (x, y) offsets to try in order
pub(crate) fn get_kick_offsets(
    block: &tetromino::Block,
    from: tetromino::State,
    to: tetromino::State,
) -> Vec<(i8, i8)> {
    match block {
        tetromino::Block::I { .. } => {
            // I piece has special kick table
            match (from, to) {
                (tetromino::State::Zero, tetromino::State::One) => vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                (tetromino::State::One, tetromino::State::Zero) => vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                (tetromino::State::One, tetromino::State::Two) => vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                (tetromino::State::Two, tetromino::State::One) => vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                (tetromino::State::Two, tetromino::State::Three) => vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                (tetromino::State::Three, tetromino::State::Two) => vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                (tetromino::State::Three, tetromino::State::Zero) => vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                (tetromino::State::Zero, tetromino::State::Three) => vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                _ => vec![(0, 0)],
            }
        }
        tetromino::Block::O { .. } => {
            // O piece doesn't kick
            vec![(0, 0)]
        }
        _ => {
            // J, L, T, S, Z pieces use standard kick table
            match (from, to) {
                (tetromino::State::Zero, tetromino::State::One) => vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                (tetromino::State::One, tetromino::State::Zero) => vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                (tetromino::State::One, tetromino::State::Two) => vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                (tetromino::State::Two, tetromino::State::One) => vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                (tetromino::State::Two, tetromino::State::Three) => vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                (tetromino::State::Three, tetromino::State::Two) => vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                (tetromino::State::Three, tetromino::State::Zero) => vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                (tetromino::State::Zero, tetromino::State::Three) => vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                _ => vec![(0, 0)],
            }
        }
    }
}

pub(crate) fn block_rotation_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut block_query: Query<(Entity, &Children, &mut tetromino::Block, &Transform), With<tetromino::Rotation>>,
    mut game_data: ResMut<GameData>,
    mut child_query: Query<&mut Transform, (With<ActiveDot>, bevy::prelude::Without<tetromino::Rotation>)>,
) {
    for (entity, children, mut block, transform) in block_query.iter_mut() {
        let (from, to);
        let original_state = *block.state();

        if keyboard_input.just_pressed(KeyCode::KeyE) {
            (from, to) = tetromino::Rotation::rotate_right(&mut block);
        } else if keyboard_input.just_pressed(KeyCode::KeyQ) {
            (from, to) = tetromino::Rotation::rotate_left(&mut block);
        } else {
            continue;
        }

        // Try rotation with wall kicks
        let kick_offsets = get_kick_offsets(&block, from, to);
        let mut successful_kick: Option<(i8, i8)> = None;

        for (kick_x, kick_y) in kick_offsets {
            let test_x = transform.translation.x + (kick_x as f32 * 25.0);
            let test_y = transform.translation.y + (kick_y as f32 * 25.0);

            if board_check_block_position(&game_data.board_matrix, test_x, test_y, &block) {
                successful_kick = Some((kick_x, kick_y));
                break;
            }
        }

        if let Some((kick_x, kick_y)) = successful_kick {
            // Apply the kick offset to the transform
            commands.entity(entity).insert(Transform {
                translation: Vec3::new(
                    transform.translation.x + (kick_x as f32 * 25.0),
                    transform.translation.y + (kick_y as f32 * 25.0),
                    transform.translation.z,
                ),
                ..*transform
            });

            // Update child dot transforms in place (each block always has exactly 4 dots)
            let new_dots = block.dots_by_state();
            for (i, child) in children.iter().enumerate() {
                if let Ok(mut child_transform) = child_query.get_mut(*child) {
                    child_transform.translation = Vec3::new(
                        new_dots[i].x as f32 * 25.0,
                        -new_dots[i].y as f32 * 25.0,
                        0.0,
                    );
                }
            }

            // Reset lock delay on successful rotation (up to 15 resets)
            if game_data.lock_delay_active {
                game_data.lock_move_count += 1;
                if game_data.lock_move_count < 15 {
                    game_data.lock_delay_timer.reset();
                }
            }
        } else {
            // Rotation failed, revert to original state
            block.set_state(original_state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tetromino::{Block, State};

    #[test]
    fn i_piece_has_5_kick_offsets() {
        let block = Block::new_i();
        let offsets = get_kick_offsets(&block, State::Zero, State::One);
        assert_eq!(offsets.len(), 5);
    }

    #[test]
    fn o_piece_has_only_identity_kick() {
        let block = Block::new_o();
        let offsets = get_kick_offsets(&block, State::Zero, State::One);
        assert_eq!(offsets, vec![(0, 0)]);
    }

    #[test]
    fn standard_piece_has_5_kick_offsets() {
        let block = Block::new_t();
        let offsets = get_kick_offsets(&block, State::Zero, State::One);
        assert_eq!(offsets.len(), 5);
    }

    #[test]
    fn kick_offsets_first_is_identity() {
        // First kick offset should always be (0,0) — try without offset first
        for block in [Block::new_i(), Block::new_t(), Block::new_s(), Block::new_z(), Block::new_j(), Block::new_l()] {
            let offsets = get_kick_offsets(&block, State::Zero, State::One);
            assert_eq!(offsets[0], (0, 0), "First kick offset should be (0,0)");
        }
    }
}
