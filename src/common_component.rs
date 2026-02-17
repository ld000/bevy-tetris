use bevy::{
    prelude::{Component, Resource, States},
    time::{Timer, TimerMode},
};

use crate::tetromino;

pub const DOT_SIZE: f32 = 25.0;
pub const BOARD_COLS: usize = 10;
pub const BOARD_ROWS: usize = 20;
pub const SPAWN_X: f32 = -DOT_SIZE * 1.5;
pub const MAX_LOCK_RESETS: u32 = 15;
pub const GRAVITY_FLOOR: f32 = 0.05;

const TIMER_DROP_SECS: f32 = 1.0;
const TIMER_HARD_DROP_SECS: f32 = 0.01;
const TIMER_SOFT_DROP_SECS: f32 = 0.05;
const TIMER_LOCK_DELAY_SECS: f32 = 0.5;

#[derive(Resource)]
pub struct GameData {
    pub board_matrix: [[i8; BOARD_COLS]; BOARD_ROWS],
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
    pub lock_delay_timer: Timer,
    pub lock_delay_active: bool,
    pub lock_move_count: u32,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            board_matrix: [[0; BOARD_COLS]; BOARD_ROWS],
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
            lock_delay_timer: Timer::from_seconds(TIMER_LOCK_DELAY_SECS, TimerMode::Once),
            lock_delay_active: false,
            lock_move_count: 0,
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
    Paused,
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

#[derive(Component)]
pub struct GhostDot;

#[derive(Component)]
pub struct PauseOverlay;
