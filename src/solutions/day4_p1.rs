use std::collections::HashSet;

pub fn solution(lines: Vec<&str>) -> String {
    let mut sum = 0;

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

        if intersection.len() > 0 {
            sum += i32::pow(2, intersection.len() as u32 - 1);
        }
    }

    sum.to_string()
}
