pub fn solution(lines: Vec<&str>) -> String {
    let mut sum = 0;

    for line in lines {
        let split: Vec<_> = line.split(' ').collect();
        let (mut opponent, mut result) = (
            split[0].parse::<char>().unwrap() as i32 + 1,
            split[1].parse::<char>().unwrap() as i32,
        );

        opponent -= 'A' as i32;
        result -= 'X' as i32;

        let me = (opponent + result + 1) % 3 + 1;

        sum += me + result * 3;
    }

    sum.to_string()
}
