mod background;
mod tetromino;

use bevy::app::{PreStartup, Startup, Update};
use bevy::color::{Color, Gray, LinearRgba};
use bevy::input::ButtonInput;
use bevy::math::{Isometry3d, UVec2, Vec2, Vec3};
#[cfg(feature = "bevy_dev_tools")]
use bevy::prelude::info_once;
use bevy::prelude::{
    BuildChildren, ChildBuild, Children, Commands, Component, Entity, Gizmos, KeyCode, PluginGroup,
    Query, Res, ResMut, Resource, Transform, With,
};
use bevy::sprite::Sprite;
use bevy::time::{Time, Timer, TimerMode};
use bevy::utils::default;
use bevy::window::Window;
use bevy::{app::App, window::WindowPlugin, DefaultPlugins};

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
    .insert_resource(GameData::default())
    .add_systems(PreStartup, background::setup_background)
    .add_systems(Update, background::setup_background_grid)
    // .add_systems(Startup, test_block)
    .add_systems(Update, spawn_test_block_gizmos)
    .add_systems(Startup, spawn_block)
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

fn spawn_block(mut commands: Commands) {
    let tetromino = tetromino::Block::new_t();
    let mut transform_y_times: f32 = 8.0;
    if let tetromino::Block::I { .. } = tetromino {
        transform_y_times = 9.0
    };

    spawn_tetromino(&mut commands, tetromino, 0.0, 25.0 * transform_y_times);
}

fn block_falling_system(
    mut query: Query<(&tetromino::Block, &mut Transform), With<ActiveBlock>>,
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
) {
    let finished = game_data.falling_timer.tick(time.delta()).finished();
    if !finished {
        return;
    }

    for (block, mut transform) in query.iter_mut() {
        let in_board = board_check_block_position(
            transform.translation.x,
            transform.translation.y - 25.0,
            block,
        );

        if in_board {
            transform.translation.y -= 25.0;
        }
    }
}

fn block_movement_system(
    mut query: Query<(&tetromino::Block, &mut Transform), With<ActiveBlock>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
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
            transform.translation.x + transform_x,
            transform.translation.y,
            block,
        );

        if in_board {
            transform.translation.x += transform_x;
        }
    }
}

fn test_block(mut commands: Commands) {
    spawn_tetromino(&mut commands, tetromino::Block::new_i(), -300.0, 0.0);
    spawn_tetromino(&mut commands, tetromino::Block::new_o(), -200.0, 0.0);
    spawn_tetromino(&mut commands, tetromino::Block::new_t(), -100.0, 0.0);
    spawn_tetromino(&mut commands, tetromino::Block::new_s(), 0.0, 0.0);
    spawn_tetromino(&mut commands, tetromino::Block::new_z(), 100.0, 0.0);
    spawn_tetromino(&mut commands, tetromino::Block::new_j(), 200.0, 0.0);
    spawn_tetromino(&mut commands, tetromino::Block::new_l(), 300.0, 0.0);
}

fn spawn_tetromino(
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

fn spawn_test_block_gizmos(mut gizmos: Gizmos) {
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
    for (entity, children, mut tetromino) in block_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            tetromino::Rotation::rotate_right(&mut tetromino);
        }
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            tetromino::Rotation::rotate_left(&mut tetromino);
        }

        commands.entity(entity).clear_children();
        children.iter().for_each(|child| {
            commands.entity(*child).despawn();
        });

        commands.entity(entity).with_children(|parent| {
            for dot in tetromino.dots_by_state().iter() {
                parent.spawn((
                    Sprite {
                        color: tetromino.color(),
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
    board_matrix: [[Option<tetromino::Dot>; 10]; 20],
    keyboard_timer: Timer,
    falling_timer: Timer,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            board_matrix: [[None; 10]; 20],
            keyboard_timer: Timer::from_seconds(TIMER_KEYBOARD_SECS, TimerMode::Repeating),
            falling_timer: Timer::from_seconds(TIMER_FALLING_SECS, TimerMode::Repeating),
        }
    }
}

/// check if the block is in the board
/// -----------------------------------
/// | 0, 0 | 0, 1 | 0, 2 | ... | 0, 9 |
/// | 1, 0 | 1, 1 | 1, 2 | ... | 1, 9 |
/// | 2, 0 | 2, 1 | 2, 2 | ... | 2, 9 |
/// | ...  | ...  | ...  | ... | ...  |
/// | 19,0 | 19,1 | 19,2 | ... | 19,9 |
/// -----------------------------------
fn board_check_block_position(x: f32, y: f32, block: &tetromino::Block) -> bool {
    let dots = block.dots_by_state();
    // println!("--------------");
    for dot in dots.iter() {
        let mut board_x: i8 = if x < 0.0 {
            (4.0 - (x.abs() / 25.0 - 0.5)) as i8
        } else {
            (5.0 + (x / 25.0 - 0.5)) as i8
        };
        board_x += dot.x;

        let mut board_y: i8 = if y < 0.0 {
            (10.0 + (y.abs() / 25.0 - 0.5)) as i8
        } else {
            (9.0 - (y / 25.0 - 0.5)) as i8
        };
        board_y += dot.y;

        // println!("x: {}, y: {}", board_x, board_y);

        if !(0..=9).contains(&board_x) || !(0..=19).contains(&board_y) {
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
