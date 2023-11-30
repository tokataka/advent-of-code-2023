use std::collections::HashSet;

const MARKER_SIZE: usize = 4;

pub fn solution(lines: Vec<&str>) -> String {
    let line = lines[0];

    for cur in 0..(line.len() - (MARKER_SIZE - 1)) {
        let slice = &line[cur..(cur + MARKER_SIZE)];

        let set: HashSet<char> = HashSet::from_iter(slice.chars());

        if set.len() == MARKER_SIZE {
            return (cur + MARKER_SIZE).to_string();
        }
    }

    "".to_string()
}
