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
    let mut seen = vec![vec![0; grid.len() * grid[0].len()]; max_steps];
    let n_col = grid[0].len();

    let mut queue: BinaryHeap<(
        std::cmp::Reverse<usize>,
        isize,
        isize,
        ((isize, isize), usize),
    )> = BinaryHeap::new();
    queue.push((std::cmp::Reverse(0), 0, 1, (RIGHT, 1)));
    queue.push((std::cmp::Reverse(0), 1, 0, (DOWN, 1)));

    while let Some((std::cmp::Reverse(score), row, col, (dir, step_count))) = queue.pop() {
        let dir_id = dir_id(dir);
        if step_count == max_steps + 1
            || row as usize >= grid.len()
            || row < 0
            || col as usize >= grid[0].len()
            || col < 0
            || seen[step_count - 1][row as usize * n_col + col as usize] & dir_id != 0
        {
            continue;
        }

        let new_score =
            std::cmp::Reverse(score + (grid[row as usize][col as usize] - b'0') as usize);

        if step_count >= min_steps
            && row as usize + 1 == grid.len()
            && col as usize + 1 == grid[0].len()
        {
            return new_score.0;
        }

        seen[step_count - 1][row as usize * n_col + col as usize] |= dir_id;

        let (row_dir, col_dir) = dir;

        queue.push((
            new_score,
            row + row_dir,
            col + col_dir,
            (dir, step_count + 1),
        ));
        if step_count >= min_steps {
            queue.push((
                new_score,
                row + col_dir,
                col + row_dir,
                ((col_dir, row_dir), 1),
            ));
            queue.push((
                new_score,
                row - col_dir,
                col - row_dir,
                ((-col_dir, -row_dir), 1),
            ));
        }
    }
    panic!()
}
fn dir_id(dir: (isize, isize)) -> u8 {
    match dir {
        dir if dir == UP => 1,
        dir if dir == DOWN => 2,
        dir if dir == LEFT => 4,
        dir if dir == RIGHT => 8,
        _ => panic!(),
    }
}
