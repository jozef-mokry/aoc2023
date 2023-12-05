use std::collections::BTreeSet;
use std::ops::Bound::{Included, Unbounded};
pub fn solve() {
    println!("---Day 5:---");
    let input = std::fs::read_to_string("data/5.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut lines = input.lines();
    let mut seeds: Vec<_> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    lines.next(); // skip empty line

    for _ in 0..7 {
        let mapping: BTreeSet<_> = lines
            .by_ref()
            .skip(1)
            .take_while(|s| !s.is_empty())
            .map(|line| {
                let mut vals = line.split_ascii_whitespace();
                let dst = vals.next().unwrap().parse::<usize>().unwrap();
                let src = vals.next().unwrap().parse::<usize>().unwrap();
                let len = vals.next().unwrap().parse::<usize>().unwrap();
                (src, dst, len)
            })
            .collect();
        for v in &mut seeds {
            *v = match mapping
                .range((Unbounded, Included((*v, usize::MAX, usize::MAX))))
                .next_back()
            {
                Some((src, dst, len)) if *v <= src + len => dst + (*v - src),
                _ => *v,
            };
        }
    }
    println!("Part 1: {:?}", seeds.into_iter().min().unwrap());
}

fn part2(input: &str) {
    let mut lines = input.lines();
    let mut seeds = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|v| v.parse::<usize>().unwrap());
    let mut intervals = vec![];
    while let (Some(start), Some(len)) = (seeds.next(), seeds.next()) {
        intervals.push((start, len));
    }

    lines.next(); // skip empty line

    for _ in 0..7 {
        let mapping: BTreeSet<_> = lines
            .by_ref()
            .skip(1)
            .take_while(|s| !s.is_empty())
            .map(|line| {
                let mut vals = line.split_ascii_whitespace();
                let dst = vals.next().unwrap().parse::<usize>().unwrap();
                let src = vals.next().unwrap().parse::<usize>().unwrap();
                let len = vals.next().unwrap().parse::<usize>().unwrap();
                (src, dst, len)
            })
            .collect();
        let mut new_intervals = vec![];
        for v in intervals {
            update_range(v, &mapping, &mut new_intervals);
        }
        intervals = new_intervals;
    }
    println!("Part 2: {:?}", intervals.into_iter().min().unwrap().0);
}

fn update_range(
    (mut v_start, mut v_len): (usize, usize),
    mapping: &BTreeSet<(usize, usize, usize)>,
    new_vals: &mut Vec<(usize, usize)>,
) {
    for &(src, dst, len) in mapping {
        // Case 1: mapping interval is before v_interval
        if src + len <= v_start {
            continue;
        }
        // Case 2: mapping interval is after v_interval
        if v_start + v_len <= src {
            new_vals.push((v_start, v_len));
            return;
        }
        // Cases with overlap
        // The v_interval might be split into at most 3 parts: unchanged, mapped, unchanged
        // Case 3: There might be a chunk at the beginning of v_interval that is not mapped
        if v_start < src {
            new_vals.push((v_start, src - v_start));
            v_start = src;
            v_len -= src - v_start;
        }

        // Case 4: Now comes the chunk that is mapped
        // At this point v_start >= src
        let new_val = (
            v_start - src + dst,
            (v_start + v_len).min(src + len) - v_start,
        );
        v_len -= new_val.1;
        v_start += new_val.1;
        new_vals.push(new_val);

        if v_len == 0 {
            return;
        }
    }

    if v_len != 0 {
        new_vals.push((v_start, v_len));
    }
}
