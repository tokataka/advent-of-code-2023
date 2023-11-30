use std::collections::HashSet;

pub fn solution(lines: Vec<&str>) -> String {
    let (mut head_x, mut head_y, mut tail_x, mut tail_y) = (0, 0, 0, 0);
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
            head_x += direction.0;
            head_y += direction.1;

            if i32::abs(head_x - tail_x) <= 1 && i32::abs(head_y - tail_y) <= 1 {
                continue;
            }

            tail_x = head_x - direction.0;
            tail_y = head_y - direction.1;

            visited.insert((tail_x, tail_y));
        }
    }

    visited.len().to_string()
}
