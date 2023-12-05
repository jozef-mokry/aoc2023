pub fn solve() {
    let input = std::fs::read_to_string("data/3.txt").unwrap();
    println!("----Day 3:----");
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let grid: Vec<_> = input.lines().map(|s| s.as_bytes()).collect();

    let mut ans = 0;
    let mut num = 0;
    let mut touching = false;
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                b'0'..=b'9' => {
                    num = num * 10 + (grid[i][j] - b'0') as usize;
                    touching |= dirs.iter().any(|(dx, dy)| {
                        match grid
                            .get(((i as isize) + dx) as usize)
                            .and_then(|v| v.get(((j as isize) + dy) as usize))
                        {
                            Some(b'.' | b'0'..=b'9') | None => false,
                            _ => true,
                        }
                    });
                }
                _ => {
                    if touching {
                        ans += num;
                    }
                    num = 0;
                    touching = false;
                }
            }
        }
        if touching {
            ans += num;
        }
        num = 0;
        touching = false;
    }

    println!("Part 1: {ans}");
}

fn part2(input: &str) {
    let grid: Vec<_> = input.lines().map(|v| v.as_bytes()).collect();
    let mut ans = 0;

    struct Check {
        start_dx: isize,
        start_dy: isize,
        unless: Option<(isize, isize)>,
    }
    let checks = [
        Check {
            start_dx: -1,
            start_dy: 0,
            unless: None,
        },
        Check {
            start_dx: -1,
            start_dy: -1,
            unless: Some((-1, 0)),
        },
        Check {
            start_dx: -1,
            start_dy: 1,
            unless: Some((-1, 0)),
        },
        Check {
            start_dx: 1,
            start_dy: 0,
            unless: None,
        },
        Check {
            start_dx: 1,
            start_dy: -1,
            unless: Some((1, 0)),
        },
        Check {
            start_dx: 1,
            start_dy: 1,
            unless: Some((1, 0)),
        },
        Check {
            start_dx: 0,
            start_dy: -1,
            unless: None,
        },
        Check {
            start_dx: 0,
            start_dy: 1,
            unless: None,
        },
    ];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != b'*' {
                continue;
            }
            let mut nums = [0, 0];
            let mut nums_idx = 0;
            let mut too_many_nums = false;

            for Check {
                start_dx,
                start_dy,
                unless,
            } in &checks
            {
                let si = i as isize + start_dx;
                let mut sj = j as isize + start_dy;
                if si < 0
                    || sj < 0
                    || si >= grid.len() as isize
                    || sj >= grid[i as usize].len() as isize
                    || !grid[si as usize][sj as usize].is_ascii_digit()
                {
                    continue;
                }
                if let Some((x, y)) = unless {
                    if grid[((i as isize) + x) as usize][((j as isize) + y) as usize]
                        .is_ascii_digit()
                    {
                        continue;
                    }
                }
                if nums_idx == 2 {
                    // sigh, this edge case never happens in input data, but I checked anyway
                    too_many_nums = true;
                    break;
                }
                if *start_dy != 1 {
                    while sj - 1 >= 0
                        && sj - 1 < grid[si as usize].len() as isize
                        && grid[si as usize][(sj - 1) as usize].is_ascii_digit()
                    {
                        sj -= 1;
                    }
                }
                nums[nums_idx] = get_int(&grid[si as usize][sj as usize..]);
                nums_idx += 1;
            }

            if nums_idx == 2 && !too_many_nums {
                ans += nums[0] * nums[1];
            }
        }
    }
    println!("Part 2: {ans}");
}

fn get_int(text: &[u8]) -> usize {
    let mut ans = 0;
    for c in text {
        if !c.is_ascii_digit() {
            break;
        }
        ans = ans * 10 + (c - b'0') as usize;
    }
    ans
}
