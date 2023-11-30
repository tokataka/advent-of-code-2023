pub fn solution(lines: Vec<&str>) -> String {
    let mut x = 1;
    let mut cycle = 1;
    let mut sum = 0;

    for line in lines {
        let (next_cycle, v) = if line.starts_with("noop") {
            (cycle + 1, 0)
        } else {
            (cycle + 2, line[5..].parse::<i32>().unwrap())
        };

        for c in cycle..next_cycle {
            if c % 40 == 20 {
                sum += c * x;
            }
        }

        x += v;

        cycle = next_cycle;
    }

    sum.to_string()
}
