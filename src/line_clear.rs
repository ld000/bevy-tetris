use std::collections::HashMap;

use bevy::prelude::{Commands, Entity, Query, ResMut, Transform};

use crate::board::BoardDot;
use crate::common_component::{GameData, BOARD_COLS, BOARD_ROWS, DOT_SIZE};
use crate::drop::gravity_seconds;

pub(crate) fn eliminate_line_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut board_dot_query: Query<((Entity, &mut Transform), &BoardDot)>,
) {
    let mut line_indices_to_eliminate: Vec<usize> = Vec::new();
    for (i, line) in game_data.board_matrix.iter().enumerate() {
        if line.iter().all(|&x| x == 1) {
            line_indices_to_eliminate.push(i);
        }
    }
    if line_indices_to_eliminate.is_empty() {
        return;
    }

    // Award points for line clears
    let lines_count = line_indices_to_eliminate.len();
    let points = line_clear_points(lines_count);
    game_data.score += points * game_data.level;
    game_data.lines_cleared += lines_count as u32;
    game_data.level = (game_data.lines_cleared / 10) + 1;
    let new_duration = std::time::Duration::from_secs_f32(gravity_seconds(game_data.level));
    game_data.drop_timer.set_duration(new_duration);

    for index in line_indices_to_eliminate.iter() {
        game_data.board_matrix[*index] = [0; BOARD_COLS];
        board_dot_query
            .iter()
            .for_each(|((entity, _transform), board_dot)| {
                if board_dot.board_y == *index as i8 {
                    commands.entity(entity).despawn();
                }
            });
    }

    let mut is_reached_dot_line: bool = false;
    let mut empty_lines: Vec<usize> = Vec::new();
    for (i, line) in game_data.board_matrix.iter().enumerate() {
        if line.contains(&1) {
            is_reached_dot_line = true;
        }
        if !is_reached_dot_line {
            continue;
        }
        if line.iter().all(|&x| x == 0) {
            empty_lines.push(i);
        }
    }

    let mut line_change_map: HashMap<usize, i8> = HashMap::new();
    for i in empty_lines.iter().copied() {
        eliminate_line_inner(&mut game_data.board_matrix, i);

        (0..i).for_each(|j| {
            if line_indices_to_eliminate.contains(&j) {
                return;
            }
            line_change_map
                .entry(j)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        });
    }

    if !line_change_map.is_empty() {
        board_dot_query
            .iter_mut()
            .for_each(|((entity, mut transform), board_dot)| {
                if line_change_map.contains_key(&(board_dot.board_y as usize)) {
                    let line_change_times = line_change_map[&(board_dot.board_y as usize)];
                    transform.translation.y -= DOT_SIZE * line_change_times as f32;
                    commands.entity(entity).insert(BoardDot {
                        board_x: board_dot.board_x,
                        board_y: board_dot.board_y + line_change_times,
                    });
                }
            });
    }
}

fn line_clear_points(lines: usize) -> u32 {
    match lines {
        1 => 100,
        2 => 300,
        3 => 500,
        4 => 800,
        _ => 0,
    }
}

fn eliminate_line_inner(board: &mut [[i8; BOARD_COLS]; BOARD_ROWS], i: usize) {
    // no more line to move down, break the recursion
    if board[i - 1].iter().all(|&x| x == 0) {
        return;
    }

    board[i] = board[i - 1];
    board[i - 1] = [0; BOARD_COLS];

    // reach the top of the board, break the recursion
    if i - 1 == 0 {
        return;
    }

    eliminate_line_inner(board, i - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoring_single_line() {
        assert_eq!(line_clear_points(1), 100);
    }

    #[test]
    fn scoring_double() {
        assert_eq!(line_clear_points(2), 300);
    }

    #[test]
    fn scoring_triple() {
        assert_eq!(line_clear_points(3), 500);
    }

    #[test]
    fn scoring_tetris() {
        assert_eq!(line_clear_points(4), 800);
    }

    #[test]
    fn scoring_zero_lines() {
        assert_eq!(line_clear_points(0), 0);
    }

    #[test]
    fn eliminate_inner_shifts_line_down() {
        let mut board = [[0i8; BOARD_COLS]; BOARD_ROWS];
        // Place a filled line at row 17, leave row 18 empty, row 19 empty
        board[17] = [1; BOARD_COLS];
        eliminate_line_inner(&mut board, 18);
        // Row 17 should have moved to row 18
        assert_eq!(board[18], [1; BOARD_COLS]);
        assert_eq!(board[17], [0; BOARD_COLS]);
    }

    #[test]
    fn eliminate_inner_shifts_multiple_lines() {
        let mut board = [[0i8; BOARD_COLS]; BOARD_ROWS];
        // Place filled lines at rows 16 and 17, leave row 18 empty
        board[16] = [1; BOARD_COLS];
        board[17] = [1; BOARD_COLS];
        eliminate_line_inner(&mut board, 18);
        // Both should shift down by 1
        assert_eq!(board[18], [1; BOARD_COLS]);
        assert_eq!(board[17], [1; BOARD_COLS]);
        assert_eq!(board[16], [0; BOARD_COLS]);
    }

    #[test]
    fn eliminate_inner_stops_at_empty_row() {
        let mut board = [[0i8; BOARD_COLS]; BOARD_ROWS];
        // Row 18 has data, row 17 is empty, row 16 has data
        board[18] = [1; BOARD_COLS];
        board[16] = [1; BOARD_COLS];
        // Shift into row 19 (which is empty)
        eliminate_line_inner(&mut board, 19);
        // Row 18 should move to 19, row 17 is empty so recursion stops
        assert_eq!(board[19], [1; BOARD_COLS]);
        assert_eq!(board[18], [0; BOARD_COLS]);
        // Row 16 should be untouched (recursion stopped at empty row 17)
        assert_eq!(board[16], [1; BOARD_COLS]);
    }

    #[test]
    fn line_detection_finds_full_rows() {
        let mut board = [[0i8; BOARD_COLS]; BOARD_ROWS];
        board[19] = [1; BOARD_COLS];
        board[18] = [1; BOARD_COLS];
        // Row 17 is partial
        board[17] = [1, 1, 1, 0, 0, 0, 0, 0, 0, 0];

        let full_lines: Vec<usize> = board
            .iter()
            .enumerate()
            .filter(|(_, line)| line.iter().all(|&x| x == 1))
            .map(|(i, _)| i)
            .collect();

        assert_eq!(full_lines, vec![18, 19]);
    }
}
