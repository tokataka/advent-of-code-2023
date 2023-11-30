fn decode_snafu(snafu: String) -> i64 {
    let mut sum = 0;
    let mut digit = 1;

    for ch in snafu.chars().rev() {
        sum += match ch {
            '2' => 2 * digit,
            '1' => digit,
            '0' => 0,
            '-' => -digit,
            '=' => -2 * digit,
            _ => unreachable!(),
        };

        digit *= 5;
    }

    sum
}

fn encode_snafu(decimal: i64) -> String {
    let mut snafu = String::new();
    let mut digit = 1;
    let mut cur: i64 = 0;

    let mut decimal = decimal;

    while decimal > digit * 2 {
        digit *= 5;
        cur += 1;
    }

    while cur >= 0 {
        let mut count = 0;

        let next_max = (i64::pow(5, cur as u32) - 1) / 2;

        while decimal > next_max {
            decimal -= digit;
            count += 1;
        }

        while decimal < -next_max {
            decimal += digit;
            count -= 1;
        }

        snafu.push(match count {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => unreachable!(),
        });

        digit /= 5;
        cur -= 1;
    }

    snafu
}

pub fn solution(lines: Vec<&str>) -> String {
    let mut sum: i64 = 0;

    for line in lines {
        sum += decode_snafu(line.to_string());
    }

    encode_snafu(sum)
}
