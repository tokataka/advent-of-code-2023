use std::collections::HashMap;

struct Next<'a> {
    left: &'a str,
    right: &'a str,
}

pub fn solution(lines: Vec<&str>) -> String {
    let mut it = lines.iter();

    let instructions = if let Some(line) = it.next() {
        line.chars().collect::<Vec<_>>()
    } else {
        unreachable!()
    };

    it.next();

    let mut map = HashMap::new();

    while let Some(line) = it.next() {
        let (key, left, right) = (&line[0..3], &line[7..10], &line[12..15]);
        map.insert(key, Next { left, right });
    }

    let mut count = 0;
    let mut cur = "AAA";

    loop {
        let next = match instructions[count % instructions.len()] {
            'L' => map[cur].left,
            'R' => map[cur].right,
            _ => unreachable!(),
        };

        count += 1;

        if next.eq("ZZZ") {
            break;
        }

        cur = next;
    }

    count.to_string()
}
