pub fn solution(lines: Vec<&str>) -> String {
    let mut board: Vec<Vec<i32>> = vec![];
    let mut max_score = 0;

    for line in lines {
        let mut row = vec![];

        for ch in line.chars() {
            row.push(ch as i32 - '0' as i32);
        }

        board.push(row);
    }

    let board_height = board.len();
    let board_width = board[0].len();

    // down, right, up, left
    let direction: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    for (i, row) in board.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            let mut score = 1;

            for (di, dj) in direction {
                let (mut cur_i, mut cur_j) = (i as i32 + di, j as i32 + dj);

                let mut d_score = 0;

                while cur_i >= 0
                    && cur_i < board_height as i32
                    && cur_j >= 0
                    && cur_j < board_width as i32
                {
                    d_score += 1;

                    if board[cur_i as usize][cur_j as usize] >= *height {
                        break;
                    }

                    cur_i += di;
                    cur_j += dj;
                }

                score *= d_score
            }

            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score.to_string()
}
