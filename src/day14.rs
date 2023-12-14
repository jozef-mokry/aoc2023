use rustc_hash::FxHashMap;
pub fn solve() {
    println!("---Day 14---");
    let input = std::fs::read_to_string("data/14.txt").unwrap();
    let mut grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    push_up(&mut grid);
    println!("Part 1: {}", calc_score(&grid));

    push_left(&mut grid);
    push_down(&mut grid);
    push_right(&mut grid);
    let mut cycles = 1;
    let mut seen: FxHashMap<Vec<Vec<char>>, usize> = Default::default();
    let target_cycles = 1_000_000_000;
    let mut loop_found = false;
    while cycles != target_cycles {
        push_up(&mut grid);
        push_left(&mut grid);
        push_down(&mut grid);
        push_right(&mut grid);
        cycles += 1;

        if let Some(prev_cycles) = seen.get(&grid) {
            if !loop_found {
                cycles = target_cycles - ((target_cycles - cycles) % (cycles - prev_cycles));
                loop_found = true;
            }
        }
        if !loop_found {
            seen.insert(grid.clone(), cycles);
        }
    }
    println!("Part 2: {}", calc_score(&grid));
}

fn calc_score(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, row)| (grid.len() - i) * row.iter().filter(|cell| **cell == 'O').count())
        .sum::<usize>()
}

fn push_up(grid: &mut [Vec<char>]) {
    let mut next_row = vec![0; grid[0].len()];
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            match grid[r][c] {
                '.' => {}
                '#' => {
                    next_row[c] = r + 1;
                }
                'O' => {
                    grid[r][c] = '.';
                    grid[next_row[c]][c] = 'O';
                    next_row[c] += 1;
                }
                _ => panic!(),
            };
        }
    }
}
fn push_down(grid: &mut [Vec<char>]) {
    let mut next_row = vec![grid[0].len() - 1; grid[0].len()];
    for r in (0..grid.len()).rev() {
        for c in 0..grid[0].len() {
            match grid[r][c] {
                '.' => {}
                '#' => {
                    if r != 0 {
                        next_row[c] = r - 1;
                    }
                }
                'O' => {
                    grid[r][c] = '.';
                    grid[next_row[c]][c] = 'O';
                    if r != 0 {
                        next_row[c] -= 1;
                    }
                }
                _ => panic!(),
            };
        }
    }
}

fn push_left(grid: &mut [Vec<char>]) {
    for r in 0..grid.len() {
        let mut next_col = 0;
        for c in 0..grid[0].len() {
            match grid[r][c] {
                '.' => {}
                '#' => {
                    next_col = c + 1;
                }
                'O' => {
                    grid[r][c] = '.';
                    grid[r][next_col] = 'O';
                    next_col += 1;
                }
                _ => panic!(),
            };
        }
    }
}

fn push_right(grid: &mut [Vec<char>]) {
    for r in 0..grid.len() {
        let mut next_col = grid[0].len() - 1;
        for c in (0..grid[0].len()).rev() {
            match grid[r][c] {
                '.' => {}
                '#' => {
                    if c != 0 {
                        next_col = c - 1;
                    }
                }
                'O' => {
                    grid[r][c] = '.';
                    grid[r][next_col] = 'O';
                    if c != 0 {
                        next_col -= 1;
                    }
                }
                _ => panic!(),
            };
        }
    }
}
