pub fn solution(lines: Vec<&str>) -> String {
    let mut data: Vec<u64> = vec![];

    for line in lines {
        let (_, line_data) = line.split_once(":").unwrap();
        data.push(line_data.replace(" ", "").parse::<u64>().unwrap());
    }

    let (time, distance) = &(data[0], data[1]);

    let mut min_time = 0;
    for charge_time in 1..(*time / 2) {
        if charge_time * (*time - charge_time) > *distance {
            min_time = charge_time;
            break;
        }
    }

    let result = *time + 1 - 2 * min_time;

    result.to_string()
}
