pub fn solution(lines: Vec<&str>) -> String {
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut is_row_empty: Vec<bool> = vec![true; lines.len()];
    let mut is_col_empty: Vec<bool> = vec![true; lines[0].len()];

    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if let '#' = ch {
                galaxies.push((i, j));
                is_row_empty[i] = false;
                is_col_empty[j] = false;
            }
        }
    }

    for (i, j) in &mut galaxies {
        *i += is_row_empty
            .iter()
            .enumerate()
            .filter(|(xi, x)| xi < i && **x)
            .count();

        *j += is_col_empty
            .iter()
            .enumerate()
            .filter(|(xj, x)| xj < j && **x)
            .count();
    }

    let mut sum = 0;

    for idx1 in 0..galaxies.len() - 1 {
        let (i, j) = galaxies[idx1];

        for idx2 in idx1 + 1..galaxies.len() {
            sum += i.abs_diff(galaxies[idx2].0) + j.abs_diff(galaxies[idx2].1);
        }
    }

    sum.to_string()
}
