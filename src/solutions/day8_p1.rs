pub fn solution(lines: Vec<&str>) -> String {
    let mut board: Vec<Vec<i32>> = vec![];
    let mut is_visible: Vec<Vec<bool>> = vec![];

    for line in lines {
        let mut row = vec![];

        for ch in line.chars() {
            row.push(ch as i32 - '0' as i32);
        }

        board.push(row);
        is_visible.push(vec![false; line.len()]);
    }

    let board_height = board.len();
    let board_width = board[0].len();

    // down, right, up, left (cur_direction, look_direction)
    let direction = [
        ((1, 0), (0, 1)),
        ((0, 1), (-1, 0)),
        ((-1, 0), (0, -1)),
        ((0, -1), (1, 0)),
    ];

    let (mut cur_i, mut cur_j) = (0, 0);

    for ((cur_di, cur_dj), (look_di, look_dj)) in direction {
        while cur_i < board_height as i32 && cur_i >= 0 && cur_j < board_width as i32 && cur_j >= 0
        {
            let (mut look_i, mut look_j) = (cur_i, cur_j);

            let mut max_height = -1;

            while look_i < board_height as i32
                && look_i >= 0
                && look_j < board_width as i32
                && look_j >= 0
            {
                let cur_height = board[look_i as usize][look_j as usize];

                if cur_height > max_height {
                    is_visible[look_i as usize][look_j as usize] = true;
                    max_height = cur_height
                }

                look_i += look_di;
                look_j += look_dj;
            }

            cur_i += cur_di;
            cur_j += cur_dj;
        }

        cur_i -= cur_di;
        cur_j -= cur_dj;
    }

    is_visible
        .iter()
        .map(|i| {
            i.iter()
                .map(|j| match j {
                    false => 0,
                    true => 1,
                })
                .sum::<i32>()
        })
        .sum::<i32>()
        .to_string()
}
