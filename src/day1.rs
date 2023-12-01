use std::io::BufRead;

pub fn solve() {
    // part1();
    part2();
}

fn part1() {
    let mut ans: usize = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut line = line.as_bytes().into_iter();
        let a = line.find(|&c| c.is_ascii_digit()).unwrap();
        let b = line.rfind(|&c| c.is_ascii_digit()).unwrap_or(&a);
        ans += (10 * (a - b'0') + b - b'0') as usize;
    }
    println!("Part 1: {ans}");
}

fn part2() {
    let mut ans: usize = 0;
    let numbers = [
        "0", "zero", "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six",
        "7", "seven", "8", "eight", "9", "nine",
    ];
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut a = None;
        let mut b = None;
        for i in 0..numbers.len() {
            if let Some(pos) = line.find(numbers[i]) {
                a = match a {
                    Some((_, a_pos)) if pos < a_pos => Some((i / 2, pos)),
                    None => Some((i / 2, pos)),
                    _ => a,
                };
            } else {
                continue;
            }

            if let Some(pos) = line.rfind(numbers[i]) {
                b = match b {
                    Some((_, b_pos)) if pos > b_pos => Some((i / 2, pos)),
                    None => Some((i / 2, pos)),
                    _ => b,
                };
            }
        }
        ans += match (a, b) {
            (Some((a, _)), Some((b, _))) => a * 10 + b,
            _ => panic!(),
        };
    }
    println!("Part 2: {ans}");
}
