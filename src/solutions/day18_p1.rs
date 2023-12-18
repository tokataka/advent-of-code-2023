pub fn solution(lines: Vec<&str>) -> String {
    let mut dig_edges: Vec<(i64, i64)> = vec![(0, 0)];
    let mut border_length = 0;
    let (mut cur_i, mut cur_j) = (0, 0);

    for line in lines {
        let mut dig_plan = line.split(' ');
        let direction = dig_plan.next().unwrap();
        let meters = dig_plan.next().unwrap().parse::<i64>().unwrap();

        let (offset_i, offset_j) = match direction {
            "U" => (-1, 0),
            "D" => (1, 0),
            "L" => (0, -1),
            "R" => (0, 1),
            _ => unreachable!(),
        };

        cur_i += offset_i * meters;
        cur_j += offset_j * meters;

        dig_edges.push((cur_i, cur_j));

        border_length += meters;
    }

    let mut area: i64 = 0;

    for ((a_i, a_j), (b_i, b_j)) in dig_edges.iter().zip(&dig_edges[1..]) {
        area += a_i * b_j - b_i * a_j;
    }

    (area.abs() / 2 + border_length / 2 + 1).to_string()
}
