pub fn solution(lines: Vec<&str>) -> String {
    let mut result = 0;

    for hash_string in lines[0].split(',') {
        let mut current_value = 0;

        for ch in hash_string.chars() {
            current_value += ch as usize;
            current_value *= 17;
            current_value %= 256;
        }

        result += current_value;
    }

    result.to_string()
}
