mod background;
mod tetromino;

use bevy::app::{PreStartup, Startup, Update};
use bevy::color::{Color, Gray, LinearRgba};
use bevy::input::ButtonInput;
use bevy::math::{Isometry2d, UVec2, Vec2, Vec3};
#[cfg(feature = "bevy_dev_tools")]
use bevy::prelude::{info_once, ResMut};
use bevy::prelude::{
    BuildChildren, ChildBuild, Children, Commands, Entity, Gizmos, KeyCode, PluginGroup, Query,
    Res, Resource, Transform, With,
};
use bevy::sprite::Sprite;
use bevy::time::{Timer, TimerMode};
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
    .add_systems(Startup, test_block)
    .add_systems(Update, spawn_block_gizmos)
    .add_systems(Update, rotation_system);

    #[cfg(feature = "bevy_dev_tools")]
    {
        app.add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
            .add_systems(Update, toggle_overlay);
    }

    app.run();
}

fn test_block(mut commands: Commands) {
    spawn_tetromino(&mut commands, tetromino::Tetromino::new_i(), -300.0);
    spawn_tetromino(&mut commands, tetromino::Tetromino::new_o(), -200.0);
    spawn_tetromino(&mut commands, tetromino::Tetromino::new_t(), -100.0);
    spawn_tetromino(&mut commands, tetromino::Tetromino::new_s(), 0.0);
    spawn_tetromino(&mut commands, tetromino::Tetromino::new_z(), 100.0);
    spawn_tetromino(&mut commands, tetromino::Tetromino::new_j(), 200.0);
    spawn_tetromino(&mut commands, tetromino::Tetromino::new_l(), 300.0);
}

fn spawn_tetromino(commands: &mut Commands, tetromino: tetromino::Tetromino, transform_x: f32) {
    let dots: [tetromino::Dot; 4] = tetromino.dots_by_state();
    let color: Color = tetromino.color();

    commands
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(0.0, 0.0)),
                ..default()
            },
            Transform {
                translation: Vec3::new(-25.0 * 1.5 + transform_x, 25.0 * 1.5, 1.0),
                ..default()
            },
            tetromino,
            tetromino::Rotation {},
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

fn spawn_block_gizmos(mut gizmos: Gizmos) {
    block_gizmos(&mut gizmos, -300.0);
    block_gizmos(&mut gizmos, -200.0);
    block_gizmos(&mut gizmos, -100.0);
    block_gizmos(&mut gizmos, 0.0);
    block_gizmos(&mut gizmos, 100.0);
    block_gizmos(&mut gizmos, 200.0);
    block_gizmos(&mut gizmos, 300.0);
}

fn block_gizmos(gizmos: &mut Gizmos, transform_x: f32) {
    gizmos.rect_2d(
        Isometry2d::from_translation(Vec2::new(transform_x, 0.0)),
        Vec2::new(100.0, 100.0),
        LinearRgba::gray(0.3),
    );
    gizmos
        .grid_2d(
            Isometry2d::from_translation(Vec2::new(transform_x, 0.0)),
            UVec2::new(4, 4),
            Vec2::new(25.0, 25.0),
            LinearRgba::gray(0.05),
        )
        .outer_edges();
}

fn rotation_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut block_query: Query<
        (Entity, &Children, &mut tetromino::Tetromino),
        With<tetromino::Rotation>,
    >,
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
    matrix: [[u8; 10]; 20],
    keyboard_timer: Timer,
    falling_timer: Timer,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            matrix: [[0; 10]; 20],
            keyboard_timer: Timer::from_seconds(TIMER_KEYBOARD_SECS, TimerMode::Repeating),
            falling_timer: Timer::from_seconds(TIMER_FALLING_SECS, TimerMode::Repeating),
        }
    }
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
