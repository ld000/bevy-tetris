use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, DespawnRecursiveExt, Entity, KeyCode, NextState, Query, Res, ResMut, Resource,
    Transform, With,
};
use bevy::sprite::Sprite;
use bevy::utils::default;

use crate::common_component::{ActiveBlock, DropType, GameData, HoldDot};
use crate::tetromino;

#[derive(Resource, Default)]
pub(crate) struct HoldTracker {
    /// Discriminant of the last rendered held block (None = no block rendered)
    last_block_disc: Option<std::mem::Discriminant<tetromino::Block>>,
    last_hold_used: bool,
}

pub(crate) fn hold_block_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
    query: Query<(Entity, &tetromino::Block), With<ActiveBlock>>,
    mut state: ResMut<NextState<DropType>>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyC) {
        return;
    }
    if game_data.hold_used {
        return;
    }

    let Ok((entity, block)) = query.get_single() else {
        return;
    };

    let mut current_block = block.clone();
    current_block.reset_rotation();

    let previously_held = game_data.held_block.take();
    game_data.held_block = Some(current_block);
    game_data.hold_used = true;

    // Reset soft drop tracking
    game_data.soft_drop_cells = 0;
    game_data.hard_drop_start_y = None;

    commands.entity(entity).despawn_recursive();

    // Spawn previously held piece if there was one
    if let Some(held) = previously_held {
        let mut transform_y_times: f32 = 8.0;
        if let tetromino::Block::I { .. } = held {
            transform_y_times = 9.0;
        }
        crate::spawn_block_system::spawn_block(&mut commands, held, 0.0, 25.0 * transform_y_times);
    }
    // If hold was empty, spawn_block_system will handle spawning next piece

    state.set(DropType::Normal);
}

const HOLD_BOX_CENTER_X: f32 = -165.0;
const HOLD_BOX_CENTER_Y: f32 = 180.0;
const HOLD_DOT_SIZE: f32 = 11.25;

pub(crate) fn update_hold_preview_system(
    mut commands: Commands,
    game_data: Res<GameData>,
    hold_dots: Query<Entity, With<HoldDot>>,
    mut tracker: ResMut<HoldTracker>,
) {
    let current_disc = game_data.held_block.as_ref().map(std::mem::discriminant);
    let current_hold_used = game_data.hold_used;

    if current_disc == tracker.last_block_disc && current_hold_used == tracker.last_hold_used {
        return;
    }

    tracker.last_block_disc = current_disc;
    tracker.last_hold_used = current_hold_used;

    for entity in hold_dots.iter() {
        commands.entity(entity).despawn();
    }

    let Some(ref block) = game_data.held_block else {
        return;
    };

    let dots = block.dots_by_state();
    let mut color = block.color();

    // Dim to 40% opacity when hold is used
    if game_data.hold_used {
        let srgba = color.to_srgba();
        color = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.4);
    }

    let min_x = dots.iter().map(|d| d.x).min().unwrap() as f32;
    let max_x = dots.iter().map(|d| d.x).max().unwrap() as f32;
    let min_y = dots.iter().map(|d| d.y).min().unwrap() as f32;
    let max_y = dots.iter().map(|d| d.y).max().unwrap() as f32;
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    for dot in dots.iter() {
        let x = HOLD_BOX_CENTER_X + (dot.x as f32 - center_x) * HOLD_DOT_SIZE;
        let y = HOLD_BOX_CENTER_Y - (dot.y as f32 - center_y) * HOLD_DOT_SIZE;

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(HOLD_DOT_SIZE, HOLD_DOT_SIZE)),
                ..default()
            },
            Transform::from_xyz(x, y, 2.0),
            HoldDot,
        ));
    }
}
