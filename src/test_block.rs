use bevy::{
    color::{Gray, LinearRgba},
    math::{Isometry3d, UVec2, Vec2, Vec3},
    prelude::{Commands, Gizmos},
};

use crate::{spawn_block_system::spawn_block, tetromino};

fn test_block(mut commands: Commands) {
    spawn_block(&mut commands, tetromino::Block::new_i(), -300.0, 0.0);
    spawn_block(&mut commands, tetromino::Block::new_o(), -200.0, 0.0);
    spawn_block(&mut commands, tetromino::Block::new_t(), -100.0, 0.0);
    spawn_block(&mut commands, tetromino::Block::new_s(), 0.0, 0.0);
    spawn_block(&mut commands, tetromino::Block::new_z(), 100.0, 0.0);
    spawn_block(&mut commands, tetromino::Block::new_j(), 200.0, 0.0);
    spawn_block(&mut commands, tetromino::Block::new_l(), 300.0, 0.0);
}

fn test_block_gizmos(mut gizmos: Gizmos) {
    block_gizmos(&mut gizmos, -300.0);
    block_gizmos(&mut gizmos, -200.0);
    block_gizmos(&mut gizmos, -100.0);
    block_gizmos(&mut gizmos, 0.0);
    block_gizmos(&mut gizmos, 100.0);
    block_gizmos(&mut gizmos, 200.0);
    block_gizmos(&mut gizmos, 300.0);
}

fn block_gizmos(gizmos: &mut Gizmos, transform_x: f32) {
    gizmos.rect(
        Isometry3d::from_translation(Vec3::new(transform_x, 0.0, 1.0)),
        Vec2::new(100.0, 100.0),
        LinearRgba::gray(0.3),
    );

    gizmos.grid(
        Isometry3d::from_translation(Vec3::new(transform_x, 0.0, 1.0)),
        UVec2::new(4, 4),
        Vec2::new(25.0, 25.0),
        LinearRgba::gray(0.05),
    );
}
