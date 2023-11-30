pub fn solution(lines: Vec<&str>) -> String {
    let mut stacks: Vec<Vec<char>> = vec![];

    let mut lines_iter = lines.iter();

    while let Some(line) = lines_iter.next() {
        if line.is_empty() {
            break;
        }

        for (i, chars) in line.chars().collect::<Vec<_>>().chunks(4).enumerate() {
            if stacks.len() <= i {
                stacks.push(vec![]);
            }

            match chars[1] {
                ' ' => continue,
                x if x >= '1' && x <= '9' => continue,
                x if x >= 'A' && x <= 'Z' => {
                    stacks[i].push(x);
                }
                _ => unreachable!(),
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    while let Some(line) = lines_iter.next() {
        let line: Vec<_> = line.split(' ').collect();

        let (count, from, to) = (
            line[1].parse::<usize>().unwrap(),
            line[3].parse::<usize>().unwrap(),
            line[5].parse::<usize>().unwrap(),
        );

        for _ in 0..count {
            let item = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(item);
        }
    }

    let mut result = String::new();

    for stack in &mut stacks {
        match stack.pop() {
            Some(item) => result.push(item),
            _ => {}
        }
    }

    result
}
