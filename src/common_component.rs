use bevy::{
    prelude::{Component, Resource, States},
    time::{Timer, TimerMode},
};

const TIMER_KEYBOARD_SECS: f32 = 0.1;
const TIMER_DROP_SECS: f32 = 1.0;
const TIMER_HARD_DROP_SECS: f32 = 0.01;

#[derive(Resource)]
pub struct GameData {
    pub board_matrix: [[i8; 10]; 20],
    pub keyboard_timer: Timer,
    pub drop_timer: Timer,
    pub hard_drop_timer: Timer,
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

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum DropType {
    #[default]
    Normal,
    Hard,
}

#[derive(Component)]
pub struct ActiveBlock;

#[derive(Component)]
pub struct ActiveDot;
