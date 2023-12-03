struct Number {
    line: usize,
    start: usize,
    end: usize,
    value: i32,
}

struct Symbol {
    line: usize,
    loc: usize,
}

pub fn solution(lines: Vec<&str>) -> String {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    for (i, line) in lines.iter().enumerate() {
        let line: Vec<char> = line.chars().collect();

        let mut j = 0;
        while j < line.len() {
            match *line.get(j).unwrap() {
                '.' => {}
                ch if ch >= '0' && ch <= '9' => {
                    let mut num_string = ch.to_string();

                    let mut end = j + 1;
                    while end < line.len() {
                        num_string.push(*line.get(end).unwrap());
                        if let Err(_) = num_string.parse::<i32>() {
                            num_string.pop();
                            break;
                        }

                        end += 1;
                    }

                    numbers.push(Number {
                        line: i,
                        start: j,
                        end: end - 1,
                        value: num_string.parse::<i32>().unwrap(),
                    });

                    j = end - 1;
                }
                _ => {
                    symbols.push(Symbol { line: i, loc: j });
                }
            }

            j += 1;
        }
    }

    let mut sum = 0;

    for number in &numbers {
        for symbol in &symbols {
            if symbol.line + 1 >= number.line
                && symbol.line <= number.line + 1
                && symbol.loc + 1 >= number.start
                && symbol.loc <= number.end + 1
            {
                sum += number.value;
                break;
            }
        }
    }

    sum.to_string()
}
