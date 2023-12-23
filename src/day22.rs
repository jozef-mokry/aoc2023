use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{BinaryHeap, VecDeque};
pub fn solve() {
    println!("---Day 22---");
    let input = std::fs::read_to_string("data/22.txt").unwrap();

    let mut queue = BinaryHeap::new();
    let mut num_blocks = 0;
    for line in input.lines() {
        let (lhs, rhs) = line.split_once("~").unwrap();
        let lhs = parse(lhs);
        let rhs = parse(rhs);
        let x = x_range(lhs, rhs);
        let y = y_range(lhs, rhs);
        let z = z_range(lhs, rhs);
        let height = z.1 - z.0 + 1;
        queue.push((std::cmp::Reverse(z.0), num_blocks, x, y, height));
        num_blocks += 1;
    }

    let mut ground: FxHashMap<(isize, isize), (usize, isize)> = Default::default();
    let mut block_supports = vec![vec![]; num_blocks];
    let mut block_sits_on = vec![FxHashSet::default(); num_blocks];

    while let Some((_z, id, (x_min, x_max), (y_min, y_max), height)) = queue.pop() {
        let mut supporters: FxHashSet<usize> = Default::default();
        let mut curr_height = 0;
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                match ground.get(&(x, y)) {
                    None => {}
                    Some((id, height)) => {
                        if *height < curr_height {
                            continue;
                        }
                        if *height > curr_height {
                            curr_height = *height;
                            supporters.clear();
                        }
                        supporters.insert(*id);
                    }
                }
            }
        }

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                ground.insert((x, y), (id, curr_height + height));
            }
        }

        for &s in &supporters {
            block_supports[s].push(id);
        }
        block_sits_on[id] = supporters;
    }

    let mut ans = 0;
    'outer: for above_blocks in &block_supports {
        for &block in above_blocks {
            debug_assert!(block_sits_on[block].len() > 0);
            if block_sits_on[block].len() == 1 {
                continue 'outer;
            }
        }
        ans += 1;
    }
    println!("Part 1: {ans}");

    let mut ans = 0;
    for i in 0..num_blocks {
        ans += disintegrate(i, &block_sits_on, &block_supports);
    }
    println!("Part 2: {}", ans,);
}

fn disintegrate(
    id: usize,
    block_sits_on: &[FxHashSet<usize>],
    block_supports: &[Vec<usize>],
) -> usize {
    let mut removed: FxHashSet<usize> = Default::default();
    let mut queue: VecDeque<usize> = VecDeque::new();
    removed.insert(id);
    queue.push_back(id);
    while let Some(id) = queue.pop_front() {
        for &above_block in &block_supports[id] {
            let below = &block_sits_on[above_block];
            if below.is_subset(&removed) {
                removed.insert(above_block);
                queue.push_back(above_block);
            }
        }
    }
    removed.len() - 1
}

fn parse(input: &str) -> (isize, isize, isize) {
    let mut vals = input.split(',').map(|v| v.parse::<isize>().unwrap());
    (
        vals.next().unwrap(),
        vals.next().unwrap(),
        vals.next().unwrap(),
    )
}

fn x_range(a: (isize, isize, isize), b: (isize, isize, isize)) -> (isize, isize) {
    (a.0.min(b.0), a.0.max(b.0))
}
fn y_range(a: (isize, isize, isize), b: (isize, isize, isize)) -> (isize, isize) {
    (a.1.min(b.1), a.1.max(b.1))
}
fn z_range(a: (isize, isize, isize), b: (isize, isize, isize)) -> (isize, isize) {
    (a.2.min(b.2), a.2.max(b.2))
}
