use std::collections::HashSet;

pub fn solution(lines: Vec<&str>) -> String {
    let mut match_counts = vec![];

    for line in lines {
        let (_, numbers) = line.split_once(": ").unwrap();

        let (winning_numbers, picked_numbers) = numbers.split_once(" | ").unwrap();

        let mut winning_numbers_set = HashSet::new();
        let mut picked_numbers_set = HashSet::new();

        for i in 0..((winning_numbers.len() + 1) / 3) {
            if let Ok(number) = winning_numbers[i * 3..i * 3 + 2].parse::<i32>() {
                winning_numbers_set.insert(number);
            } else {
                winning_numbers_set
                    .insert(winning_numbers[i * 3 + 1..i * 3 + 2].parse::<i32>().unwrap());
            }
        }

        for i in 0..((picked_numbers.len() + 1) / 3) {
            if let Ok(number) = picked_numbers[i * 3..i * 3 + 2].parse::<i32>() {
                picked_numbers_set.insert(number);
            } else {
                picked_numbers_set
                    .insert(picked_numbers[i * 3 + 1..i * 3 + 2].parse::<i32>().unwrap());
            }
        }

        let intersection = &winning_numbers_set & &picked_numbers_set;

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
