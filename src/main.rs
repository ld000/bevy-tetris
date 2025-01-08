mod background;
mod tetromino;

use bevy::app::{PreStartup, Startup, Update};
use bevy::color::{Color, Gray, LinearRgba};
use bevy::input::ButtonInput;
use bevy::math::{Isometry3d, UVec2, Vec2, Vec3};
#[cfg(feature = "bevy_dev_tools")]
use bevy::prelude::info_once;
use bevy::prelude::{
    BuildChildren, ChildBuild, Children, Commands, Component, Entity, Gizmos, KeyCode, Mut,
    PluginGroup, Query, Res, ResMut, Resource, Transform, With,
};
use bevy::sprite::Sprite;
use bevy::time::{Time, Timer, TimerMode};
use bevy::utils::default;
use bevy::window::Window;
use bevy::{app::App, window::WindowPlugin, DefaultPlugins};
use rand::seq::SliceRandom;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Tetris".to_string(),
            resolution: (800.0, 600.0).into(),
            resizable: false,
            ..Default::default()
        }),
        ..default()
    }))
    .init_resource::<GameData>()
    .add_systems(PreStartup, background::setup_background)
    .add_systems(Update, background::setup_background_grid)
    // .add_systems(Startup, test_block)
    // .add_systems(Update, test_block_gizmos)
    .init_resource::<Randomizer>()
    .add_systems(Update, spawn_block_system)
    .add_systems(Update, (rotation_system, block_movement_system))
    .add_systems(Update, block_falling_system);

    #[cfg(feature = "bevy_dev_tools")]
    {
        app.add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
            .add_systems(Update, toggle_overlay);
    }

    app.run();
}

#[derive(Component)]
struct ActiveBlock;

/// There are 3 main kinds of tetris being competitively played right now and they all play in very different ways:
/// 1. NES Tetris has a no bag randomiser and the win condition for a match is getting a higher score than your opponent.
/// 2. TGM has the system you talked about and the win condition for a match is finishing earlier than your opponent, similar to a speedrun race.
/// 3. Guideline Tetris has the 7-bag we all know and its win condition is making your opponent top out through sending garbage.
/// https://simon.lc/the-history-of-tetris-randomizers

#[derive(Resource)]
struct Randomizer {
    bag: Vec<tetromino::Block>,
}

impl Default for Randomizer {
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

impl Randomizer {
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

fn block_randomizer_7bag(randomizer: &mut ResMut<Randomizer>) -> tetromino::Block {
    if randomizer.bag.is_empty() {
        randomizer.init();
    }

    let mut rng = rand::thread_rng();
    randomizer.bag.shuffle(&mut rng);
    randomizer.bag.pop().unwrap()
}

fn spawn_block_system(
    mut commands: Commands,
    mut randomizer: ResMut<Randomizer>,
    query: Query<Entity, With<ActiveBlock>>,
) {
    if query.iter().count() > 0 {
        return;
    }

    let block = block_randomizer_7bag(&mut randomizer);
    let mut transform_y_times: f32 = 8.0;
    if let tetromino::Block::I { .. } = block {
        transform_y_times = 9.0
    };

    spawn_block(&mut commands, block, 0.0, 25.0 * transform_y_times);
}

fn block_falling_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Children, &tetromino::Block, &mut Transform), With<ActiveBlock>>,
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
) {
    let finished = game_data.falling_timer.tick(time.delta()).finished();
    if !finished {
        return;
    }

    for (entity, children, block, mut transform) in query.iter_mut() {
        let in_board = board_check_block_position(
            &mut game_data,
            transform.translation.x,
            transform.translation.y - 25.0,
            block,
        );

        if in_board {
            transform.translation.y -= 25.0;
        } else {
            block_falling_done(
                &mut commands,
                &mut game_data,
                entity,
                children,
                block,
                &transform,
            );
        }
    }
}

fn block_movement_system(
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

        let in_board = board_check_block_position(
            &mut game_data,
            transform.translation.x + transform_x,
            transform.translation.y,
            block,
        );

        if in_board {
            transform.translation.x += transform_x;
        }
    }
}

#[derive(Component)]
struct BoardDot {
    board_x: i8,
    board_y: i8,
}

fn block_falling_done(
    commands: &mut Commands,
    game_data: &mut ResMut<GameData>,
    entity: Entity,
    children: &Children,
    block: &tetromino::Block,
    transform: &Mut<'_, Transform>,
) {
    children.iter().for_each(|child| {
        commands.entity(*child).despawn();
    });
    commands.entity(entity).despawn();

    block.dots_by_state().iter().for_each(|dot| {
        let (board_x, board_y) = get_dot_board_position(
            transform.translation.x,
            transform.translation.y,
            dot.x,
            dot.y,
        );

        commands.spawn((
            Sprite {
                color: block.color(),
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            Transform {
                translation: Vec3::new(
                    dot.x as f32 * 25.0 + transform.translation.x,
                    -dot.y as f32 * 25.0 + transform.translation.y,
                    1.0,
                ),
                ..default()
            },
            BoardDot { board_x, board_y },
        ));

        dot_in_block(board_x, board_y, game_data);
    });
}

fn dot_in_block(board_x: i8, board_y: i8, game_data: &mut ResMut<GameData>) {
    game_data.board_matrix[board_y as usize][board_x as usize] = 1;
}

fn test_block(mut commands: Commands) {
    spawn_block(&mut commands, tetromino::Block::new_i(), -300.0, 0.0);
    spawn_block(&mut commands, tetromino::Block::new_o(), -200.0, 0.0);
    spawn_block(&mut commands, tetromino::Block::new_t(), -100.0, 0.0);
    spawn_block(&mut commands, tetromino::Block::new_s(), 0.0, 0.0);
    spawn_block(&mut commands, tetromino::Block::new_z(), 100.0, 0.0);
    spawn_block(&mut commands, tetromino::Block::new_j(), 200.0, 0.0);
    spawn_block(&mut commands, tetromino::Block::new_l(), 300.0, 0.0);
}

fn spawn_block(
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
                ));
            }
        });
}

fn test_block_gizmos(mut gizmos: Gizmos) {
    block_gizmos(&mut gizmos, -300.0);
    block_gizmos(&mut gizmos, -200.0);
    block_gizmos(&mut gizmos, -100.0);
    block_gizmos(&mut gizmos, 0.0);
    block_gizmos(&mut gizmos, 100.0);
    block_gizmos(&mut gizmos, 200.0);
    block_gizmos(&mut gizmos, 300.0);
}

fn block_gizmos(gizmos: &mut Gizmos, transform_x: f32) {
    gizmos.rect(
        Isometry3d::from_translation(Vec3::new(transform_x, 0.0, 1.0)),
        Vec2::new(100.0, 100.0),
        LinearRgba::gray(0.3),
    );

    gizmos.grid(
        Isometry3d::from_translation(Vec3::new(transform_x, 0.0, 1.0)),
        UVec2::new(4, 4),
        Vec2::new(25.0, 25.0),
        LinearRgba::gray(0.05),
    );
}

fn rotation_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut block_query: Query<(Entity, &Children, &mut tetromino::Block), With<tetromino::Rotation>>,
) {
    for (entity, children, mut block) in block_query.iter_mut() {
        let mut is_rotation = false;
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            tetromino::Rotation::rotate_right(&mut block);
            is_rotation = true;
        }
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            tetromino::Rotation::rotate_left(&mut block);
            is_rotation = true;
        }

        if !is_rotation {
            return;
        }

        commands.entity(entity).clear_children();
        children.iter().for_each(|child| {
            commands.entity(*child).despawn();
        });

        commands.entity(entity).with_children(|parent| {
            for dot in block.dots_by_state().iter() {
                parent.spawn((
                    Sprite {
                        color: block.color(),
                        custom_size: Some(Vec2::new(25.0, 25.0)),
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(dot.x as f32 * 25.0, -dot.y as f32 * 25.0, 0.0),
                        ..default()
                    },
                ));
            }
        });
    }
}

const TIMER_KEYBOARD_SECS: f32 = 0.1;
const TIMER_FALLING_SECS: f32 = 1.0;

#[derive(Resource)]
struct GameData {
    board_matrix: [[i8; 10]; 20],
    keyboard_timer: Timer,
    falling_timer: Timer,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            board_matrix: [[0; 10]; 20],
            keyboard_timer: Timer::from_seconds(TIMER_KEYBOARD_SECS, TimerMode::Repeating),
            falling_timer: Timer::from_seconds(TIMER_FALLING_SECS, TimerMode::Repeating),
        }
    }
}

fn get_dot_board_position(x: f32, y: f32, dot_x: i8, dot_y: i8) -> (i8, i8) {
    let mut board_x: i8 = if x < 0.0 {
        (4.0 - (x.abs() / 25.0 - 0.5)) as i8
    } else {
        (5.0 + (x / 25.0 - 0.5)) as i8
    };
    board_x += dot_x;

    let mut board_y: i8 = if y < 0.0 {
        (10.0 + (y.abs() / 25.0 - 0.5)) as i8
    } else {
        (9.0 - (y / 25.0 - 0.5)) as i8
    };
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
fn board_check_block_position(
    game_data: &mut ResMut<GameData>,
    x: f32,
    y: f32,
    block: &tetromino::Block,
) -> bool {
    let dots = block.dots_by_state();
    // println!("--------------");
    for dot in dots.iter() {
        let (board_x, board_y) = get_dot_board_position(x, y, dot.x, dot.y);

        // println!("x: {}, y: {}", board_x, board_y);

        if !(0..=9).contains(&board_x) || !(0..=19).contains(&board_y) {
            return false;
        }

        if game_data.board_matrix[board_y as usize][board_x as usize] == 1 {
            return false;
        }
    }
    // println!("--------------");

    true
}

#[cfg(feature = "bevy_dev_tools")]
// The system that will enable/disable the debug outlines around the nodes
fn toggle_overlay(
    input: Res<ButtonInput<KeyCode>>,
    mut options: ResMut<bevy::dev_tools::ui_debug_overlay::UiDebugOptions>,
) {
    info_once!("The debug outlines are enabled, press Space to turn them on/off");
    if input.just_pressed(KeyCode::Space) {
        // The toggle method will enable the debug_overlay if disabled and disable if enabled
        options.toggle();
    }
}
