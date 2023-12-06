pub fn solution(lines: Vec<&str>) -> String {
    let mut line_iter = lines.iter();

    // seeds
    let mut result = line_iter.next().unwrap()[7..]
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| (x[0], x[1]))
        .collect::<Vec<_>>();

    // skip empty line
    line_iter.next();

    for _ in 0..7 {
        // skip map header
        line_iter.next();

        let mut mapped_result = vec![];

        while let Some(line) = line_iter.next() {
            if line == &"" {
                break;
            }

            let map = line
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let (dest, src, range) = (map[0], map[1], map[2]);

            let mut next = vec![];

            for (item, item_range) in result {
                if item_range == 0 {
                    continue;
                }

                let start = src.max(item);
                let end = (src + range).min(item + item_range);

                if end <= start {
                    next.push((item, item_range));
                    continue;
                }

                mapped_result.push((start + dest - src, end - start));

                if item < src {
                    next.push((item, src - item));
                }

                if item + item_range > src + range {
                    next.push((src + range, (item + item_range) - (src + range)));
                }
            }

            result = next;
        }

        for (item, item_range) in result {
            mapped_result.push((item, item_range));
        }

        result = mapped_result;
    }

    result.iter().map(|x| x.0).min().unwrap().to_string()
}
