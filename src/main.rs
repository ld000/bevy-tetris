use bevy::app::{PreStartup, Update};
use bevy::color::palettes::css::{BLACK, WHITE};
use bevy::color::Color;
#[cfg(feature = "bevy_dev_tools")]
use bevy::prelude::{info_once, ButtonInput, KeyCode, Res, ResMut};
use bevy::prelude::{BuildChildren, Camera2dBundle, Commands, NodeBundle, PluginGroup};
use bevy::ui::{
    AlignItems, BackgroundColor, BorderColor, Display, FlexDirection, JustifyContent, PositionType,
    Style, UiRect, Val,
};
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
    .add_systems(PreStartup, setup_screen);

    #[cfg(feature = "bevy_dev_tools")]
    {
        app.add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
            .add_systems(Update, toggle_overlay);
    }

    app.run();
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

const BACKGROUND_COLOR: Color = Color::srgb(62.0 / 255.0, 209.0 / 255.0, 185.0 / 255.0);

fn setup_screen(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        margin: UiRect {
                            top: Val::Px(50.0),
                            ..default()
                        },
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Start,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    background_color: BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(80.0),
                            height: Val::Px(50.0),
                            // border: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(BACKGROUND_COLOR),
                        ..default()
                    });
                    // grid
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(266.0),
                                height: Val::Px(516.0),
                                border: UiRect::all(Val::Px(8.0)),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                position_type: PositionType::Relative,
                                ..default()
                            },
                            border_color: BorderColor(BACKGROUND_COLOR),
                            ..default()
                        })
                        .with_children(|parent| {
                            (0..19).for_each(|i| {
                                parent.spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(250.0),
                                        border: UiRect {
                                            top: Val::Px(1.0),
                                            ..default()
                                        },
                                        margin: UiRect {
                                            top: Val::Px(24.0),
                                            ..default()
                                        },
                                        position_type: PositionType::Absolute,
                                        top: Val::Px(25.0 * i as f32),
                                        ..default()
                                    },
                                    border_color: WHITE.into(),
                                    ..default()
                                });
                            });
                            (1..10).for_each(|i| {
                                parent.spawn(NodeBundle {
                                    style: Style {
                                        height: Val::Px(500.0),
                                        border: UiRect {
                                            right: Val::Px(1.0),
                                            ..default()
                                        },
                                        margin: UiRect {
                                            right: Val::Px(24.0),
                                            ..default()
                                        },
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(25.0 * i as f32),
                                        ..default()
                                    },
                                    border_color: WHITE.into(),
                                    ..default()
                                });
                            });
                        });
                    // right
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(80.0),
                            height: Val::Px(500.0),
                            // border: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(BACKGROUND_COLOR),
                        ..default()
                    });
                });
        });
}
