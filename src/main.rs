mod background;
mod common_component;
mod spawn_block_system;
mod test_block;
mod tetromino;

use std::collections::HashMap;

use bevy::app::{PreStartup, Update};
use bevy::input::ButtonInput;
use bevy::log::debug;
use bevy::math::{Vec2, Vec3};
#[cfg(feature = "bevy_dev_tools")]
use bevy::prelude::info_once;
use bevy::prelude::{
    in_state, AppExtStates, BuildChildren, BuildChildrenTransformExt, ChildBuild, Children,
    Commands, Component, Condition, DespawnRecursiveExt, DetectChanges, Entity, GlobalTransform, IntoSystemConfigs, KeyCode, NextState,
    OnEnter, OnExit, PluginGroup, Query, Res, ResMut, State, Text, Transform, With, Without,
};
use bevy::sprite::Sprite;
use bevy::time::Time;
use bevy::utils::default;
use bevy::window::Window;
use bevy::{app::App, window::WindowPlugin, DefaultPlugins};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy::color::Color;
use bevy::text::{TextColor, TextFont};
use bevy::ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, PositionType, Val};
use common_component::{ActiveBlock, ActiveDot, DropType, GameData, GameOverOverlay, GameState, GhostDot, HoldDot, LevelText, LinesText, PauseOverlay, PreviewDot, ScoreText};
use spawn_block_system::{spawn_block_system, update_preview_system};
use spawn_block_system::Randomizer7Bag;

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
    .add_systems(Update, update_score_display)
    .add_systems(Update, update_hold_preview_system)
    .init_resource::<GameData>()
    .add_systems(PreStartup, background::setup_background)
    .add_systems(Update, background::setup_background_grid)
    // .add_systems(Startup, test_block)
    // .add_systems(Update, test_block_gizmos)
    .init_resource::<Randomizer7Bag>()
    .init_state::<DropType>()
    .init_state::<GameState>()
    .add_systems(Update, (spawn_block_system, update_preview_system).chain().run_if(in_state(GameState::Playing)))
    .add_systems(
        Update,
        (block_rotation_system, block_movement_system, hold_block_system)
            .run_if(in_state(GameState::Playing).and(in_state(DropType::Normal).or(in_state(DropType::Soft)))),
    )
    .add_systems(
        Update,
        (
            block_drop_type_system,
            block_drop_system,
            eliminate_line_system,
        )
            .chain()
            .run_if(in_state(GameState::Playing)),
    )
    .add_systems(Update, update_ghost_piece_system.run_if(in_state(GameState::Playing)))
    .add_systems(Update, pause_system.run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))))
    .add_systems(OnEnter(GameState::Paused), pause_display_system)
    .add_systems(OnExit(GameState::Paused), unpause_cleanup_system)
    .add_systems(OnEnter(GameState::GameOver), game_over_display_system)
    .add_systems(Update, restart_system.run_if(in_state(GameState::GameOver)));

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

fn gravity_seconds(level: u32) -> f32 {
    let l = level as f32;
    let base = (0.8 - ((l - 1.0) * 0.007)).max(0.0);
    base.powf(l - 1.0).max(0.05)
}

fn update_score_display(
    game_data: Res<GameData>,
    mut score_query: Query<&mut Text, With<ScoreText>>,
    mut lines_query: Query<&mut Text, (With<LinesText>, Without<ScoreText>, Without<LevelText>)>,
    mut level_query: Query<&mut Text, (With<LevelText>, Without<ScoreText>, Without<LinesText>)>,
) {
    if let Ok(mut text) = score_query.get_single_mut() {
        **text = format!("{}", game_data.score);
    }
    if let Ok(mut text) = lines_query.get_single_mut() {
        **text = format!("Lines: {}", game_data.lines_cleared);
    }
    if let Ok(mut text) = level_query.get_single_mut() {
        **text = format!("Level: {}", game_data.level);
    }
}

fn block_drop_type_system(
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

fn block_drop_system(
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
        &mut game_data,
        transform.translation.x,
        transform.translation.y - 25.0,
        block,
    );

    if can_drop {
        transform.translation.y -= 25.0;
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
        let cells_dropped = ((start_y - current_y) / 25.0).round() as u32;
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

        let child_global_transform = children_query.get(*child).unwrap();
        let (board_x, board_y) = get_object_position_in_board(
            child_global_transform.translation().x,
            child_global_transform.translation().y,
        );

        commands
            .entity(*child)
            .remove::<ActiveDot>()
            .insert(BoardDot { board_x, board_y });

        place_dot_on_board(board_x, board_y, game_data);
    });
    commands.entity(entity).despawn();

    // Reset hold availability when a piece locks down
    game_data.hold_used = false;

    // Reset lock delay state
    game_data.lock_delay_active = false;
    game_data.lock_move_count = 0;
}

fn eliminate_line_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut board_dot_query: Query<((Entity, &mut Transform), &BoardDot)>,
) {
    let mut line_indexs_to_eliminate: Vec<usize> = Vec::new();
    for (i, line) in game_data.board_matrix.iter().enumerate() {
        if line.iter().all(|&x| x == 1) {
            line_indexs_to_eliminate.push(i);
        }
    }
    if line_indexs_to_eliminate.is_empty() {
        return;
    }

    // Award points for line clears
    let lines_count = line_indexs_to_eliminate.len();
    let points = match lines_count {
        1 => 100,
        2 => 300,
        3 => 500,
        4 => 800,
        _ => 0,
    };
    game_data.score += points * game_data.level;
    game_data.lines_cleared += lines_count as u32;
    game_data.level = (game_data.lines_cleared / 10) + 1;
    let new_duration = std::time::Duration::from_secs_f32(gravity_seconds(game_data.level));
    game_data.drop_timer.set_duration(new_duration);

    for index in line_indexs_to_eliminate.iter() {
        game_data.board_matrix[*index] = [0; 10];
        board_dot_query
            .iter()
            .for_each(|((entity, _transform), board_dot)| {
                if board_dot.board_y == *index as i8 {
                    commands.entity(entity).despawn();
                }
            });
    }

    let mut is_reached_dot_line: bool = false;
    let mut line_change_map: HashMap<usize, i8> = HashMap::new();
    for (i, line) in game_data.board_matrix.clone().iter().enumerate() {
        if line.iter().any(|&x| x == 1) {
            is_reached_dot_line = true;
        }

        if !is_reached_dot_line {
            continue;
        }

        debug!("eliminate line to y index: {}", i);

        if line.iter().all(|&x| x == 0) {
            eliminate_line_inner(&mut game_data, i);

            (0..i).for_each(|j| {
                if line_indexs_to_eliminate.contains(&j) {
                    return;
                }
                line_change_map
                    .entry(j)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            });
        }
    }

    if !line_change_map.is_empty() {
        board_dot_query
            .iter_mut()
            .for_each(|((entity, mut transform), board_dot)| {
                debug!("entity: {:?}, board_dot: {:?}", entity, board_dot);

                if line_change_map.contains_key(&(board_dot.board_y as usize)) {
                    let line_change_times = line_change_map[&(board_dot.board_y as usize)];
                    transform.translation.y -= 25.0 * line_change_times as f32;
                    commands.entity(entity).insert(BoardDot {
                        board_x: board_dot.board_x,
                        board_y: board_dot.board_y + line_change_times,
                    });
                }
            });
    }
}

fn eliminate_line_inner(game_data: &mut ResMut<GameData>, i: usize) {
    debug!("move line down 1 index, y index: {}", i - 1);

    // no more line to move down, break the recursion
    if game_data.board_matrix[i - 1].iter().all(|&x| x == 0) {
        return;
    }

    debug_game_data("before", game_data);

    game_data.board_matrix[i] = game_data.board_matrix[i - 1];
    game_data.board_matrix[i - 1] = [0; 10];

    debug_game_data("after", game_data);

    // reach the top of the board, break the recursion
    if i - 1 == 0 {
        return;
    }

    eliminate_line_inner(game_data, i - 1);
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

        if transform_x == 0.0 {
            continue;
        }

        let in_board = board_check_block_position(
            &mut game_data,
            transform.translation.x + transform_x,
            transform.translation.y,
            block,
        );

        if in_board {
            transform.translation.x += transform_x;

            // Reset lock delay on successful move (up to 15 resets)
            if game_data.lock_delay_active {
                game_data.lock_move_count += 1;
                if game_data.lock_move_count < 15 {
                    game_data.lock_delay_timer.reset();
                }
            }
        }
    }
}

#[derive(Component, Debug)]
struct BoardDot {
    board_x: i8,
    board_y: i8,
}

fn place_dot_on_board(board_x: i8, board_y: i8, game_data: &mut ResMut<GameData>) {
    game_data.board_matrix[board_y as usize][board_x as usize] = 1;
}

/// Returns kick offset table for SRS (Super Rotation System)
/// Returns array of (x, y) offsets to try in order
fn get_kick_offsets(block: &tetromino::Block, from: tetromino::State, to: tetromino::State) -> Vec<(i8, i8)> {
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

fn block_rotation_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut block_query: Query<(Entity, &Children, &mut tetromino::Block, &Transform), With<tetromino::Rotation>>,
    mut game_data: ResMut<GameData>,
) {
    for (entity, children, mut block, transform) in block_query.iter_mut() {
        let (from, to);
        let original_state = block.state().clone();

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

            if board_check_block_position(&mut game_data, test_x, test_y, &block) {
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
                ..transform.clone()
            });

            // Update visual representation
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

fn get_object_position_in_board(x: f32, y: f32) -> (i8, i8) {
    let board_x: i8 = if x < 0.0 {
        (4.0 - (x.abs() / 25.0 - 0.5)) as i8
    } else {
        (5.0 + (x / 25.0 - 0.5)) as i8
    };

    let board_y: i8 = if y < 0.0 {
        (10.0 + (y.abs() / 25.0 - 0.5)) as i8
    } else {
        (9.0 - (y / 25.0 - 0.5)) as i8
    };

    (board_x, board_y)
}

pub fn get_dot_position_in_board(x: f32, y: f32, dot_x: i8, dot_y: i8) -> (i8, i8) {
    let (mut board_x, mut board_y) = get_object_position_in_board(x, y);

    board_x += dot_x;
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
pub fn board_check_block_position(
    game_data: &mut ResMut<GameData>,
    x: f32,
    y: f32,
    block: &tetromino::Block,
) -> bool {
    let dots = block.dots_by_state();
    // println!("--------------");
    for dot in dots.iter() {
        let (board_x, board_y) = get_dot_position_in_board(x, y, dot.x, dot.y);

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

fn hold_block_system(
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
        spawn_block_system::spawn_block(&mut commands, held, 0.0, 25.0 * transform_y_times);
    }
    // If hold was empty, spawn_block_system will handle spawning next piece

    state.set(DropType::Normal);
}

const HOLD_BOX_CENTER_X: f32 = -165.0;
const HOLD_BOX_CENTER_Y: f32 = 180.0;
const HOLD_DOT_SIZE: f32 = 11.25;

fn update_hold_preview_system(
    mut commands: Commands,
    game_data: Res<GameData>,
    hold_dots: Query<Entity, With<HoldDot>>,
) {
    if !game_data.is_changed() {
        return;
    }

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

fn debug_game_data(info: &str, game_data: &GameData) {
    debug!("{}-----------------", info);
    for line in game_data.board_matrix.iter() {
        debug!("{:?}", line);
    }
    debug!("{}-----------------", info);
}

fn game_over_display_system(mut commands: Commands, game_data: Res<GameData>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            GameOverOverlay,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
            ));
            parent.spawn((
                Text::new(format!("Score: {}", game_data.score)),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                Node {
                    margin: bevy::ui::UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("Press Enter to restart"),
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                Node {
                    margin: bevy::ui::UiRect::top(Val::Px(30.0)),
                    ..default()
                },
            ));
        });
}

fn restart_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
    board_dots: Query<Entity, With<BoardDot>>,
    active_blocks: Query<Entity, With<ActiveBlock>>,
    overlay: Query<Entity, With<GameOverOverlay>>,
    preview_dots: Query<Entity, With<PreviewDot>>,
    hold_dots: Query<Entity, With<HoldDot>>,
    ghost_dots: Query<Entity, With<GhostDot>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut drop_state: ResMut<NextState<DropType>>,
    mut randomizer: ResMut<Randomizer7Bag>,
) {
    if !keyboard_input.just_pressed(KeyCode::Enter) {
        return;
    }

    *game_data = GameData::default();
    *randomizer = Randomizer7Bag::default();

    for entity in board_dots.iter() {
        commands.entity(entity).despawn();
    }
    for entity in active_blocks.iter() {
        commands.entity(entity).despawn();
    }
    for entity in overlay.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in preview_dots.iter() {
        commands.entity(entity).despawn();
    }
    for entity in hold_dots.iter() {
        commands.entity(entity).despawn();
    }
    for entity in ghost_dots.iter() {
        commands.entity(entity).despawn();
    }

    game_state.set(GameState::Playing);
    drop_state.set(DropType::Normal);
}

fn update_ghost_piece_system(
    mut commands: Commands,
    ghost_dots: Query<Entity, With<GhostDot>>,
    query: Query<(&tetromino::Block, &Transform), With<ActiveBlock>>,
    mut game_data: ResMut<GameData>,
) {
    // Despawn existing ghost dots
    for entity in ghost_dots.iter() {
        commands.entity(entity).despawn();
    }

    let Ok((block, transform)) = query.get_single() else {
        return;
    };

    // Simulate dropping until we can't go further
    let mut ghost_y = transform.translation.y;
    while board_check_block_position(&mut game_data, transform.translation.x, ghost_y - 25.0, block) {
        ghost_y -= 25.0;
    }

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

fn pause_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyP) {
        return;
    }

    match game_state.get() {
        GameState::Playing => next_state.set(GameState::Paused),
        GameState::Paused => next_state.set(GameState::Playing),
        _ => {}
    }
}

fn pause_display_system(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            PauseOverlay,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSED"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("Press P to resume"),
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                Node {
                    margin: bevy::ui::UiRect::top(Val::Px(30.0)),
                    ..default()
                },
            ));
        });
}

fn unpause_cleanup_system(
    mut commands: Commands,
    overlay: Query<Entity, With<PauseOverlay>>,
) {
    for entity in overlay.iter() {
        commands.entity(entity).despawn_recursive();
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
