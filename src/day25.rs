use rand::seq::SliceRandom;
use rustc_hash::FxHashMap;
pub fn solve() {
    println!("---Day 25---");
    let mut nodes = FxHashMap::default();
    let mut get_id = |name| {
        let nodes_len = nodes.len();
        *nodes.entry(name).or_insert(nodes_len)
    };
    let mut edges = vec![];
    let input = std::fs::read_to_string("data/25.txt").unwrap();
    for line in input.lines() {
        let (from, dsts) = line.split_once(": ").unwrap();
        let from = get_id(from);
        for dst in dsts.split_ascii_whitespace() {
            edges.push((from, get_id(dst)));
        }
    }

    // Karger's algorithm
    let mut rng = rand::thread_rng();
    let mut iterations = 0;
    loop {
        iterations += 1;
        edges.shuffle(&mut rng);
        let mut components = nodes.len();
        let mut sets = Sets::new(components);
        let mut edges_to_check = &edges[..];
        for i in 0..edges.len() {
            let (from, to) = edges[i];
            let from = sets.find(from);
            let to = sets.find(to);
            if to == from {
                continue;
            }

            sets.union(from, to);
            components -= 1;
            if components == 2 {
                edges_to_check = &edges[i + 1..];
                break;
            }
        }

        let mut cross_edges = 0;
        for &(from, to) in edges_to_check {
            if sets.find(from) != sets.find(to) {
                cross_edges += 1;
            }
        }
        if cross_edges == 3 {
            // we found the right cut
            let mut counts = FxHashMap::default();
            for i in 0..nodes.len() {
                *counts.entry(sets.find(i)).or_insert(0usize) += 1;
            }
            debug_assert!(counts.len() == 2);
            println!(
                "Part 1: {} (found in {} iterations)",
                counts.into_values().reduce(|acc, x| acc * x).unwrap(),
                iterations,
            );
            break;
        }
    }
}

struct Sets {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Sets {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![0; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.find(self.parent[x]);
            self.parent[x]
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if self.size[x] >= self.size[y] {
            self.parent[y] = x;
            self.size[x] += 1;
        } else {
            self.parent[x] = y;
            self.size[y] += 1;
        }
    }
}
