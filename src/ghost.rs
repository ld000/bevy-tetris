use bevy::math::Vec2;
use bevy::prelude::{Commands, Entity, Query, Res, Resource, Transform, With};
use bevy::sprite::Sprite;
use bevy::utils::default;
use bevy::color::Color;

use crate::board::board_check_block_position;
use crate::common_component::{ActiveBlock, GameData, GhostDot};
use crate::tetromino;

#[derive(Resource, Default)]
pub(crate) struct GhostTracker {
    block_x: f32,
    ghost_y: f32,
    rotation_state: Option<tetromino::State>,
}

pub(crate) fn update_ghost_piece_system(
    mut commands: Commands,
    ghost_dots: Query<Entity, With<GhostDot>>,
    query: Query<(&tetromino::Block, &Transform), With<ActiveBlock>>,
    game_data: Res<GameData>,
    mut tracker: bevy::prelude::ResMut<GhostTracker>,
) {
    let Ok((block, transform)) = query.get_single() else {
        // No active block — clear ghost if present
        if tracker.rotation_state.is_some() {
            for entity in ghost_dots.iter() {
                commands.entity(entity).despawn();
            }
            *tracker = GhostTracker::default();
        }
        return;
    };

    // Simulate dropping until we can't go further
    let mut ghost_y = transform.translation.y;
    while board_check_block_position(
        &game_data.board_matrix,
        transform.translation.x,
        ghost_y - 25.0,
        block,
    ) {
        ghost_y -= 25.0;
    }

    let current_state = *block.state();
    let block_x = transform.translation.x;

    // Skip rebuild if nothing changed
    if tracker.rotation_state == Some(current_state)
        && tracker.block_x == block_x
        && tracker.ghost_y == ghost_y
    {
        return;
    }

    // Despawn existing ghost dots
    for entity in ghost_dots.iter() {
        commands.entity(entity).despawn();
    }

    // Update tracker
    tracker.block_x = block_x;
    tracker.ghost_y = ghost_y;
    tracker.rotation_state = Some(current_state);

    // Don't draw ghost if it's at the same position as the active piece
    if ghost_y == transform.translation.y {
        return;
    }

    // Spawn ghost dots at the landing position
    let srgba = block.color().to_srgba();
    let ghost_color = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.2);

    for dot in block.dots_by_state().iter() {
        let x = transform.translation.x + dot.x as f32 * 25.0;
        let y = ghost_y + (-dot.y as f32) * 25.0;

        commands.spawn((
            Sprite {
                color: ghost_color,
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.5),
            GhostDot,
        ));
    }
}
