pub fn solution(lines: Vec<&str>) -> String {
    let mut sums = vec![];
    let mut cur_sum = 0;

    for line in lines {
        if line.is_empty() {
            sums.push(cur_sum);

            cur_sum = 0;
            continue;
        }

        cur_sum += line.parse::<i32>().unwrap();
    }

    sums.sort();
    sums.reverse();

    (sums[0] + sums[1] + sums[2]).to_string()
}
