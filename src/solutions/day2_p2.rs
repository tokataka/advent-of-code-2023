pub fn solution(lines: Vec<&str>) -> String {
    let mut sum = 0;

    for line in lines {
        let (_, subsets) = line.split_once(": ").unwrap();

        let (mut red, mut green, mut blue) = (0, 0, 0);

        for subset in subsets.split("; ") {
            for cube in subset.split(", ") {
                let (count, color) = cube.split_once(" ").unwrap();
                match (count.parse::<i32>().unwrap(), color) {
                    (n, "red") if n > red => red = n,
                    (n, "green") if n > green => green = n,
                    (n, "blue") if n > blue => blue = n,
                    _ => {}
                }
            }
        }

        sum += red * green * blue;
    }

    sum.to_string()
}
