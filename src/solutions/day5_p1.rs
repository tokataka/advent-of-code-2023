pub fn solution(lines: Vec<&str>) -> String {
    let mut line_iter = lines.iter();

    // seeds
    let mut result = line_iter.next().unwrap()[7..]
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // skip empty line
    line_iter.next();

    for _ in 0..7 {
        // skip map header
        line_iter.next();

        let mut found = vec![false; result.len()];

        while let Some(line) = line_iter.next() {
            if line == &"" {
                break;
            }

            let map = line
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let (dest, src, range) = (map[0], map[1], map[2]);

            for (item, found) in result.iter_mut().zip(found.iter_mut()) {
                if !(*found) && *item >= src && *item < src + range {
                    *item = *item + dest - src; // cannot use AddAssign since `dest - src` may be negative.
                    *found = true;
                }
            }
        }
    }

    result.iter().min().unwrap().to_string()
}
