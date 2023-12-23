use std::collections::VecDeque;
pub fn solve() {
    println!("---Day 21---");
    let input = std::fs::read_to_string("data/21.txt").unwrap();
    let grid: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    let mut start = (0, 0);
    'outer: for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'S' {
                start = (r as isize, c as isize);
                break 'outer;
            }
        }
    }
    let side_len = grid.len();
    let grid = tile(&grid, 3);
    start = (start.0 + side_len as isize, start.1 + side_len as isize);
    let max_steps = 64; // Part 1
    let dist = flood_fill(&grid, start);
    let ans = dist
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|v| v.as_ref())
        .filter(|v| **v <= max_steps && **v % 2 == max_steps % 2)
        .count();
    println!("Part 1: {}", ans);

    // Part 2
    // The below works because the row and column of start do not contain any '#' rocks
    debug_assert!(grid[0].len() == grid.len());
    let mut ans2 = 0;
    let max_steps = 26501365;
    // let max_steps = max_steps_1;
    for r in 1..2 * side_len {
        for c in 1..2 * side_len {
            let r = side_len / 2 + r;
            let c = side_len / 2 + c;

            if let Some(d) = dist[r][c] {
                if d > max_steps {
                    continue;
                }
                let remaining_steps = max_steps - d;
                let reachable_tiles = remaining_steps / side_len;
                if remaining_steps % 2 == 1 && reachable_tiles == 0 {
                    continue;
                }
                if d == 0 {
                    debug_assert!(grid[r][c] == b'S');
                    // dealing with center point
                    if remaining_steps % 2 == 0 {
                        ans2 += 1 + 4 * arithmetic_sum(2, reachable_tiles, 2);
                    } else {
                        ans2 += 4 * arithmetic_sum(1, reachable_tiles, 2);
                    }
                } else if r == grid.len() / 2 || c == grid.len() / 2 {
                    // dealing with cross
                    if remaining_steps % 2 == 0 {
                        ans2 += 2 * arithmetic_sum(1, reachable_tiles + 1, 2)
                            - (reachable_tiles + 2) / 2;
                    } else {
                        ans2 += 2 * arithmetic_sum(2, reachable_tiles + 1, 2)
                            - (reachable_tiles + 1) / 2;
                    }
                } else {
                    // dealing with quadrant
                    if remaining_steps % 2 == 0 {
                        ans2 += arithmetic_sum(1, reachable_tiles + 1, 2);
                    } else {
                        ans2 += arithmetic_sum(2, reachable_tiles + 1, 2);
                    }
                }
            }
        }
    }
    println!("Part 2: {ans2}");
}

fn arithmetic_sum(first_term: usize, up_to: usize, step: usize) -> usize {
    if first_term > up_to {
        return 0;
    }
    let num_of_terms = (up_to - first_term) / step + 1;
    let last_term = first_term + (num_of_terms - 1) * step;
    (num_of_terms * (first_term + last_term)) / 2
}

fn tile(grid: &[&[u8]], n: usize) -> Vec<Vec<u8>> {
    let mut tiled = vec![vec![b'.'; n * grid[0].len()]; n * grid.len()];
    for r in 0..tiled.len() {
        for c in 0..tiled[0].len() {
            tiled[r][c] = grid[r % grid.len()][c % grid[0].len()];
        }
    }
    tiled
}

fn flood_fill(grid: &[Vec<u8>], start: (isize, isize)) -> Vec<Vec<Option<usize>>> {
    let mut dist = vec![vec![None; grid[0].len()]; grid.len()];
    let mut queue: VecDeque<_> = Default::default();
    queue.push_back((start, 0));
    debug_assert!(grid[start.0 as usize][start.1 as usize] == b'S');

    while let Some(((r, c), steps)) = queue.pop_front() {
        if r < 0
            || c < 0
            || r as usize == grid.len()
            || c as usize == grid[0].len()
            || grid[r as usize][c as usize] == b'#'
            || dist[r as usize][c as usize].is_some()
        {
            continue;
        }
        dist[r as usize][c as usize] = Some(steps);
        for (d_row, d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            queue.push_back(((r + d_row, c + d_col), steps + 1));
        }
    }
    dist
}
