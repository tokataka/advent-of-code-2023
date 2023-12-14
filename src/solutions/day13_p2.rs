pub fn solution(lines: Vec<&str>) -> String {
    let mut sum = 0;

    'outer: for pattern in lines.split(|x| x.is_empty()) {
        let height = pattern.len();
        let width = pattern[0].len();

        // vertical
        for mirror_col in 1..=width - 1 {
            if (0..height)
                .map(|row| {
                    (0..mirror_col)
                        .rev()
                        .zip(mirror_col..width)
                        .filter(|(a, b)| {
                            pattern[row].chars().nth(*a) != pattern[row].chars().nth(*b)
                        })
                        .count()
                })
                .sum::<usize>()
                == 1
            {
                sum += mirror_col;
                continue 'outer;
            }
        }

        // horizontal
        for mirror_row in 1..=height - 1 {
            if (0..width)
                .map(|col| {
                    (0..mirror_row)
                        .rev()
                        .zip(mirror_row..height)
                        .filter(|(a, b)| {
                            pattern[*a].chars().nth(col) != pattern[*b].chars().nth(col)
                        })
                        .count()
                })
                .sum::<usize>()
                == 1
            {
                sum += mirror_row * 100;
                continue 'outer;
            }
        }
    }

    sum.to_string()
}
