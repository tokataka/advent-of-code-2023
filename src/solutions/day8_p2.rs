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

    let mut cur = vec![];

    while let Some(line) = it.next() {
        let (key, left, right) = (&line[0..3], &line[7..10], &line[12..15]);
        map.insert(key, Next { left, right });
        if key.chars().nth(2).unwrap() == 'A' {
            cur.push(key);
        }
    }

    let mut count = 0;

    let mut z_found_counts = vec![0; cur.len()];

    loop {
        let mut next = vec![];

        for c in cur {
            next.push(match instructions[count % instructions.len()] {
                'L' => map[c].left,
                'R' => map[c].right,
                _ => unreachable!(),
            });
        }

        count += 1;

        for (i, x) in next.iter().enumerate() {
            if x.chars().nth(2).unwrap().eq(&'Z') {
                z_found_counts[i] = count;
            }
        }

        if z_found_counts.iter().all(|x| *x > 0) {
            break;
        }

        cur = next;
    }

    let mut common_mul = 1;

    for x in z_found_counts {
        if common_mul == 1 {
            common_mul = x;
            continue;
        }

        for i in 1.. {
            if (i * common_mul) % x == 0 {
                common_mul *= i;
                break;
            }
        }
    }

    common_mul.to_string()
}
