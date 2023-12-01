pub fn solution(lines: Vec<&str>) -> String {
    let mut sum = 0;

    for line in lines {
        let mut number = 0;

        for ch in line.chars() {
            if ch >= '0' && ch <= '9' {
                number += 10 * (ch as i32 - '0' as i32);
                break;
            }
        }

        for ch in line.chars().rev() {
            if ch >= '0' && ch <= '9' {
                number += ch as i32 - '0' as i32;
                break;
            }
        }

        sum += number;
    }

    sum.to_string()
}
