use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;
pub fn solve() {
    println!("---Day 15---");
    let input = std::fs::read_to_string("data/15.txt").unwrap();
    let input = input.trim();
    let mut ans = 0;
    let mut boxes: Boxes = Boxes::new();
    for instr in input.split(',') {
        ans += hash(instr);

        if let Some((label, focal)) = instr.split_once('=') {
            boxes.add(label, focal.parse::<usize>().unwrap());
        } else {
            boxes.remove(&instr[..instr.len() - 1]);
        }
    }
    println!("Part 1: {ans}");
    println!("Part 2: {}", boxes.focusing_power());
}

fn hash(input: &str) -> usize {
    let mut hash = 0;
    for c in input.as_bytes() {
        hash = ((hash + *c as usize) * 17) % 256;
    }
    hash
}

#[derive(Debug)]
struct Boxes<'a> {
    lens: Vec<Option<(usize, usize)>>,
    label_to_pos: FxHashMap<&'a str, usize>,
}

impl<'a> Boxes<'a> {
    fn new() -> Self {
        Self {
            lens: vec![],
            label_to_pos: FxHashMap::default(),
        }
    }
    fn remove(&mut self, label: &'a str) {
        if let Entry::Occupied(e) = self.label_to_pos.entry(label) {
            let pos = e.remove();
            self.lens[pos] = None;
        }
    }

    fn add(&mut self, label: &'a str, lens: usize) {
        match self.label_to_pos.entry(label) {
            Entry::Vacant(e) => {
                let box_id = hash(label);
                e.insert(self.lens.len());
                self.lens.push(Some((box_id, lens)));
            }
            Entry::Occupied(e) => {
                self.lens[*e.get()].as_mut().unwrap().1 = lens;
            }
        }
    }

    fn focusing_power(&self) -> usize {
        let mut box_size = [0; 256];
        let mut power = 0;
        for &(box_id, lens) in self.lens.iter().filter_map(|v| v.as_ref()) {
            box_size[box_id] += 1;
            power += (box_id + 1) * box_size[box_id] * lens;
        }
        power
    }
}
