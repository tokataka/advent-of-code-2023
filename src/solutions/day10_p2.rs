pub fn solution(lines: Vec<&str>) -> String {
    let mut x = 1;
    let mut cycle = 1;

    let mut crt = String::new();

    for line in lines {
        let (next_cycle, v) = if line.starts_with("noop") {
            (cycle + 1, 0)
        } else {
            (cycle + 2, line[5..].parse::<i32>().unwrap())
        };

        for c in cycle..next_cycle {
            let horizontal_cycle = (c - 1) % 40 + 1;

            if horizontal_cycle >= x && horizontal_cycle <= x + 2 {
                crt.push('#');
            } else {
                crt.push('.');
            }

            if horizontal_cycle == 40 {
                crt.push('\n');
            }
        }

        x += v;

        cycle = next_cycle;
    }

    crt
}
