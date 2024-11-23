use bevy::prelude::PluginGroup;
use bevy::window::Window;
use bevy::{app::App, window::WindowPlugin, DefaultPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hello Bevy!".to_string(),
                resolution: (800.0, 600.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();
}
