use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::{
        BuildChildren, ChildBuild, Commands, Entity, NextState, Query, ResMut, Resource, Transform,
        With,
    },
    sprite::Sprite,
    utils::default,
};
use rand::seq::SliceRandom;

use crate::{
    common_component::{ActiveBlock, ActiveDot, DropType, GameData},
    tetromino,
};

/// https://simon.lc/the-history-of-tetris-randomizers
/// There are 3 main kinds of tetris being competitively played right now and they all play in very different ways:
/// 1. NES Tetris has a no bag randomiser and the win condition for a match is getting a higher score than your opponent.
/// 2. TGM has the system you talked about and the win condition for a match is finishing earlier than your opponent, similar to a speedrun race.
/// 3. Guideline Tetris has the 7-bag we all know and its win condition is making your opponent top out through sending garbage.

#[derive(Resource)]
pub struct Randomizer7Bag {
    bag: Vec<tetromino::Block>,
}

impl Default for Randomizer7Bag {
    fn default() -> Self {
        Self {
            bag: vec![
                tetromino::Block::new_i(),
                tetromino::Block::new_o(),
                tetromino::Block::new_t(),
                tetromino::Block::new_s(),
                tetromino::Block::new_z(),
                tetromino::Block::new_j(),
                tetromino::Block::new_l(),
            ],
        }
    }
}

impl Randomizer7Bag {
    fn init(&mut self) {
        self.bag = vec![
            tetromino::Block::new_i(),
            tetromino::Block::new_o(),
            tetromino::Block::new_t(),
            tetromino::Block::new_s(),
            tetromino::Block::new_z(),
            tetromino::Block::new_j(),
            tetromino::Block::new_l(),
        ];
    }
}

fn block_randomizer_7bag(randomizer: &mut ResMut<Randomizer7Bag>) -> tetromino::Block {
    if randomizer.bag.is_empty() {
        randomizer.init();
    }

    let mut rng = rand::thread_rng();
    randomizer.bag.shuffle(&mut rng);
    randomizer.bag.pop().unwrap()
}

pub fn spawn_block_system(
    mut commands: Commands,
    mut randomizer: ResMut<Randomizer7Bag>,
    query: Query<Entity, With<ActiveBlock>>,
    mut state: ResMut<NextState<DropType>>,
    mut game_data: ResMut<GameData>,
) {
    if query.iter().count() > 0 {
        return;
    }

    let block = block_randomizer_7bag(&mut randomizer);
    let mut transform_y_times: f32 = 8.0;
    if let tetromino::Block::I { .. } = block {
        transform_y_times = 9.0
    };

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
