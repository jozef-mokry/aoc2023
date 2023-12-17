use std::collections::VecDeque;

const UP: (isize, isize) = (-1, 0);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);
const RIGHT: (isize, isize) = (0, 1);

pub fn solve() {
    println!("---Day 16---");
    let input = std::fs::read_to_string("data/16.txt").unwrap();
    let grid: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    println!("Part 1: {}", bfs(&grid, (0, 0), RIGHT));

    let mut ans2 = 0;
    for c in 0..grid[0].len() {
        ans2 = ans2.max(bfs(&grid, (0, c as isize), DOWN));
        ans2 = ans2.max(bfs(&grid, (grid.len() as isize - 1, c as isize), UP));
    }
    for r in 0..grid.len() {
        ans2 = ans2.max(bfs(&grid, (r as isize, 0), RIGHT));
        ans2 = ans2.max(bfs(&grid, (r as isize, grid[0].len() as isize - 1), LEFT));
    }
    println!("Part 2: {ans2}");
}

fn bfs(grid: &[&[u8]], (row, col): (isize, isize), dir: (isize, isize)) -> usize {
    let mut visited: Vec<u8> = vec![0; grid.len() * grid[0].len()];

    let mut queue: VecDeque<(isize, isize, (isize, isize))> = VecDeque::new();
    queue.push_back((row, col, dir));
    let n_cols = grid[0].len();
    while let Some((row, col, dir)) = queue.pop_front() {
        if row < 0
            || col < 0
            || row >= grid.len() as isize
            || col >= grid[0].len() as isize
            || visited[row as usize * n_cols + col as usize] & dir_id(dir) != 0
        {
            continue;
        }
        visited[row as usize * n_cols + col as usize] |= dir_id(dir);

        match grid[row as usize][col as usize] {
            b'.' => {
                let (dir_row, dir_col) = dir;
                queue.push_back((row + dir_row, col + dir_col, dir));
            }
            b'-' if dir == LEFT || dir == RIGHT => {
                let (dir_row, dir_col) = dir;
                queue.push_back((row + dir_row, col + dir_col, dir));
            }
            b'-' => {
                queue.push_back((row, col + 1, RIGHT));
                queue.push_back((row, col - 1, LEFT));
            }
            b'|' if dir == UP || dir == DOWN => {
                let (dir_row, dir_col) = dir;
                queue.push_back((row + dir_row, col + dir_col, dir));
            }
            b'|' => {
                queue.push_back((row + 1, col, DOWN));
                queue.push_back((row - 1, col, UP));
            }
            b'/' => {
                let (dir_row, dir_col) = dir;
                let (dir_row, dir_col) = (dir_col * -1, dir_row * -1);
                queue.push_back((row + dir_row, col + dir_col, (dir_row, dir_col)));
            }
            b'\\' => {
                let (dir_row, dir_col) = dir;
                let (dir_row, dir_col) = (dir_col, dir_row);
                queue.push_back((row + dir_row, col + dir_col, (dir_row, dir_col)));
            }
            _ => {
                panic!();
            }
        }
    }
    visited.iter().filter(|v| **v != 0).count()
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
