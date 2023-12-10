use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North = 0b0001,
    South = 0b0010,
    West = 0b0100,
    East = 0b1000,
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Vertical = 0b0011,
    Horizontal = 0b1100,
    NorthEast = 0b1001,
    NorthWest = 0b0101,
    SouthEast = 0b1010,
    SouthWest = 0b0110,
    Empty = 0b0000,
    Start = 0b1111,
}

use Direction::*;
use Tile::*;

impl Direction {
    fn to_offset(&self) -> (isize, isize) {
        match self {
            North => (-1, 0),
            South => (1, 0),
            West => (0, -1),
            East => (0, 1),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
}

impl Tile {
    fn has(&self, direction: &Direction) -> bool {
        *self as usize & *direction as usize > 0
    }

    fn directions(&self) -> Vec<Direction> {
        [North, South, West, East]
            .into_iter()
            .filter(|x| *self as usize & *x as usize > 0)
            .collect::<Vec<_>>()
    }
}

pub fn solution(lines: Vec<&str>) -> String {
    let mut map: Vec<Vec<Tile>> = vec![];

    let mut start = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        map.push(
            line.chars()
                .enumerate()
                .map(|(j, x)| match x {
                    '.' => Empty,
                    '|' => Vertical,
                    '-' => Horizontal,
                    'L' => NorthEast,
                    'J' => NorthWest,
                    '7' => SouthWest,
                    'F' => SouthEast,
                    'S' => {
                        start = (i, j);
                        Start
                    }
                    _ => unreachable!(),
                })
                .collect(),
        );
    }

    let mut visited = HashSet::from([start]);
    let mut next = Some(start);

    while let Some((cur_i, cur_j)) = next {
        next = None;

        for cur_direction in map[cur_i][cur_j].directions() {
            let (cur_di, cur_dj) = cur_direction.to_offset();
            let next_i = (cur_i as isize + cur_di) as usize;
            let next_j = (cur_j as isize + cur_dj) as usize;

            if !map[next_i][next_j].has(&cur_direction.opposite()) {
                continue;
            }

            if visited.contains(&(next_i, next_j)) {
                continue;
            }

            visited.insert((next_i, next_j));
            next = Some((next_i, next_j));
            break;
        }
    }

    (visited.len() / 2).to_string()
}
