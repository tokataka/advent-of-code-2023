fn hash(hash_string: &str) -> usize {
    let mut current_value = 0;

    for ch in hash_string.chars() {
        current_value += ch as usize;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

pub fn solution(lines: Vec<&str>) -> String {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    for operation in lines[0].split(',') {
        if operation.contains('=') {
            let (label, focal_length) = operation.split_once('=').unwrap();
            let focal_length = focal_length.parse::<usize>().unwrap();

            let target_box = &mut boxes[hash(label)];

            match target_box.iter().position(|x| x.0 == label) {
                Some(idx) => target_box[idx].1 = focal_length,
                None => target_box.push((label, focal_length)),
            }
        } else {
            let (label, _) = operation.split_once('-').unwrap();

            let target_box = &mut boxes[hash(label)];

            if let Some(idx) = target_box.iter().position(|x| x.0 == label) {
                target_box.remove(idx);
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_idx, slots)| {
            (box_idx + 1)
                * slots
                    .iter()
                    .enumerate()
                    .map(|(slot_idx, (_, focal_length))| (slot_idx + 1) * focal_length)
                    .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}
