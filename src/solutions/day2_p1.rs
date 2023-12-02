pub fn solution(lines: Vec<&str>) -> String {
    let mut sum = 0;

    for line in lines {
        let (id, subsets) = line.split_once(": ").unwrap();
        let id = id.split_once(" ").unwrap().1.parse::<i32>().unwrap();

        let check_possible = || {
            for subset in subsets.split("; ") {
                for cube in subset.split(", ") {
                    let (count, color) = cube.split_once(" ").unwrap();
                    match (count.parse::<i32>().unwrap(), color) {
                        (n, "red") if n > 12 => return false,
                        (n, "green") if n > 13 => return false,
                        (n, "blue") if n > 14 => return false,
                        _ => {}
                    }
                }
            }

            true
        };

        if check_possible() {
            sum += id;
        }
    }

    sum.to_string()
}
