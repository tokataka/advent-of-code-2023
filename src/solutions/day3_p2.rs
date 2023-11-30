use std::collections::HashSet;

pub fn solution(lines: Vec<&str>) -> String {
    let mut sum = 0;

    for chunk in lines.chunks_exact(3) {
        let (first, second, third) = (chunk[0], chunk[1], chunk[2]);

        let first_chars: HashSet<char> = HashSet::from_iter(first.chars());
        let second_chars: HashSet<char> = HashSet::from_iter(second.chars());
        let third_chars: HashSet<char> = HashSet::from_iter(third.chars());

        let intersection = &first_chars & &second_chars;
        let intersection = &intersection & &third_chars;

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
