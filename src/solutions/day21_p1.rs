use std::collections::{HashMap, VecDeque};

enum Tile {
    Empty,
    Rock,
}

use Tile::*;

const TARGET_STEPS: i64 = 64;

pub fn solution(lines: Vec<&str>) -> String {
    let mut start = (0, 0);
    let mut board: Vec<Vec<Tile>> = vec![];

    board.push((0..lines[0].len() + 2).map(|_| Rock).collect());
    for (i, line) in lines.iter().enumerate() {
        let mut row = vec![Rock];
        row.extend(line.chars().enumerate().map(|(j, ch)| match ch {
            '.' => Empty,
            '#' => Rock,
            'S' => {
                start = (i as i64 + 1, j as i64 + 1);
                Empty
            }
            _ => unreachable!(),
        }));
        row.push(Rock);

        board.push(row);
    }
    board.push((0..lines[0].len() + 2).map(|_| Rock).collect());

    let offsets = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut queue = VecDeque::from_iter([(start, 0)]);

    let mut step_counts: HashMap<(i64, i64), i64> = HashMap::new();

    while let Some(((cur_i, cur_j), steps)) = queue.pop_front() {
        if step_counts.contains_key(&(cur_i, cur_j)) {
            continue;
        }

        step_counts.insert((cur_i, cur_j), steps);

        if steps >= TARGET_STEPS {
            continue;
        }

        for (offset_i, offset_j) in offsets {
            let (next_i, next_j) = (cur_i + offset_i, cur_j + offset_j);

            if let Rock = board[next_i as usize][next_j as usize] {
                continue;
            }

            queue.push_back(((next_i, next_j), steps + 1));
        }
    }

    step_counts
        .iter()
        .filter(|(_, steps)| **steps % 2 == TARGET_STEPS % 2)
        .count()
        .to_string()
}
