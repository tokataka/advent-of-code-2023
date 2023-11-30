pub fn solution(lines: Vec<&str>) -> String {
    let mut count = 0;

    for line in lines {
        let split: Vec<Vec<i32>> = line
            .split(',')
            .map(|x| x.split('-').map(|x| x.parse::<i32>().unwrap()).collect())
            .collect();

        match split {
            split if split[0][0] <= split[1][1] && split[0][1] >= split[1][0] => count += 1,
            _ => {}
        }
    }

    count.to_string()
}
