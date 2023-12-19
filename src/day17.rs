use std::collections::BinaryHeap;

const UP: (isize, isize) = (-1, 0);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);
const RIGHT: (isize, isize) = (0, 1);

pub fn solve() {
    println!("---Day 17---");

    let input = std::fs::read_to_string("data/17.txt").unwrap();
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();

    println!("Part 1: {}", min_dist(&grid, 1, 3));
    println!("Part 2: {}", min_dist(&grid, 4, 10));
}

fn min_dist(grid: &[&[u8]], min_steps: usize, max_steps: usize) -> usize {
    let mut seen = vec![0; grid.len() * grid[0].len()];
    let n_col = grid[0].len();

    let mut queue: BinaryHeap<(std::cmp::Reverse<usize>, isize, isize, (isize, isize))> =
        Default::default();
    queue.push((std::cmp::Reverse(0), 0, 0, RIGHT));
    queue.push((std::cmp::Reverse(0), 0, 0, DOWN));

    while let Some((std::cmp::Reverse(score), row, col, dir)) = queue.pop() {
        let dir_id = dir_id(dir);
        if row as usize >= grid.len()
            || row < 0
            || col as usize >= grid[0].len()
            || col < 0
            || seen[row as usize * n_col + col as usize] & dir_id != 0
        {
            continue;
        }

        if row as usize + 1 == grid.len() && col as usize + 1 == grid[0].len() {
            return score;
        }

        seen[row as usize * n_col + col as usize] |= dir_id;

        let (row_dir, col_dir) = dir;

        for new_dir in [(col_dir, row_dir), (-col_dir, -row_dir)] {
            let mut new_score = score;
            let mut row = row;
            let mut col = col;
            for steps in 1..=max_steps {
                row += new_dir.0;
                col += new_dir.1;
                if let Some(val) = grid.get(row as usize).and_then(|row| row.get(col as usize)) {
                    new_score += (val - b'0') as usize;
                    if steps >= min_steps {
                        queue.push((std::cmp::Reverse(new_score), row, col, new_dir));
                    }
                } else {
                    break;
                }
            }
        }
    }
    panic!("Solution not found")
}
fn dir_id(dir: (isize, isize)) -> u8 {
    match dir {
        dir if dir == UP => 1,
        dir if dir == DOWN => 1,
        dir if dir == LEFT => 2,
        dir if dir == RIGHT => 2,
        _ => panic!(),
    }
}
