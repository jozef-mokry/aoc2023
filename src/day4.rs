use std::collections::BTreeSet;
pub fn solve() {
    println!("----Day 4----");
    let input = std::fs::read_to_string("data/4.txt").unwrap();

    let mut ans = 0;
    let mut ans2 = 0;
    let mut card_counts = [1; 198];
    for (i, line) in input.lines().enumerate() {
        let curr_count = card_counts[i];
        ans2 += curr_count;
        let mut line = line.split("|");
        let left: BTreeSet<_> = line
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(2)
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let count = line
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .filter(|v| left.contains(v))
            .count();
        if count > 0 {
            ans += 1 << (count - 1);
        }
        for v in &mut card_counts[i + 1..=i + count] {
            *v += curr_count;
        }
    }
    println!("Part 1: {ans}");
    println!("Part 2: {ans2}");
}
