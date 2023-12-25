use rustc_hash::FxHashMap;
use std::collections::VecDeque;
pub fn solve() {
    println!("---Day 23---");
    let input = std::fs::read_to_string("data/23.txt").unwrap();

    let grid: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    let start = (0, grid[0].iter().position(|c| *c == b'.').unwrap() as isize);
    let end = (
        grid.len() as isize - 1,
        grid[grid.len() - 1]
            .iter()
            .position(|c| *c == b'.')
            .unwrap() as isize,
    );

    let graph = build_graph(start, &grid, end, true);
    println!("Part 1: {}", find_longest(0, 1, &graph, 0, 0).unwrap());
    let graph = build_graph(start, &grid, end, false);
    println!("Part 2: {}", find_longest(0, 1, &graph, 0, 0).unwrap());
}

fn find_longest(
    start: usize,
    end: usize,
    graph: &Graph,
    visited: usize,
    steps: usize,
) -> Option<usize> {
    if visited & (1 << start) != 0 {
        return None;
    }
    if start == end {
        return Some(steps);
    }
    let visited = visited | (1 << start);

    graph[start]
        .iter()
        .map(|&(next, next_steps)| find_longest(next, end, graph, visited, steps + next_steps))
        .max()
        .unwrap()
}

type Graph = Vec<Vec<(usize, usize)>>;
fn build_graph(
    start: (isize, isize),
    grid: &[&[u8]],
    end: (isize, isize),
    follow_arrows: bool,
) -> Graph {
    let mut nodes = FxHashMap::default();
    nodes.insert(start, 0);
    nodes.insert(end, 1);
    let mut graph: Graph = vec![];
    let mut get_id = |pos| {
        let nodes_len = nodes.len();
        *nodes.entry(pos).or_insert(nodes_len)
    };
    let mut add_edge = |from, to, steps| {
        if graph.len() <= from {
            graph.resize(from + 1, Default::default());
        }
        graph[from].push((to, steps));
    };

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, start, start));
    let mut visited = vec![vec![0; grid[0].len()]; grid.len()];

    while let Some((id, steps, pos, prev_pos)) = queue.pop_front() {
        if visited[pos.0 as usize][pos.1 as usize] & (1 << id) != 0 {
            continue;
        }
        if pos == end {
            let end_id = get_id(end);
            add_edge(id, end_id, steps);
        }
        visited[pos.0 as usize][pos.1 as usize] |= 1 << id;
        match grid[pos.0 as usize][pos.1 as usize] {
            b'^' | b'<' => unimplemented!(), // these do not appear in test data :-)
            b'>' if follow_arrows => queue.push_back((id, steps + 1, (pos.0, pos.1 + 1), pos)),
            b'v' if follow_arrows => queue.push_back((id, steps + 1, (pos.0 + 1, pos.1), pos)),
            b'.' | b'v' | b'>' => {
                let mut opts = [(0, 1), (1, 0), (0, -1), (-1, 0)]
                    .into_iter()
                    .map(|(x, y)| (pos.0 + x, pos.1 + y))
                    .filter(|&pos| {
                        !(pos.0 < 0
                            || pos.1 < 0
                            || pos.0 >= grid.len() as isize
                            || pos.1 >= grid[0].len() as isize
                            || pos == prev_pos
                            || grid[pos.0 as usize][pos.1 as usize] == b'#')
                    });

                let opts_count = opts.clone().count();
                if opts_count == 1 {
                    queue.push_back((id, steps + 1, opts.next().unwrap(), pos));
                } else if opts_count > 1 {
                    // we encounter an intersection
                    let pos_id = get_id(pos);
                    add_edge(id, pos_id, steps);
                    for new_pos in opts {
                        queue.push_back((pos_id, 1, new_pos, pos));
                    }
                }
            }
            c => panic!("{} {:?}", c as char, start),
        };
    }
    graph
}
