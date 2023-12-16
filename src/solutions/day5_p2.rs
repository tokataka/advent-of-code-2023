pub fn solution(lines: Vec<&str>) -> String {
    let (_, seeds) = lines[0].split_once(':').unwrap();

    let mut result = seeds
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| (x[0], x[1]))
        .collect::<Vec<_>>();

    for chunk in lines[2..].split(|x| x.is_empty()) {
        let mut mapped_result = vec![];

        for line in &chunk[1..] {
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
