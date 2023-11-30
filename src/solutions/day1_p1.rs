pub fn solution(lines: Vec<&str>) -> String {
    let mut max = i32::MIN;
    let mut cur_sum = 0;

    for line in lines {
        if line.is_empty() {
            if cur_sum > max {
                max = cur_sum;
            }

            cur_sum = 0;
            continue;
        }

        cur_sum += line.parse::<i32>().unwrap();
    }

    max.to_string()
}
