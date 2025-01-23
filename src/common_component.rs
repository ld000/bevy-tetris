use bevy::prelude::{Component, States};

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
