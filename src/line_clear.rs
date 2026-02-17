use std::collections::HashMap;

use bevy::prelude::{Commands, Entity, Query, ResMut, Transform};

use crate::board::BoardDot;
use crate::common_component::GameData;
use crate::drop::gravity_seconds;

pub(crate) fn eliminate_line_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut board_dot_query: Query<((Entity, &mut Transform), &BoardDot)>,
) {
    let mut line_indexs_to_eliminate: Vec<usize> = Vec::new();
    for (i, line) in game_data.board_matrix.iter().enumerate() {
        if line.iter().all(|&x| x == 1) {
            line_indexs_to_eliminate.push(i);
        }
    }
    if line_indexs_to_eliminate.is_empty() {
        return;
    }

    // Award points for line clears
    let lines_count = line_indexs_to_eliminate.len();
    let points = match lines_count {
        1 => 100,
        2 => 300,
        3 => 500,
        4 => 800,
        _ => 0,
    };
    game_data.score += points * game_data.level;
    game_data.lines_cleared += lines_count as u32;
    game_data.level = (game_data.lines_cleared / 10) + 1;
    let new_duration = std::time::Duration::from_secs_f32(gravity_seconds(game_data.level));
    game_data.drop_timer.set_duration(new_duration);

    for index in line_indexs_to_eliminate.iter() {
        game_data.board_matrix[*index] = [0; 10];
        board_dot_query
            .iter()
            .for_each(|((entity, _transform), board_dot)| {
                if board_dot.board_y == *index as i8 {
                    commands.entity(entity).despawn();
                }
            });
    }

    let mut is_reached_dot_line: bool = false;
    let mut line_change_map: HashMap<usize, i8> = HashMap::new();
    for (i, line) in game_data.board_matrix.clone().iter().enumerate() {
        if line.contains(&1) {
            is_reached_dot_line = true;
        }

        if !is_reached_dot_line {
            continue;
        }

        if line.iter().all(|&x| x == 0) {
            eliminate_line_inner(&mut game_data, i);

            (0..i).for_each(|j| {
                if line_indexs_to_eliminate.contains(&j) {
                    return;
                }
                line_change_map
                    .entry(j)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            });
        }
    }

    if !line_change_map.is_empty() {
        board_dot_query
            .iter_mut()
            .for_each(|((entity, mut transform), board_dot)| {
                if line_change_map.contains_key(&(board_dot.board_y as usize)) {
                    let line_change_times = line_change_map[&(board_dot.board_y as usize)];
                    transform.translation.y -= 25.0 * line_change_times as f32;
                    commands.entity(entity).insert(BoardDot {
                        board_x: board_dot.board_x,
                        board_y: board_dot.board_y + line_change_times,
                    });
                }
            });
    }
}

fn eliminate_line_inner(game_data: &mut ResMut<GameData>, i: usize) {
    // no more line to move down, break the recursion
    if game_data.board_matrix[i - 1].iter().all(|&x| x == 0) {
        return;
    }

    game_data.board_matrix[i] = game_data.board_matrix[i - 1];
    game_data.board_matrix[i - 1] = [0; 10];

    // reach the top of the board, break the recursion
    if i - 1 == 0 {
        return;
    }

    eliminate_line_inner(game_data, i - 1);
}
