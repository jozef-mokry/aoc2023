use rustc_hash::FxHashMap;
use std::collections::VecDeque;

pub fn solve() {
    println!("---Day 20---");
    let input = std::fs::read_to_string("data/20.txt").unwrap();
    let mut ids = FxHashMap::default();
    let mut graph = vec![];
    let mut start_nodes = vec![];
    let part_2_gate = "rx";
    let mut part_2_gate_id = None;
    for line in input.lines() {
        let (lhs, rhs) = line.split_once(" -> ").unwrap();

        let neighbours: Vec<_> = rhs
            .split(',')
            .map(|s| {
                let candidate_id = ids.len();
                let id = *ids.entry(s.trim()).or_insert(candidate_id);

                if s == part_2_gate {
                    part_2_gate_id = Some(id);
                }
                id
            })
            .collect();

        if lhs == "broadcaster" {
            start_nodes = neighbours;
            continue;
        }

        let gate_type = if &lhs[..1] == "&" {
            Gate::Nand
        } else {
            Gate::FlipFlop
        };
        let candidate_id = ids.len();
        let lhs_id = *ids.entry(&lhs[1..]).or_insert(candidate_id);
        if lhs == part_2_gate {
            part_2_gate_id = Some(lhs_id);
        }
        while graph.len() < ids.len() {
            graph.push((Gate::FlipFlop, vec![]));
        }
        graph[lhs_id] = (gate_type, neighbours);
    }

    let (part1, part2) = press_button(&start_nodes, &graph, 1000, part_2_gate_id.unwrap());
    println!("Part 1: {}", part1.unwrap());

    println!(
        "Part 2: {}",
        part2.unwrap().into_iter().reduce(|acc, x| x * acc).unwrap()
    );
}

#[derive(Clone, Debug)]
enum Gate {
    Nand,
    FlipFlop,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Signal {
    Low,
    High,
}

fn press_button(
    start: &[usize],
    graph: &[(Gate, Vec<usize>)],
    part_1_rounds: usize,
    part_2_gate_id: usize,
) -> (Option<usize>, Option<Vec<usize>>) {
    #[derive(Hash, Clone, Eq, PartialEq, Debug)]
    struct State {
        nands: Vec<usize>,
        flip_flops: Vec<Signal>,
    }
    let mut state = State {
        nands: vec![1; graph.len()],
        flip_flops: vec![Signal::Low; graph.len()],
    };

    // build inverted graph for NAND gates
    let mut inverted_graph: Vec<Vec<usize>> = vec![vec![]; graph.len()];
    for (node, (_, neighbours)) in graph.iter().enumerate() {
        for &n in neighbours {
            inverted_graph[n].push(node);
            match &graph[n].0 {
                Gate::Nand => {
                    state.nands[n] <<= 1;
                }
                Gate::FlipFlop => {}
            }
        }
    }

    let mut high_signals = 0;
    let mut low_signals = 0;
    let mut queue: VecDeque<(usize, usize, Signal)> = Default::default();

    let mut part1 = None;
    let mut part2 = None;
    let gate_id = inverted_graph[part_2_gate_id][0]; // the id of the gate that sends its signal to RX (part_2_gate)
    let mut seen = state.nands[gate_id];
    let mut cycles = vec![];
    for steps in 1.. {
        if steps == part_1_rounds + 1 {
            part1 = Some(low_signals * high_signals);
            if part2.is_some() {
                return (part1, part2);
            }
        }

        low_signals += 1; // push of the button is 1 low signal
        for &n in start {
            queue.push_back((n, 99999, Signal::Low));
        }
        while let Some((to, from, signal)) = queue.pop_front() {
            if to == gate_id {
                let mask = 1 << inverted_graph[to].iter().position(|v| *v == from).unwrap();
                if signal == Signal::High && seen & mask == 0 {
                    seen |= mask;
                    // This happens to work because cycle length is the same as the first number of
                    // steps to see High signal for this input
                    cycles.push(steps);
                    if (seen + 1).is_power_of_two() {
                        part2 = Some(cycles.clone());
                        if part1.is_some() {
                            return (part1, part2);
                        }
                    }
                }
            }
            if signal == Signal::High {
                high_signals += 1;
            } else {
                low_signals += 1;
            }

            match &graph[to] {
                (Gate::Nand, _) => {
                    let mask = 1 << inverted_graph[to].iter().position(|v| *v == from).unwrap();
                    state.nands[to] |= mask;
                    if signal == Signal::Low {
                        state.nands[to] ^= mask;
                    }
                    let signal = if (state.nands[to] + 1).is_power_of_two() {
                        Signal::Low
                    } else {
                        Signal::High
                    };
                    for &neighbour in &graph[to].1 {
                        queue.push_back((neighbour, to, signal));
                    }
                }
                (Gate::FlipFlop, neighbours) => {
                    if signal == Signal::Low {
                        state.flip_flops[to] = match state.flip_flops[to] {
                            Signal::Low => Signal::High,
                            Signal::High => Signal::Low,
                        };
                        for &n in neighbours {
                            queue.push_back((n, to, state.flip_flops[to]));
                        }
                    }
                }
            }
        }
    }
    panic!()
}
