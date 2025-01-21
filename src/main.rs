mod background;
mod tetromino;

use std::collections::{HashMap, HashSet};

use bevy::app::{PreStartup, Update};
use bevy::color::{Color, Gray, LinearRgba};
use bevy::input::ButtonInput;
use bevy::log::{debug, LogPlugin};
use bevy::math::{Isometry3d, UVec2, Vec2, Vec3};
#[cfg(feature = "bevy_dev_tools")]
use bevy::prelude::info_once;
use bevy::prelude::{
    in_state, AppExtStates, BuildChildren, BuildChildrenTransformExt, ChildBuild, Children,
    Commands, Component, Entity, Gizmos, GlobalTransform, IntoSystemConfigs, KeyCode, NextState,
    PluginGroup, Query, Res, ResMut, Resource, State, States, Transform, With,
};
use bevy::sprite::Sprite;
use bevy::time::{Time, Timer, TimerMode};
use bevy::utils::default;
use bevy::window::Window;
use bevy::{app::App, window::WindowPlugin, DefaultPlugins};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
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
    .add_plugins(EguiPlugin)
    .add_systems(Update, ui_example_system)
    .init_resource::<GameData>()
    .add_systems(PreStartup, background::setup_background)
    .add_systems(Update, background::setup_background_grid)
    // .add_systems(Startup, test_block)
    // .add_systems(Update, test_block_gizmos)
    .init_resource::<Randomizer7Bag>()
    .init_state::<DropType>()
    .add_systems(Update, spawn_block_system)
    .add_systems(
        Update,
        (block_rotation_system, block_movement_system).run_if(in_state(DropType::Normal)),
    )
    .add_systems(
        Update,
        (
            block_drop_type_system,
            block_drop_system,
            eliminate_line_system,
        )
            .chain(),
    );

    #[cfg(feature = "bevy_dev_tools")]
    {
        app.add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
            .add_systems(Update, toggle_overlay);
    }

    // // this code is compiled only if debug assertions are enabled (debug mode)
    // #[cfg(debug_assertions)]
    // app.add_plugins(DefaultPlugins.set(LogPlugin {
    //     level: bevy::log::Level::DEBUG,
    //     filter: "debug,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
    //     ..Default::default()
    // }));

    // // this code is compiled only if debug assertions are disabled (release mode)
    // #[cfg(not(debug_assertions))]
    // app.add_plugins(DefaultPlugins.set(LogPlugin {
    //     level: bevy::log::Level::INFO,
    //     filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    //     ..Default::default()
    // }));

    app.run();
}

fn ui_example_system(mut contexts: EguiContexts, game_data: Res<GameData>) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        for line in game_data.board_matrix.iter() {
            // ui.label(format!("{:?}", line));
            let mut line_str = String::new();
            line.iter().for_each(|&x| {
                if x == 1 {
                    line_str.push_str("|■");
                } else {
                    line_str.push_str("|□");
                }
            });
            line_str.push('|');
            ui.label(line_str);
        }
    });
}

#[derive(Component)]
struct ActiveBlock;

#[derive(Component)]
struct ActiveDot;

/// https://simon.lc/the-history-of-tetris-randomizers
/// There are 3 main kinds of tetris being competitively played right now and they all play in very different ways:
/// 1. NES Tetris has a no bag randomiser and the win condition for a match is getting a higher score than your opponent.
/// 2. TGM has the system you talked about and the win condition for a match is finishing earlier than your opponent, similar to a speedrun race.
/// 3. Guideline Tetris has the 7-bag we all know and its win condition is making your opponent top out through sending garbage.

#[derive(Resource)]
struct Randomizer7Bag {
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

fn spawn_block_system(
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

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum DropType {
    #[default]
    Normal,
    Hard,
}

fn block_drop_type_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<DropType>>,
    old_state: Res<State<DropType>>,
    mut game_data: ResMut<GameData>,
) {
    if old_state.get() == &DropType::Hard {
        return;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        state.set(DropType::Hard);
        game_data.hard_drop_timer.reset();
    }
}

fn block_drop_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Children, &tetromino::Block, &mut Transform), With<ActiveBlock>>,
    children_query: Query<&GlobalTransform, With<ActiveDot>>,
    time: Res<Time>,
    state: Res<State<DropType>>,
    mut game_data: ResMut<GameData>,
) {
    if state.get() == &DropType::Hard {
        let finished = game_data.hard_drop_timer.tick(time.delta()).finished();
        if !finished {
            return;
        }
    } else {
        let finished = game_data.drop_timer.tick(time.delta()).finished();
        if !finished {
            return;
        }
    }

    if query.iter().count() == 0 {
        return;
    }
    let (entity, children, block, mut transform) = query.single_mut();

    let can_drop = board_check_block_position(
        &mut game_data,
        transform.translation.x,
        transform.translation.y - 25.0,
        block,
    );

    if can_drop {
        transform.translation.y -= 25.0;
    } else {
        block_drop_done(
            &mut commands,
            &mut game_data,
            children_query,
            entity,
            children,
        );
    }
}

fn eliminate_line_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut board_dot_query: Query<((Entity, &mut Transform), &BoardDot)>,
) {
    let mut eliminate_line_index: Vec<usize> = Vec::new();
    for (i, line) in game_data.board_matrix.iter().enumerate() {
        if line.iter().all(|&x| x == 1) {
            eliminate_line_index.push(i);
        }
    }

    if eliminate_line_index.is_empty() {
        return;
    }

    let mut despawned_dot_set: HashSet<u32> = HashSet::new();
    for index in eliminate_line_index.iter() {
        game_data.board_matrix[*index] = [0; 10];
        board_dot_query
            .iter()
            .for_each(|((entity, _transform), board_dot)| {
                if board_dot.board_y == *index as i8 {
                    commands.entity(entity).despawn();
                    despawned_dot_set.insert(entity.index());
                }
            });
    }

    let mut reach_dot_line: bool = false;
    for (i, line) in game_data.board_matrix.clone().iter().enumerate() {
        if line.iter().any(|&x| x == 1) {
            reach_dot_line = true;
        }

        if !reach_dot_line {
            continue;
        }

        debug!("eliminate line to y index: {}", i);

        let mut line_change_map: HashMap<usize, i8> = HashMap::new();
        if line.iter().all(|&x| x == 0) {
            (0..i).for_each(|j| {
                line_change_map
                    .entry(j)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            });

            eliminate_line_inner(&mut commands, &mut game_data, &mut board_dot_query, i);
        }

        // still have bugs, need to fix
        if !line_change_map.is_empty() {
            board_dot_query
                .iter_mut()
                .for_each(|((entity, mut transform), board_dot)| {
                    debug!("entity: {:?}, board_dot: {:?}", entity, board_dot);
                    if despawned_dot_set.contains(&entity.index()) {
                        return;
                    }

                    if line_change_map.contains_key(&(board_dot.board_y as usize)) {
                        let times = line_change_map[&(board_dot.board_y as usize)];
                        transform.translation.y -= 25.0 * times as f32;
                        commands.entity(entity).insert(BoardDot {
                            board_x: board_dot.board_x,
                            board_y: board_dot.board_y + times,
                        });
                    }
                });
        }
    }
}

fn debug_game_data(info: &str, game_data: &GameData) {
    debug!("{}-----------------", info);
    for line in game_data.board_matrix.iter() {
        debug!("{:?}", line);
    }
    debug!("{}-----------------", info);
}

fn eliminate_line_inner(
    commands: &mut Commands,
    game_data: &mut ResMut<GameData>,
    board_dot_query: &mut Query<((Entity, &mut Transform), &BoardDot)>,
    i: usize,
) {
    debug!("move line down 1 index, y index: {}", i - 1);

    // no more line to move down, break the recursion
    if game_data.board_matrix[i - 1].iter().all(|&x| x == 0) {
        return;
    }

    debug_game_data("before", game_data);

    game_data.board_matrix[i] = game_data.board_matrix[i - 1];
    game_data.board_matrix[i - 1] = [0; 10];

    // board_dot_query
    //             .iter_mut()
    //             .for_each(|((entity, mut transform), board_dot)| {
    //                 debug!("entity: {:?}, board_dot: {:?}", entity, board_dot);
    //                 if board_dot.board_y == (i - 1) as i8 {
    //                     transform.translation.y -= 25.0;
    //                     if let Some(mut e) = commands.get_entity(entity) {
    //                         e.try_insert(BoardDot {
    //                             board_x: board_dot.board_x,
    //                             board_y: board_dot.board_y + 1,
    //                         });
    //                     }}

    //             });

    debug_game_data("after", game_data);

    // reach the top of the board, break the recursion
    if i - 1 == 0 {
        return;
    }

    eliminate_line_inner(commands, game_data, board_dot_query, i - 1);
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

#[derive(Component, Debug)]
struct BoardDot {
    board_x: i8,
    board_y: i8,
}

fn block_drop_done(
    commands: &mut Commands,
    game_data: &mut ResMut<GameData>,
    children_query: Query<&GlobalTransform, With<ActiveDot>>,
    entity: Entity,
    children: &Children,
) {
    children.iter().for_each(|child| {
        commands.entity(*child).remove_parent_in_place();
        let child_global_transform = children_query.get(*child).unwrap();
        let (board_x, board_y) = get_dot_board_position2(
            child_global_transform.translation().x,
            child_global_transform.translation().y,
        );

        commands
            .entity(*child)
            .remove::<ActiveDot>()
            .insert(BoardDot { board_x, board_y });

        dot_in_board(board_x, board_y, game_data);
    });
    commands.entity(entity).despawn();
}

fn dot_in_board(board_x: i8, board_y: i8, game_data: &mut ResMut<GameData>) {
    game_data.board_matrix[board_y as usize][board_x as usize] = 1;
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
                    ActiveDot,
                ));
            }
        });
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

fn block_rotation_system(
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
                    ActiveDot,
                ));
            }
        });
    }
}

const TIMER_KEYBOARD_SECS: f32 = 0.1;
const TIMER_DROP_SECS: f32 = 1.0;
const TIMER_HARD_DROP_SECS: f32 = 0.01;

#[derive(Resource)]
struct GameData {
    board_matrix: [[i8; 10]; 20],
    keyboard_timer: Timer,
    drop_timer: Timer,
    hard_drop_timer: Timer,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            board_matrix: [[0; 10]; 20],
            keyboard_timer: Timer::from_seconds(TIMER_KEYBOARD_SECS, TimerMode::Repeating),
            drop_timer: Timer::from_seconds(TIMER_DROP_SECS, TimerMode::Repeating),
            hard_drop_timer: Timer::from_seconds(TIMER_HARD_DROP_SECS, TimerMode::Repeating),
        }
    }
}

fn get_dot_board_position2(x: f32, y: f32) -> (i8, i8) {
    let mut board_x: i8 = if x < 0.0 {
        (4.0 - (x.abs() / 25.0 - 0.5)) as i8
    } else {
        (5.0 + (x / 25.0 - 0.5)) as i8
    };
    // board_x += dot_x;

    let mut board_y: i8 = if y < 0.0 {
        (10.0 + (y.abs() / 25.0 - 0.5)) as i8
    } else {
        (9.0 - (y / 25.0 - 0.5)) as i8
    };
    // board_y += dot_y;

    (board_x, board_y)
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
