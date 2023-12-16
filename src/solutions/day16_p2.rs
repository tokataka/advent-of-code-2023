use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Left => (0, -1),
            Right => (0, 1),
            Up => (-1, 0),
            Down => (1, 0),
        }
    }

    fn next_position(
        &self,
        (i, j): (usize, usize),
        (board_height, board_width): (usize, usize),
    ) -> Option<(usize, usize)> {
        let (offset_i, offset_j) = self.offset();
        let (next_i, next_j) = (i as i32 + offset_i, j as i32 + offset_j);

        if next_i >= 0 && next_i < board_height as i32 && next_j >= 0 && next_j < board_width as i32
        {
            return Some((next_i as usize, next_j as usize));
        }

        None
    }
}

use Direction::*;

enum Tile {
    Empty,
    MirrorUpRightToDownLeft,
    MirrorUpLeftToDownRight,
    SplitVertical,
    SplitHorizontal,
}

impl Tile {
    fn encounter(&self, direction: Direction) -> Vec<Direction> {
        match self {
            Empty => vec![direction],
            MirrorUpLeftToDownRight => match direction {
                Left => vec![Up],
                Right => vec![Down],
                Up => vec![Left],
                Down => vec![Right],
            },
            MirrorUpRightToDownLeft => match direction {
                Left => vec![Down],
                Right => vec![Up],
                Up => vec![Right],
                Down => vec![Left],
            },
            SplitHorizontal => match direction {
                Left | Right => vec![direction],
                Up | Down => vec![Left, Right],
            },
            SplitVertical => match direction {
                Left | Right => vec![Up, Down],
                Up | Down => vec![direction],
            },
        }
    }
}

use Tile::*;

pub fn solution(lines: Vec<&str>) -> String {
    let board: Vec<Vec<Tile>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Empty,
                    '/' => MirrorUpRightToDownLeft,
                    '\\' => MirrorUpLeftToDownRight,
                    '|' => SplitVertical,
                    '-' => SplitHorizontal,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let board_height = board.len();
    let board_width = board[0].len();

    let start_list = (0..board_width)
        .map(|j| ((0, j), Down))
        .chain((0..board_width).map(|j| ((board_height - 1, j), Up)))
        .chain((0..board_height).map(|i| ((i, 0), Right)))
        .chain((0..board_height).map(|i| ((i, board_width - 1), Left)));

    let mut max_energized_count = 0;

    for start in start_list {
        let mut queue: Vec<((usize, usize), Direction)> = vec![start];
        let mut visited: HashSet<((usize, usize), Direction)> = HashSet::new();
        let mut energized: HashSet<(usize, usize)> = HashSet::new();

        while let Some((cur, direction)) = queue.pop() {
            if !visited.insert((cur, direction)) {
                continue;
            }

            energized.insert(cur);

            let cur_tile = &board[cur.0][cur.1];

            for next_direction in cur_tile.encounter(direction) {
                if let Some(next) = next_direction.next_position(cur, (board_height, board_width)) {
                    queue.push((next, next_direction));
                }
            }
        }

        let energized_count = energized.len();

        if energized_count > max_energized_count {
            max_energized_count = energized_count;
        }
    }

    max_energized_count.to_string()
}
