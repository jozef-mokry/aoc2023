pub fn solve() {
    let input = std::fs::read_to_string("data/11.txt").unwrap();
    println!("---Day 11---");
    println!("Part 1: {}", calc_dist(&input, 1));
    println!("Part 2: {}", calc_dist(&input, 999_999));
}

fn calc_dist(input: &str, extra_for_empty: isize) -> isize {
    let mut row = 0;
    let mut col_has_galaxy = vec![false; input.lines().next().unwrap().len()];
    let mut galaxies: Vec<(isize, isize)> = vec![];
    for line in input.lines() {
        let mut has_galaxy = false;
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                has_galaxy = true;
                col_has_galaxy[i] = true;
                galaxies.push((row, i as isize)); // fix cols later
            }
        }
        row += 1;
        if !has_galaxy {
            row += extra_for_empty;
        }
    }

    // Fix columns
    let mut col_idx = 0;
    let mut curr_row = None;
    let mut empty_cols = 0;
    for (row, col) in &mut galaxies {
        if Some(*row) != curr_row {
            curr_row = Some(*row);
            col_idx = 0;
            empty_cols = 0;
        }
        while col_idx < *col {
            if !col_has_galaxy[col_idx as usize] {
                empty_cols += 1;
            }
            col_idx += 1;
        }
        *col += extra_for_empty * empty_cols;
    }

    let mut dist = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (x, y) = galaxies[i];
            let (xx, yy) = galaxies[j];
            dist += (x - xx).abs() + (y - yy).abs();
        }
    }
    dist
}
