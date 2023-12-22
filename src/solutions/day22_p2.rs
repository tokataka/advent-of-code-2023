use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

impl Cube {
    fn from_str(s: &str) -> Cube {
        let mut s_iter = s.split(',');
        Cube {
            x: s_iter.next().unwrap().parse::<usize>().unwrap(),
            y: s_iter.next().unwrap().parse::<usize>().unwrap(),
            z: s_iter.next().unwrap().parse::<usize>().unwrap(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Brick(usize),
    Empty,
}

use Tile::*;

pub fn solution(lines: Vec<&str>) -> String {
    let mut board: Vec<Vec<Vec<Tile>>> = vec![vec![vec![Empty; 1000]; 10]; 10];
    let mut min_z_map: Vec<(usize, usize)> = vec![];

    let mut label_cubes: Vec<Vec<Cube>> = vec![vec![]; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let (from, to) = line.split_once('~').unwrap();

        let from = Cube::from_str(from);
        let to = Cube::from_str(to);

        for x in from.x..=to.x {
            for y in from.y..=to.y {
                for z in from.z..=to.z {
                    board[x][y][z] = Brick(i);
                    label_cubes[i].push(Cube { x, y, z });
                }
            }
        }

        min_z_map.push((i, from.z.min(to.z)));
    }

    min_z_map.sort_by(|(_, z1), (_, z2)| z1.cmp(z2));

    let mut children: Vec<HashSet<usize>> = vec![HashSet::new(); lines.len()];
    let mut parents: Vec<HashSet<usize>> = vec![HashSet::new(); lines.len()];

    for &(label, _) in &min_z_map {
        let cubes = &label_cubes[label];

        if cubes.iter().any(|cube| cube.z == 1) {
            continue;
        }

        let mut drop_count = 0;

        while cubes.iter().all(|&Cube { x, y, z }| {
            let cur = board[x][y][z - drop_count - 1];

            (cur == Empty || cur == Brick(label)) && z - drop_count - 1 > 0
        }) {
            drop_count += 1;
        }

        for &Cube { x, y, z } in cubes {
            board[x][y][z] = Empty;
            board[x][y][z - drop_count] = Brick(label);

            if let Brick(children_label) = board[x][y][z - drop_count - 1] {
                if label != children_label {
                    children[label].insert(children_label);
                    parents[children_label].insert(label);
                }
            }
        }
    }

    let mut result = 0;

    for label in 0..lines.len() {
        let mut fallen_bricks: HashSet<usize> = HashSet::from([label]);
        let mut queue = vec![label];

        while let Some(label) = queue.pop() {
            for &parent in &parents[label] {
                if children[parent]
                    .iter()
                    .all(|&parent_child| fallen_bricks.contains(&parent_child))
                {
                    fallen_bricks.insert(parent);
                    queue.push(parent);
                }
            }
        }

        result += fallen_bricks.len() - 1;
    }

    result.to_string()
}
