pub fn solution(lines: Vec<&str>) -> String {
    let height = lines.len();
    let width = lines[0].len();
    let mut available_row = vec![0; width];

    let mut load = 0;

    for (i, row) in lines.iter().enumerate() {
        for (j, ch) in row.chars().enumerate() {
            match ch {
                '.' => {}
                'O' => {
                    load += height - available_row[j];
                    available_row[j] += 1;
                }
                '#' => {
                    available_row[j] = i + 1;
                }
                _ => unreachable!(),
            }
        }
    }

    load.to_string()
}
