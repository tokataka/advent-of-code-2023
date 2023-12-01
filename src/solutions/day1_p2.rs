pub fn solution(lines: Vec<&str>) -> String {
    let mut sum = 0;

    let map = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for line in lines {
        let mut number = 0;

        'outer: for cur in 0..line.len() {
            if let Ok(n) = line[cur..cur + 1].parse::<i32>() {
                number += 10 * n;
                break;
            } else {
                for (s, n) in map {
                    if line[cur..].starts_with(s) {
                        number += 10 * n;
                        break 'outer;
                    }
                }
            }
        }

        'outer: for cur in (1..line.len() + 1).rev() {
            if let Ok(n) = line[cur - 1..cur].parse::<i32>() {
                number += n;
                break;
            } else {
                for (s, n) in map {
                    if line[..cur].ends_with(s) {
                        number += n;
                        break 'outer;
                    }
                }
            }
        }

        sum += number;
    }

    sum.to_string()
}
