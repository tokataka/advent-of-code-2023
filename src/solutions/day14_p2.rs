fn rotate_clockwise(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_platform = vec![];
    for old_col in 0..platform[0].len() {
        let mut row = vec![];
        for old_row in (0..platform.len()).rev() {
            row.push(platform[old_row][old_col]);
        }
        new_platform.push(row);
    }

    new_platform
}

fn get_load(platform: &Vec<Vec<char>>) -> usize {
    let mut load = 0;

    for (i, row) in platform.iter().enumerate() {
        for ch in row {
            match ch {
                '.' => {}
                'O' => {
                    load += platform.len() - i;
                }
                '#' => {}
                _ => unreachable!(),
            }
        }
    }

    load
}

pub fn solution(lines: Vec<&str>) -> String {
    let mut platform: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut states: Vec<Vec<Vec<char>>> = vec![];

    for cycle in 1..=1000000000 {
        for _ in 0..4 {
            let height = platform.len();
            let width = platform[0].len();

            let mut available_row = vec![0; width];

            for row in 0..height {
                for col in 0..width {
                    match platform[row][col] {
                        '.' => {}
                        'O' => {
                            platform[row][col] = '.';
                            platform[available_row[col]][col] = 'O';
                            available_row[col] += 1;
                        }
                        '#' => {
                            available_row[col] = row + 1;
                        }
                        _ => unreachable!(),
                    }
                }
            }

            platform = rotate_clockwise(&platform);
        }

        if let Some(idx) = states.iter().position(|x| x == &platform) {
            let cycles_remain = 1000000000 - cycle;
            let idx = idx + cycles_remain % (states.len() - idx);

            return get_load(&states[idx]).to_string();
        }

        states.push(platform.clone());
    }

    get_load(&platform).to_string()
}
