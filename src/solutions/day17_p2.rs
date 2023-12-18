use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

use Direction::*;

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Left => (0, -1),
            Right => (0, 1),
            Up => (-1, 0),
            Down => (1, 0),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        }
    }

    fn turn(&self) -> Vec<Self> {
        match self {
            Left | Right => vec![Up, Down],
            Up | Down => vec![Left, Right],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    i: usize,
    j: usize,
    direction: Direction,
    move_count: u32,
}

impl State {
    fn new(position: (usize, usize), direction: Direction, move_count: u32) -> Self {
        Self {
            i: position.0,
            j: position.1,
            direction,
            move_count,
        }
    }
}

pub fn solution(lines: Vec<&str>) -> String {
    let board: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|ch| ch as u32 - '0' as u32).collect())
        .collect();

    let board_height = board.len();
    let board_width = board[0].len();

    let mut queue: VecDeque<State> = VecDeque::from(vec![
        State::new((0, 1), Right, 1),
        State::new((1, 0), Down, 1),
    ]);

    let mut heart_loss_map: HashMap<State, u32> = HashMap::new();

    while let Some(state) = queue.pop_front() {
        let mut last_heart_loss = 0;

        match state.move_count {
            1 => {
                let mut min_heart_loss = u32::MAX;

                for last_direction in state.direction.turn() {
                    let last_offset = state.direction.opposite().offset();
                    let (last_i, last_j) = (
                        state.i as i32 + last_offset.0,
                        state.j as i32 + last_offset.1,
                    );

                    if last_i >= 0 && last_i >= board_height as i32
                        || last_j < 0
                        || last_j >= board_width as i32
                    {
                        continue;
                    }

                    for move_count in 4..=10 {
                        let maybe_last_state = State::new(
                            (last_i as usize, last_j as usize),
                            last_direction,
                            move_count,
                        );

                        if let Some(heart_loss) = heart_loss_map.get(&maybe_last_state) {
                            if *heart_loss < min_heart_loss {
                                min_heart_loss = *heart_loss;
                            }
                        }
                    }
                }

                if min_heart_loss < u32::MAX {
                    last_heart_loss = min_heart_loss;
                }
            }
            2..=10 => {
                let last_direction = state.direction;

                let last_offset = state.direction.opposite().offset();
                let (last_i, last_j) = (
                    (state.i as i32 + last_offset.0) as usize,
                    (state.j as i32 + last_offset.1) as usize,
                );

                let last_state = State::new(
                    (last_i as usize, last_j as usize),
                    last_direction,
                    state.move_count - 1,
                );

                if let Some(heart_loss) = heart_loss_map.get(&last_state) {
                    last_heart_loss = *heart_loss;
                }
            }
            _ => unreachable!(),
        };

        let cur_heart_loss = board[state.i][state.j] + last_heart_loss;

        let min_heart_loss = if state.move_count > 4 {
            (4..=state.move_count)
                .filter_map(|move_count| {
                    let mut cur_state = state;
                    cur_state.move_count = move_count;
                    heart_loss_map.get(&cur_state)
                })
                .min()
        } else {
            heart_loss_map.get(&state)
        };

        match min_heart_loss {
            Some(min_heart_loss) if cur_heart_loss >= *min_heart_loss => {
                continue;
            }
            Some(_) | None => {
                heart_loss_map.insert(state, cur_heart_loss);
            }
        }

        let mut directions_and_move_counts = vec![];

        if state.move_count < 10 {
            directions_and_move_counts.push((state.direction, state.move_count + 1));
        }

        if state.move_count >= 4 {
            for direction in state.direction.turn() {
                directions_and_move_counts.push((direction, 1));
            }
        }

        for (direction, move_count) in directions_and_move_counts {
            let (offset_i, offset_j) = direction.offset();
            let (next_i, next_j) = (state.i as i32 + offset_i, state.j as i32 + offset_j);
            if next_i < 0
                || next_i >= board_height as i32
                || next_j < 0
                || next_j >= board_width as i32
            {
                continue;
            }

            let (next_i, next_j) = (next_i as usize, next_j as usize);

            queue.push_back(State::new((next_i, next_j), direction, move_count));
        }
    }

    heart_loss_map
        .iter()
        .filter(|(k, _)| k.i == board_height - 1 && k.j == board_width - 1 && k.move_count >= 4)
        .map(|(_, v)| v)
        .min()
        .unwrap()
        .to_string()
}
