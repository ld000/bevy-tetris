use std::collections::VecDeque;

use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::{
        BuildChildren, ChildBuild, Commands, DetectChanges, Entity, NextState, Query, Res, ResMut, Resource,
        Transform, With,
    },
    sprite::Sprite,
    utils::default,
};
use rand::seq::SliceRandom;

use crate::{
    board_check_block_position,
    common_component::{ActiveBlock, ActiveDot, DropType, GameData, GameState, PreviewDot},
    tetromino,
};

/// https://simon.lc/the-history-of-tetris-randomizers
/// Guideline Tetris 7-bag randomizer: shuffles all 7 pieces, dispenses one by one,
/// then refills. Uses a queue so we can peek ahead for the NEXT preview.

fn new_shuffled_bag() -> Vec<tetromino::Block> {
    let mut bag = vec![
        tetromino::Block::new_i(),
        tetromino::Block::new_o(),
        tetromino::Block::new_t(),
        tetromino::Block::new_s(),
        tetromino::Block::new_z(),
        tetromino::Block::new_j(),
        tetromino::Block::new_l(),
    ];
    let mut rng = rand::thread_rng();
    bag.shuffle(&mut rng);
    bag
}

#[derive(Resource)]
pub struct Randomizer7Bag {
    queue: VecDeque<tetromino::Block>,
}

impl Default for Randomizer7Bag {
    fn default() -> Self {
        let mut queue = VecDeque::with_capacity(14);
        queue.extend(new_shuffled_bag());
        queue.extend(new_shuffled_bag());
        Self { queue }
    }
}

impl Randomizer7Bag {
    pub fn pop_next(&mut self) -> tetromino::Block {
        let block = self.queue.pop_front().unwrap();
        self.ensure_minimum();
        block
    }

    pub fn peek(&self, count: usize) -> Vec<&tetromino::Block> {
        self.queue.iter().take(count).collect()
    }

    fn ensure_minimum(&mut self) {
        if self.queue.len() < 7 {
            self.queue.extend(new_shuffled_bag());
        }
    }
}

pub fn spawn_block_system(
    mut commands: Commands,
    mut randomizer: ResMut<Randomizer7Bag>,
    query: Query<Entity, With<ActiveBlock>>,
    mut state: ResMut<NextState<DropType>>,
    mut game_data: ResMut<GameData>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if query.iter().count() > 0 {
        return;
    }

    let block = randomizer.pop_next();
    let mut transform_y_times: f32 = 8.0;
    if let tetromino::Block::I { .. } = block {
        transform_y_times = 9.0
    };

    let spawn_x = -25.0 * 1.5;
    let spawn_y = 25.0 * 1.5 + 25.0 * transform_y_times;

    if !board_check_block_position(&mut game_data, spawn_x, spawn_y, &block) {
        game_state.set(GameState::GameOver);
        return;
    }

    state.set(DropType::Normal);
    game_data.drop_timer.reset();
    spawn_block(&mut commands, block, 0.0, 25.0 * transform_y_times);
}

pub fn spawn_block(
    commands: &mut Commands,
    block: tetromino::Block,
    transform_x: f32,
    transform_y: f32,
) {
    let dots: [tetromino::Dot; 4] = block.dots_by_state();
    let color: Color = block.color();

    commands
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(0.0, 0.0)),
                ..default()
            },
            Transform {
                translation: Vec3::new(-25.0 * 1.5 + transform_x, 25.0 * 1.5 + transform_y, 1.0),
                ..default()
            },
            block,
            tetromino::Rotation {},
            ActiveBlock,
        ))
        .with_children(|parent| {
            for dot in dots.iter() {
                parent.spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::new(25.0, 25.0)),
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(dot.x as f32 * 25.0, -dot.y as f32 * 25.0, 0.0),
                        ..default()
                    },
                    ActiveDot,
                ));
            }
        });
}

// NEXT box world coordinates (right panel center x=165, inner box center y=70)
const NEXT_BOX_CENTER_X: f32 = 165.0;
const NEXT_BOX_CENTER_Y: f32 = 70.0;
const PREVIEW_DOT_SIZE: f32 = 11.25;
const PREVIEW_SLOT_HEIGHT: f32 = 50.0;

pub fn update_preview_system(
    mut commands: Commands,
    randomizer: Res<Randomizer7Bag>,
    preview_dots: Query<Entity, With<PreviewDot>>,
) {
    if !randomizer.is_changed() {
        return;
    }

    for entity in preview_dots.iter() {
        commands.entity(entity).despawn();
    }

    let upcoming = randomizer.peek(6);
    for (i, block) in upcoming.iter().enumerate() {
        let dots = block.dots_by_state();
        let color = block.color();

        let min_x = dots.iter().map(|d| d.x).min().unwrap() as f32;
        let max_x = dots.iter().map(|d| d.x).max().unwrap() as f32;
        let min_y = dots.iter().map(|d| d.y).min().unwrap() as f32;
        let max_y = dots.iter().map(|d| d.y).max().unwrap() as f32;
        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;

        let slot_y = NEXT_BOX_CENTER_Y + 125.0 - i as f32 * PREVIEW_SLOT_HEIGHT;

        for dot in dots.iter() {
            let x = NEXT_BOX_CENTER_X + (dot.x as f32 - center_x) * PREVIEW_DOT_SIZE;
            let y = slot_y - (dot.y as f32 - center_y) * PREVIEW_DOT_SIZE;

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(PREVIEW_DOT_SIZE, PREVIEW_DOT_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x, y, 2.0),
                PreviewDot,
            ));
        }
    }
}
