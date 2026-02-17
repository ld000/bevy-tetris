mod background;
mod board;
mod common_component;
mod drop;
mod game_state;
mod ghost;
mod hold;
mod line_clear;
mod movement;
mod rotation;
mod spawn_block_system;
mod tetromino;

use bevy::app::{PreStartup, Update};
#[cfg(feature = "bevy_dev_tools")]
use bevy::prelude::info_once;
use bevy::prelude::{
    in_state, AppExtStates, Condition, IntoSystemConfigs, PluginGroup, Res,
};
use bevy::utils::default;
use bevy::window::Window;
use bevy::{app::App, window::WindowPlugin, DefaultPlugins};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use common_component::{DropType, GameData, GameState};
use spawn_block_system::{spawn_block_system, update_preview_system, Randomizer7Bag};

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
    .add_systems(Update, game_state::update_score_display)
    .add_systems(Update, hold::update_hold_preview_system)
    .init_resource::<GameData>()
    .init_resource::<ghost::GhostTracker>()
    .init_resource::<hold::HoldTracker>()
    .add_systems(PreStartup, background::setup_background)
    .add_systems(Update, background::setup_background_grid)
    .init_resource::<Randomizer7Bag>()
    .init_state::<DropType>()
    .init_state::<GameState>()
    .add_systems(Update, (spawn_block_system, update_preview_system).chain().run_if(in_state(GameState::Playing)))
    .add_systems(
        Update,
        (rotation::block_rotation_system, movement::block_movement_system, hold::hold_block_system)
            .run_if(in_state(GameState::Playing).and(in_state(DropType::Normal).or(in_state(DropType::Soft)))),
    )
    .add_systems(
        Update,
        (
            drop::block_drop_type_system,
            drop::block_drop_system,
            line_clear::eliminate_line_system,
        )
            .chain()
            .run_if(in_state(GameState::Playing)),
    )
    .add_systems(Update, ghost::update_ghost_piece_system.run_if(in_state(GameState::Playing)))
    .add_systems(Update, game_state::pause_system.run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))))
    .add_systems(bevy::prelude::OnEnter(GameState::Paused), game_state::pause_display_system)
    .add_systems(bevy::prelude::OnExit(GameState::Paused), game_state::unpause_cleanup_system)
    .add_systems(bevy::prelude::OnEnter(GameState::GameOver), game_state::game_over_display_system)
    .add_systems(Update, game_state::restart_system.run_if(in_state(GameState::GameOver)));

    #[cfg(feature = "bevy_dev_tools")]
    {
        app.add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
            .add_systems(Update, toggle_overlay);
    }

    app.run();
}

fn ui_example_system(mut contexts: EguiContexts, game_data: Res<GameData>) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        for line in game_data.board_matrix.iter() {
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

#[cfg(feature = "bevy_dev_tools")]
fn toggle_overlay(
    input: Res<bevy::input::ButtonInput<bevy::prelude::KeyCode>>,
    mut options: bevy::prelude::ResMut<bevy::dev_tools::ui_debug_overlay::UiDebugOptions>,
) {
    info_once!("The debug outlines are enabled, press Space to turn them on/off");
    if input.just_pressed(bevy::prelude::KeyCode::Space) {
        options.toggle();
    }
}
