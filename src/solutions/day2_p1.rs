pub fn solution(lines: Vec<&str>) -> String {
    let mut sum = 0;

    for line in lines {
        let split: Vec<_> = line.split(' ').collect();
        let (mut opponent, mut me) = (
            split[0].parse::<char>().unwrap() as i32 + 1,
            split[1].parse::<char>().unwrap() as i32 + 1,
        );

        opponent -= 'A' as i32;
        me -= 'X' as i32;

        let result = (me - opponent + 4) % 3;

        sum += me + result * 3;
    }

    sum.to_string()
}
