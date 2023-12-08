use std::collections::{HashMap, HashSet};
pub fn solve() {
    let input = std::fs::read_to_string("data/8.txt").unwrap();
    println!("---Day 8---");
    let mut lines = input.lines();
    let mut instrs = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .enumerate()
        .cycle();
    let instrs_part2 = instrs.clone();
    lines.next(); // skip empty line
    let num_of_nodes = lines.clone().count();
    let mut graph = vec![[0, 0]; num_of_nodes];
    let mut start_nodes = vec![];
    let mut ends_with_z = vec![false; num_of_nodes];
    let mut node_ids = HashMap::new();
    for line in lines {
        let mut edge = line
            .split(['=', ' ', '(', ')', ','])
            .filter(|s| !s.is_empty());
        if let (Some(from), Some(left), Some(right)) = (edge.next(), edge.next(), edge.next()) {
            let next_id = node_ids.len();
            let &mut from_id = node_ids.entry(from).or_insert(next_id);
            if from.ends_with('A') {
                start_nodes.push(from_id);
            }
            if from.ends_with('Z') {
                ends_with_z[from_id] = true;
            }

            let next_id = node_ids.len();
            let &mut left_id = node_ids.entry(left).or_insert(next_id);
            let next_id = node_ids.len();
            let &mut right_id = node_ids.entry(right).or_insert(next_id);
            graph[from_id] = [left_id, right_id];
        } else {
            panic!();
        }
    }

    let mut curr = *node_ids.get("AAA").unwrap();
    let goal = *node_ids.get("ZZZ").unwrap();
    let mut steps = 0;
    while curr != goal {
        let (_, instr) = instrs.next().unwrap();
        curr = graph[curr][instr];
        steps += 1;
    }
    println!("Part 1: {steps}");

    // Part 2
    let mut global_steps_to_z = 1;

    for i in 0..start_nodes.len() {
        // it turns out that cycle length is the same as time to reach xxxZ node
        // Also, each ghost visits only one Z node on its path. If these assumptions were not true,
        // than more work would be needed.
        let (_, cycle) = get_path_info(start_nodes[i], instrs_part2.clone(), &graph, &ends_with_z);
        global_steps_to_z = global_steps_to_z * cycle / gcd(global_steps_to_z, cycle);
    }
    println!("Part 2: {global_steps_to_z}");
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn get_path_info<'a>(
    mut curr: usize,
    mut instrs: impl Iterator<Item = (usize, usize)>,
    graph: &[[usize; 2]],
    ends_with_z: &[bool],
) -> (usize, usize) {
    let mut seen = HashMap::new();
    let mut time_to_z = 0;
    let mut steps = 0;

    loop {
        let (instr_idx, instr) = instrs.next().unwrap();
        curr = graph[curr][instr];
        steps += 1;
        if let Some(prev_steps) = seen.get(&(instr_idx, curr)) {
            let cycle_len = steps - prev_steps;
            return (time_to_z, cycle_len);
        } else {
            seen.insert((instr_idx, curr), steps);
        }

        if ends_with_z[curr] {
            time_to_z = steps;
        }
    }
}
