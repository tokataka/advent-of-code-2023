use std::collections::HashSet;

pub fn solution(lines: Vec<&str>) -> String {
    let mut sum = 0;

    for line in lines {
        let (first, second) = line.split_at(line.len() / 2);

        let first_chars: HashSet<char> = HashSet::from_iter(first.chars());
        let second_chars: HashSet<char> = HashSet::from_iter(second.chars());

        let intersection = &first_chars & &second_chars;

        for c in intersection {
            sum += match c {
                c if c >= 'a' && c <= 'z' => (c as i32 - 'a' as i32) + 1,
                c if c >= 'A' && c <= 'Z' => (c as i32 - 'A' as i32) + 27,
                _ => unreachable!(),
            }
        }
    }

    sum.to_string()
}
