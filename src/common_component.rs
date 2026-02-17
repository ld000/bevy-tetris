use bevy::{
    prelude::{Component, Resource, States},
    time::{Timer, TimerMode},
};

use crate::tetromino;

const TIMER_KEYBOARD_SECS: f32 = 0.1;
const TIMER_DROP_SECS: f32 = 1.0;
const TIMER_HARD_DROP_SECS: f32 = 0.01;
const TIMER_SOFT_DROP_SECS: f32 = 0.05;

#[derive(Resource)]
pub struct GameData {
    pub board_matrix: [[i8; 10]; 20],
    pub keyboard_timer: Timer,
    pub drop_timer: Timer,
    pub hard_drop_timer: Timer,
    pub soft_drop_timer: Timer,
    pub soft_drop_cells: u32,
    pub score: u32,
    pub lines_cleared: u32,
    pub hard_drop_start_y: Option<f32>,
    pub held_block: Option<tetromino::Block>,
    pub hold_used: bool,
    pub level: u32,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            board_matrix: [[0; 10]; 20],
            keyboard_timer: Timer::from_seconds(TIMER_KEYBOARD_SECS, TimerMode::Repeating),
            drop_timer: Timer::from_seconds(TIMER_DROP_SECS, TimerMode::Repeating),
            hard_drop_timer: Timer::from_seconds(TIMER_HARD_DROP_SECS, TimerMode::Repeating),
            soft_drop_timer: Timer::from_seconds(TIMER_SOFT_DROP_SECS, TimerMode::Repeating),
            soft_drop_cells: 0,
            score: 0,
            lines_cleared: 0,
            hard_drop_start_y: None,
            held_block: None,
            hold_used: false,
            level: 1,
        }
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum DropType {
    #[default]
    Normal,
    Hard,
    Soft,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Component)]
pub struct ActiveBlock;

#[derive(Component)]
pub struct ActiveDot;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct LinesText;

#[derive(Component)]
pub struct GameOverOverlay;

#[derive(Component)]
pub struct PreviewDot;

#[derive(Component)]
pub struct HoldDot;

#[derive(Component)]
pub struct LevelText;
