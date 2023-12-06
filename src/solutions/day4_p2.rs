use std::collections::HashSet;

pub fn solution(lines: Vec<&str>) -> String {
    let mut match_counts = vec![];

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

        match_counts.push(intersection.len());
    }

    let card_num = match_counts.len();

    let mut result_counts = vec![1; card_num];

    for i in 0..card_num {
        for j in (i + 1)..(usize::min(card_num, i + 1 + match_counts[i])) {
            result_counts[j] += result_counts[i]
        }
    }

    result_counts.iter().sum::<i32>().to_string()
}
