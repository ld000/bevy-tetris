use bevy::prelude::{
    BuildChildrenTransformExt, Children, Commands, Entity, GlobalTransform, KeyCode, Query, Res, ResMut, State, Transform,
    With,
};
use bevy::input::ButtonInput;
use bevy::prelude::NextState;
use bevy::time::Time;

use crate::board::{board_check_block_position, get_object_position_in_board, place_dot_on_board, BoardDot};
use crate::common_component::{ActiveBlock, ActiveDot, DropType, GameData, DOT_SIZE, GRAVITY_FLOOR};
use crate::tetromino;

pub(crate) fn gravity_seconds(level: u32) -> f32 {
    let l = level as f32;
    let base = (0.8 - ((l - 1.0) * 0.007)).max(0.0);
    base.powf(l - 1.0).max(GRAVITY_FLOOR)
}

pub(crate) fn block_drop_type_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<DropType>>,
    old_state: Res<State<DropType>>,
    mut game_data: ResMut<GameData>,
    query: Query<&Transform, With<ActiveBlock>>,
) {
    if old_state.get() == &DropType::Hard {
        return;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        state.set(DropType::Hard);
        game_data.hard_drop_timer.reset();

        // Capture starting y position for hard drop scoring
        if let Ok(transform) = query.get_single() {
            game_data.hard_drop_start_y = Some(transform.translation.y);
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) && old_state.get() != &DropType::Soft {
        state.set(DropType::Soft);
        game_data.soft_drop_timer.reset();
    }
    if keyboard_input.just_released(KeyCode::ArrowDown) && old_state.get() == &DropType::Soft {
        state.set(DropType::Normal);
        game_data.score += game_data.soft_drop_cells;
        game_data.soft_drop_cells = 0;
    }
}

pub(crate) fn block_drop_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Children, &tetromino::Block, &mut Transform), With<ActiveBlock>>,
    children_query: Query<&GlobalTransform, With<ActiveDot>>,
    time: Res<Time>,
    state: Res<State<DropType>>,
    mut game_data: ResMut<GameData>,
) {
    if query.is_empty() {
        return;
    }

    // If lock delay is active, tick it every frame (independent of drop timer)
    if game_data.lock_delay_active && state.get() != &DropType::Hard {
        // Check if the piece can still drop (rotation/movement may have opened space below)
        let (_entity, _children, block, transform) = query.single();
        let can_drop = board_check_block_position(
            &game_data.board_matrix,
            transform.translation.x,
            transform.translation.y - DOT_SIZE,
            block,
        );
        if can_drop {
            // Piece is no longer on the ground — cancel lock delay, resume normal drop
            game_data.lock_delay_active = false;
            game_data.lock_move_count = 0;
        } else {
            game_data.lock_delay_timer.tick(time.delta());
            if game_data.lock_delay_timer.finished() {
                let (entity, children, _block, transform) = query.single_mut();
                place_block_on_board(
                    &mut commands,
                    &mut game_data,
                    children_query,
                    entity,
                    children,
                    &transform,
                );
            }
            return;
        }
    }

    // Normal drop timer gating
    if state.get() == &DropType::Hard {
        let finished = game_data.hard_drop_timer.tick(time.delta()).finished();
        if !finished {
            return;
        }
    } else if state.get() == &DropType::Soft {
        let finished = game_data.soft_drop_timer.tick(time.delta()).finished();
        if !finished {
            return;
        }
    } else {
        let finished = game_data.drop_timer.tick(time.delta()).finished();
        if !finished {
            return;
        }
    }

    let (entity, children, block, mut transform) = query.single_mut();

    let can_drop = board_check_block_position(
        &game_data.board_matrix,
        transform.translation.x,
        transform.translation.y - DOT_SIZE,
        block,
    );

    if can_drop {
        transform.translation.y -= DOT_SIZE;
        if state.get() == &DropType::Soft {
            game_data.soft_drop_cells += 1;
        }
        game_data.lock_delay_active = false;
    } else if state.get() == &DropType::Hard {
        place_block_on_board(
            &mut commands,
            &mut game_data,
            children_query,
            entity,
            children,
            &transform,
        );
    } else {
        // Start lock delay
        game_data.lock_delay_active = true;
        game_data.lock_delay_timer.reset();
        game_data.lock_move_count = 0;
    }
}

fn place_block_on_board(
    commands: &mut Commands,
    game_data: &mut ResMut<GameData>,
    children_query: Query<&GlobalTransform, With<ActiveDot>>,
    entity: Entity,
    children: &Children,
    transform: &Transform,
) {
    // Calculate hard drop score
    if let Some(start_y) = game_data.hard_drop_start_y {
        let current_y = transform.translation.y;
        let cells_dropped = ((start_y - current_y) / DOT_SIZE).round() as u32;
        game_data.score += cells_dropped * 2;
        game_data.hard_drop_start_y = None;
    }

    // Award soft drop score on placement
    if game_data.soft_drop_cells > 0 {
        game_data.score += game_data.soft_drop_cells;
        game_data.soft_drop_cells = 0;
    }

    children.iter().for_each(|child| {
        commands.entity(*child).remove_parent_in_place();

        let child_global_transform = children_query.get(*child)
            .expect("active dot child should have GlobalTransform");
        let (board_x, board_y) = get_object_position_in_board(
            child_global_transform.translation().x,
            child_global_transform.translation().y,
        );

        commands
            .entity(*child)
            .remove::<ActiveDot>()
            .insert(BoardDot { board_x, board_y });

        place_dot_on_board(board_x, board_y, &mut game_data.board_matrix);
    });
    commands.entity(entity).despawn();

    // Reset hold availability when a piece locks down
    game_data.hold_used = false;

    // Reset lock delay state
    game_data.lock_delay_active = false;
    game_data.lock_move_count = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gravity_level_1_is_about_1_second() {
        let g = gravity_seconds(1);
        assert!((g - 1.0).abs() < 0.01, "Level 1 gravity should be ~1.0s, got {g}");
    }

    #[test]
    fn gravity_monotonically_decreasing() {
        for level in 1..30 {
            assert!(
                gravity_seconds(level + 1) <= gravity_seconds(level),
                "Gravity should decrease: level {} ({}) > level {} ({})",
                level, gravity_seconds(level), level + 1, gravity_seconds(level + 1)
            );
        }
    }

    #[test]
    fn gravity_has_floor() {
        for level in [20, 50, 100, 1000] {
            assert!(
                gravity_seconds(level) >= GRAVITY_FLOOR,
                "Gravity at level {level} should be >= {GRAVITY_FLOOR}, got {}",
                gravity_seconds(level)
            );
        }
    }
}
