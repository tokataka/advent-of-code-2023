use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn directions() -> [Direction; 4] {
        [Up, Down, Left, Right]
    }

    fn offset(&self) -> (i32, i32) {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

use Direction::*;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

use Tile::*;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Debug)]
struct Node {
    i: usize,
    j: usize,
}

impl From<(usize, usize)> for Node {
    fn from(value: (usize, usize)) -> Self {
        Node {
            i: value.0,
            j: value.1,
        }
    }
}

pub fn solution(lines: Vec<&str>) -> String {
    let board: Vec<Vec<Tile>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Path,
                    '#' => Forest,
                    '^' => Slope(Up),
                    'v' => Slope(Down),
                    '<' => Slope(Left),
                    '>' => Slope(Right),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let start = (
        0,
        board
            .first()
            .unwrap()
            .iter()
            .position(|&tile| tile == Path)
            .unwrap(),
    );

    let end = (
        board.len() - 1,
        board
            .last()
            .unwrap()
            .iter()
            .position(|&tile| tile == Path)
            .unwrap(),
    );

    let mut nodes: HashSet<Node> = HashSet::from([start.into(), end.into()]);
    let mut weights: HashMap<(Node, Node), i64> = HashMap::new();

    let mut visited: HashSet<Node> = HashSet::from([start.into()]);
    let mut stack: Vec<(Node, Node, i64, bool)> =
        vec![(start.into(), (start.0 + 1, start.1).into(), 1, false)];

    while let Some((from, cur, steps, is_passed_slope)) = stack.pop() {
        if cur == from {
            continue;
        }

        if cur == end.into() {
            weights.insert((from, cur), steps);
            if !is_passed_slope {
                weights.insert((cur, from), steps);
            }
            continue;
        }

        let next_nodes: Vec<(Node, bool, bool)> = Direction::directions()
            .into_iter()
            .filter_map(|direction| {
                let (di, dj) = direction.offset();
                let next = (
                    cur.i.checked_add_signed(di as isize).unwrap(),
                    cur.j.checked_add_signed(dj as isize).unwrap(),
                );

                match board[next.0][next.1] {
                    Forest => None,
                    Path => Some((next.into(), false, false)),
                    Slope(next_direction) => {
                        Some((next.into(), true, direction == next_direction.opposite()))
                    }
                }
            })
            .collect();

        // cur is not passage
        if next_nodes.len() != 2 {
            weights.insert((from, cur), steps);
            if !is_passed_slope {
                weights.insert((cur, from), steps);
            }
            nodes.insert(cur);
        }

        if visited.contains(&cur.into()) {
            continue;
        }

        visited.insert(cur);

        for &(next, next_is_passed_slope, is_passing_opposite_slope) in &next_nodes {
            if is_passing_opposite_slope {
                continue;
            }

            match next_nodes.len() {
                2 => stack.push((
                    from,
                    next.into(),
                    steps + 1,
                    is_passed_slope || next_is_passed_slope,
                )),
                _ => stack.push((cur, next.into(), 1, next_is_passed_slope)),
            }
        }
    }

    let mut stack: Vec<(Node, HashSet<Node>, i64)> =
        vec![(start.into(), HashSet::from([start.into()]), 0)];

    let mut result = 0;

    while let Some((cur, visited, steps)) = stack.pop() {
        for (next, weight) in weights
            .iter()
            .filter_map(|(&(node_from, node_to), &weight)| match node_from {
                x if x == cur => Some((node_to, weight)),
                _ => None,
            })
        {
            if next == end.into() {
                if steps + weight > result {
                    result = steps + weight;
                    continue;
                }
            }

            if visited.contains(&next) {
                continue;
            }

            let mut next_visited = visited.clone();
            next_visited.insert(next);

            stack.push((next, next_visited, steps + weight));
        }
    }

    result.to_string()
}
