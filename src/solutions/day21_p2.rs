use std::collections::{HashMap, VecDeque};

enum Tile {
    Empty,
    Rock,
}

use Tile::*;

const TARGET_STEPS: i64 = 26501365;
const STEP_COUNTS_MAX_GRID: i64 = 3;

pub fn solution(lines: Vec<&str>) -> String {
    let mut start: (i64, i64) = (0, 0);

    let board: Vec<Vec<Tile>> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| match ch {
                    '.' => Empty,
                    '#' => Rock,
                    'S' => {
                        start = (i as i64, j as i64);
                        Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    // assume board is square...
    let board_size: i64 = board.len() as i64;

    let offsets = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut queue = VecDeque::from_iter([(start, 0)]);

    let mut step_counts: HashMap<(i64, i64), i64> = HashMap::new();

    while let Some(((cur_i, cur_j), steps)) = queue.pop_front() {
        if step_counts.contains_key(&(cur_i, cur_j)) {
            continue;
        }

        if cur_i < -STEP_COUNTS_MAX_GRID * board_size
            || cur_i >= (STEP_COUNTS_MAX_GRID + 1) * board_size
            || cur_j < -STEP_COUNTS_MAX_GRID * board_size
            || cur_j >= (STEP_COUNTS_MAX_GRID + 1) * board_size
        {
            continue;
        }

        step_counts.insert((cur_i, cur_j), steps);

        for (offset_i, offset_j) in offsets {
            let (next_i, next_j) = (cur_i + offset_i, cur_j + offset_j);

            let (board_i, board_j) = (
                match next_i % board_size {
                    x if x < 0 => (x + board_size) as usize,
                    x => x as usize,
                },
                match next_j % board_size {
                    x if x < 0 => (x + board_size) as usize,
                    x => x as usize,
                },
            );

            if let Rock = board[board_i][board_j] {
                continue;
            }

            queue.push_back(((next_i, next_j), steps + 1));
        }
    }

    let mut grid_tile_counts = (0, 0);

    for i in 0..board_size {
        for j in 0..board_size {
            if let Some(current_step) = step_counts.get(&(i, j)) {
                if *current_step % 2 == TARGET_STEPS % 2 {
                    grid_tile_counts.0 += 1;
                } else {
                    grid_tile_counts.1 += 1;
                }
            };
        }
    }

    let mut result = 0;

    let max_grid_size = (TARGET_STEPS as f64 / board_size as f64).ceil() as i64;

    // max_grid_size is too small
    if max_grid_size <= STEP_COUNTS_MAX_GRID {
        for i in -(STEP_COUNTS_MAX_GRID * board_size)..=(STEP_COUNTS_MAX_GRID * board_size) {
            for j in -(STEP_COUNTS_MAX_GRID * board_size)..=(STEP_COUNTS_MAX_GRID * board_size) {
                if let Some(steps) = step_counts.get(&(i, j)) {
                    if *steps <= TARGET_STEPS && *steps % 2 == TARGET_STEPS % 2 {
                        result += 1;
                    }
                }
            }
        }

        return result.to_string();
    }

    // grid_i == 0 || grid_j == 0 line
    for grid in -max_grid_size..=max_grid_size {
        if grid.abs() < max_grid_size - 2 {
            if grid % 2 == 0 {
                result += 2 * grid_tile_counts.0;
            } else {
                result += 2 * grid_tile_counts.1;
            }

            continue;
        }

        let grid_offset = match grid.abs() <= STEP_COUNTS_MAX_GRID {
            true => 0,
            false => grid.signum() * (grid.abs() - STEP_COUNTS_MAX_GRID),
        };

        let grid_base = grid - grid_offset;
        let steps_offset = grid_offset.abs() * board_size;

        for i in 0..board_size {
            for j in grid_base * board_size..(grid_base + 1) * board_size {
                if let Some(steps) = step_counts.get(&(i, j)) {
                    if steps + steps_offset <= TARGET_STEPS
                        && (steps + steps_offset) % 2 == TARGET_STEPS % 2
                    {
                        result += 1;
                    }
                }
            }
        }

        for i in grid_base * board_size..(grid_base + 1) * board_size {
            for j in 0..board_size {
                if let Some(steps) = step_counts.get(&(i, j)) {
                    if steps + steps_offset <= TARGET_STEPS
                        && (steps + steps_offset) % 2 == TARGET_STEPS % 2
                    {
                        result += 1;
                    }
                }
            }
        }
    }

    // subtract grid_i == 0 && grid_j == 0 duplicate
    result -= grid_tile_counts.0;

    // else
    for (grid_base_i, grid_base_j) in [(1, -1), (1, 1), (-1, 1), (-1, -1)] {
        for grid_offset in 0..=max_grid_size - 2 {
            let mut current_result = 0;

            if grid_offset < max_grid_size - 4 {
                if grid_offset % 2 == 0 {
                    current_result += grid_tile_counts.0;
                } else {
                    current_result += grid_tile_counts.1;
                }
            } else {
                let steps_offset = grid_offset * board_size;

                for i in grid_base_i * board_size..(grid_base_i + 1) * board_size {
                    for j in grid_base_j * board_size..(grid_base_j + 1) * board_size {
                        if let Some(steps) = step_counts.get(&(i, j)) {
                            if steps + steps_offset <= TARGET_STEPS
                                && (steps + steps_offset) % 2 == TARGET_STEPS % 2
                            {
                                current_result += 1;
                            }
                        }
                    }
                }
            }

            result += current_result * (grid_offset + 1);
        }
    }

    result.to_string()
}
