use bevy::prelude::*;

use crate::common_component::{GameState, StartScreenOverlay};

pub fn start_screen_display_system(mut commands: Commands) {
    commands
        .spawn((
            StartScreenOverlay,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("TETRIS"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgba(0.2, 0.8, 1.0, 1.0)),
            ));
            parent.spawn((
                Text::new("Controls"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.9)),
                Node {
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("Left/Right  Move\nUp          Hard Drop\nDown        Soft Drop\nQ/E         Rotate\nC           Hold\nP           Pause"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(12.0)),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("Press Enter to Start"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 0.4, 1.0)),
                Node {
                    margin: UiRect::top(Val::Px(36.0)),
                    ..default()
                },
            ));
        });
}

pub fn start_screen_input_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    overlay: Query<Entity, With<StartScreenOverlay>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Enter) {
        for entity in &overlay {
            commands.entity(entity).despawn_recursive();
        }
        next_state.set(GameState::Playing);
    }
}
