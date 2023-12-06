pub fn solution(lines: Vec<&str>) -> String {
    let mut data: Vec<Vec<u32>> = vec![];

    for line in lines {
        let (_, line_data) = line.split_once(":").unwrap();
        data.push(
            line_data
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>(),
        )
    }

    let mut result = 1;

    for (time, distance) in data[0].iter().zip(data[1].iter()) {
        let mut min_time = 0;
        for charge_time in 1..(*time / 2) {
            if charge_time * (*time - charge_time) > *distance {
                min_time = charge_time;
                break;
            }
        }

        result *= *time + 1 - 2 * min_time;
    }

    result.to_string()
}
