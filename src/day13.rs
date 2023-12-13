pub fn solve() {
    println!("---Day 13---");
    let input = std::fs::read_to_string("data/13.txt").unwrap();

    let mut grid = vec![];
    let mut ans = 0;
    let mut ans2 = 0;
    for line in input.lines().chain(std::iter::once("")) {
        if !line.is_empty() {
            grid.push(line.as_bytes());
        } else {
            if let Some(rows) = reflect_rows(&grid[..], 0) {
                ans += 100 * rows;
            } else {
                ans += reflect_cols(&grid[..], 0).unwrap();
            }
            if let Some(rows) = reflect_rows(&grid[..], 1) {
                ans2 += 100 * rows;
            } else {
                ans2 += reflect_cols(&grid[..], 1).unwrap();
            }
            grid.clear();
        }
    }
    println!("Part 1: {ans}");
    println!("Part 2: {ans2}");
}

fn reflect_rows(grid: &[&[u8]], mistakes_allowed: usize) -> Option<usize> {
    for r in 1..grid.len() {
        let mut j = 1;
        let mut mistakes = 0;
        'outer: while r >= j && r + j - 1 < grid.len() {
            for c in 0..grid[0].len() {
                if grid[r - j][c] != grid[r + j - 1][c] {
                    mistakes += 1;
                    if mistakes > mistakes_allowed {
                        break 'outer;
                    }
                }
            }
            j += 1;
        }
        if mistakes == mistakes_allowed {
            return Some(r);
        }
    }
    None
}

fn reflect_cols(grid: &[&[u8]], mistakes_allowed: usize) -> Option<usize> {
    for c in 1..grid[0].len() {
        let mut j = 1;
        let mut mistakes = 0;
        'outer: while c >= j && c + j - 1 < grid[0].len() {
            // check column c - j and c + j - 1
            for k in 0..grid.len() {
                if grid[k][c - j] != grid[k][c + j - 1] {
                    mistakes += 1;
                    if mistakes > mistakes_allowed {
                        break 'outer;
                    }
                }
            }
            j += 1;
        }
        if mistakes == mistakes_allowed {
            return Some(c);
        }
    }
    return None;
}
