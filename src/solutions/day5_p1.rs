pub fn solution(lines: Vec<&str>) -> String {
    let (_, seeds) = lines[0].split_once(':').unwrap();

    let mut result = seeds
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for chunk in lines[2..].split(|x| x.is_empty()) {
        let mut found = vec![false; result.len()];

        for line in &chunk[1..] {
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
