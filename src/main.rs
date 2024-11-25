mod background;

use bevy::app::{PreStartup, Update};
use bevy::prelude::PluginGroup;
#[cfg(feature = "bevy_dev_tools")]
use bevy::prelude::{info_once, ButtonInput, KeyCode, Res, ResMut};
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
    .add_systems(PreStartup, background::setup_screen);

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
