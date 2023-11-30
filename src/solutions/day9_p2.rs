use std::collections::HashSet;

const ROPE_LENGTH: usize = 10;

pub fn solution(lines: Vec<&str>) -> String {
    let mut rope = [(0, 0); ROPE_LENGTH];

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for line in lines {
        let (direction, count) = line.split_once(' ').unwrap();

        let count = count.parse::<usize>().unwrap();
        let direction = match direction {
            "D" => (0, -1),
            "U" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!(),
        };

        for _ in 0..count {
            rope[0].0 += direction.0;
            rope[0].1 += direction.1;

            for i in 0..(rope.len() - 1) {
                let (head_x, head_y) = rope[i];
                let (tail_x, tail_y) = rope[i + 1];

                let abs_x = i32::abs(head_x - tail_x);
                let abs_y = i32::abs(head_y - tail_y);

                if abs_x <= 1 && abs_y <= 1 {
                    continue;
                }

                let new_tail_x = if abs_x > 1 {
                    head_x - (head_x - tail_x) / abs_x
                } else {
                    head_x
                };

                let new_tail_y = if abs_y > 1 {
                    head_y - (head_y - tail_y) / abs_y
                } else {
                    head_y
                };

                rope[i + 1] = (new_tail_x, new_tail_y);
            }

            visited.insert(rope.last().unwrap().clone());
        }
    }

    visited.len().to_string()
}
