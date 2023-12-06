use std::collections::HashSet;

pub fn solution(lines: Vec<&str>) -> String {
    let mut sum = 0;

    for line in lines {
        let (_, numbers) = line.split_once(": ").unwrap();

        let (winning_numbers, picked_numbers) = numbers.split_once(" | ").unwrap();

        let winning_numbers: HashSet<i32> = HashSet::from_iter(
            winning_numbers
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap()),
        );

        let picked_numbers: HashSet<i32> = HashSet::from_iter(
            picked_numbers
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap()),
        );

        let intersection = &winning_numbers & &picked_numbers;

        if intersection.len() > 0 {
            sum += i32::pow(2, intersection.len() as u32 - 1);
        }
    }

    sum.to_string()
}
