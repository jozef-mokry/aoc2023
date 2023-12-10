pub fn solve() {
    println!("---Day 10---");
    let input = std::fs::read_to_string("data/10.txt").unwrap();
    let mut grid: Vec<Vec<u8>> = input.lines().map(|s| s.bytes().collect()).collect();
    let s_pos = grid
        .iter()
        .flat_map(|v| v.iter())
        .position(|&c| c == b'S')
        .unwrap();

    let s_row = s_pos / grid[0].len();
    let s_col = s_pos % grid[0].len();

    // looking at input, only up and down make sense as start positions
    let mut a_pos = (s_row - 1, s_col);
    let mut a_dir = (-1, 0);
    let mut b_pos = (s_row + 1, s_col);
    let mut b_dir = (1, 0);

    let mut steps = 1;

    // mark all visited cells based on the type of movement
    fn get_marker(cell: u8) -> u8 {
        match cell {
            b'|' => b'V',        // Vertical
            b'-' => b'H',        // Horizontal
            b'L' | b'J' => b'U', // turn Upwards
            b'7' | b'F' => b'D', // turn Downwards
            _ => panic!("{cell}"),
        }
    }
    grid[s_row][s_col] = b'V';

    loop {
        let a_cell = &mut grid[a_pos.0][a_pos.1];
        (a_pos, a_dir) = update(*a_cell, a_pos, a_dir);
        *a_cell = get_marker(*a_cell);
        if a_pos == b_pos {
            break;
        }
        let b_cell = &mut grid[b_pos.0][b_pos.1];
        (b_pos, b_dir) = update(*b_cell, b_pos, b_dir);
        *b_cell = get_marker(*b_cell);
        steps += 1;
        if a_pos == b_pos {
            break;
        }
    }
    grid[a_pos.0][a_pos.1] = get_marker(grid[a_pos.0][a_pos.1]);

    println!("Part 1: {steps}");

    let mut inside_count = 0;
    // you enter a loop if you cross a Vertical cell or if you find a horizontal segment that has
    // one Downwards and one Upwards end
    for row in &mut grid {
        let mut inside = false;
        let mut prev_turn = None;
        for val in row {
            match *val {
                b'V' => {
                    inside = !inside;
                }
                b'H' => {}
                b'D' | b'U' => {
                    if let Some(turn) = prev_turn {
                        if *val != turn {
                            inside = !inside;
                        }
                        prev_turn = None;
                    } else {
                        prev_turn = Some(*val);
                    }
                }
                _ => {
                    if inside {
                        inside_count += 1;
                    }
                }
            };
        }
    }
    println!("Part 2: {inside_count}");
}

fn update(
    cell_value: u8,
    (row, col): (usize, usize),
    direction: (isize, isize),
) -> ((usize, usize), (isize, isize)) {
    let new_dir = match (cell_value, direction) {
        (b'|' | b'-', _) => direction,
        (b'L', (1, 0)) => (0, 1),
        (b'L', (0, -1)) => (-1, 0),
        (b'J', (1, 0)) => (0, -1),
        (b'J', (0, 1)) => (-1, 0),
        (b'7', (0, 1)) => (1, 0),
        (b'7', (-1, 0)) => (0, -1),
        (b'F', (0, -1)) => (1, 0),
        (b'F', (-1, 0)) => (0, 1),
        _ => panic!("{:?} {direction:?}", cell_value as char),
    };
    (
        (
            (row as isize + new_dir.0) as usize,
            (col as isize + new_dir.1) as usize,
        ),
        new_dir,
    )
}
