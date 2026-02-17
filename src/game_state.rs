use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::prelude::{
    BuildChildren, ChildBuild, Commands, DespawnRecursiveExt, Entity, KeyCode, NextState, Query,
    Res, ResMut, State, Text, With, Without,
};
use bevy::text::{TextColor, TextFont};
use bevy::ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, PositionType, Val};
use bevy::utils::default;

use crate::board::BoardDot;
use crate::common_component::{
    ActiveBlock, DropType, GameData, GameOverOverlay, GameState, GhostDot, HoldDot, LevelText,
    LinesText, PauseOverlay, PreviewDot, ScoreText,
};
use crate::ghost::GhostTracker;
use crate::hold::HoldTracker;
use crate::spawn_block_system::Randomizer7Bag;

#[allow(clippy::type_complexity)]
pub(crate) fn update_score_display(
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

pub(crate) fn game_over_display_system(mut commands: Commands, game_data: Res<GameData>) {
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

#[allow(clippy::too_many_arguments)]
pub(crate) fn restart_system(
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
    mut ghost_tracker: ResMut<GhostTracker>,
    mut hold_tracker: ResMut<HoldTracker>,
) {
    if !keyboard_input.just_pressed(KeyCode::Enter) {
        return;
    }

    *game_data = GameData::default();
    *randomizer = Randomizer7Bag::default();
    *ghost_tracker = GhostTracker::default();
    *hold_tracker = HoldTracker::default();

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

pub(crate) fn pause_system(
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

pub(crate) fn pause_display_system(mut commands: Commands) {
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

pub(crate) fn unpause_cleanup_system(
    mut commands: Commands,
    overlay: Query<Entity, With<PauseOverlay>>,
) {
    for entity in overlay.iter() {
        commands.entity(entity).despawn_recursive();
    }
}