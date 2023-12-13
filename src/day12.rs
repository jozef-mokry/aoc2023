use rustc_hash::FxHashMap;

pub fn solve() {
    println!("---Day 12---");
    let input = std::fs::read_to_string("data/12.txt").unwrap();

    let mut ans = 0;
    let mut ans2 = 0;
    let mut mem = Mem::default();
    for line in input.lines() {
        let (symbols, counts) = line.split_once(' ').unwrap();
        let mut counts: Vec<_> = counts
            .split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let ranges = parse_symbols(symbols);
        ans += process(&mut counts[..], &ranges[..], &mut mem);
        mem.clear();

        // Part 2
        let new_counts_len = counts.len() * 5;
        let mut counts: Vec<_> = counts.into_iter().cycle().take(new_counts_len).collect();
        let mut symbols = (symbols.to_owned() + "?").repeat(5);
        symbols.pop();
        let new_ranges = parse_symbols(&symbols);
        ans2 += process(&mut counts[..], &new_ranges[..], &mut mem);
        mem.clear();
    }
    println!("Part 1: {ans}");
    println!("Part 2: {ans2}");
}

#[derive(Default)]
struct Mem {
    known: FxHashMap<(usize, usize, Option<usize>), usize>,
    unknown: FxHashMap<(usize, usize, Option<usize>, usize, bool), usize>,
}

impl Mem {
    fn clear(&mut self) {
        self.known.clear();
        self.unknown.clear();
    }
}

fn process(counts: &mut [usize], ranges: &[Range], mem: &mut Mem) -> usize {
    if let Some(Range {
        range_type: RangeType::Known,
        ..
    }) = ranges.first()
    {
        process_known(counts, ranges, mem)
    } else {
        process_unknown(counts, ranges, 0, true, mem)
    }
}

fn process_known(counts: &mut [usize], ranges: &[Range], mem: &mut Mem) -> usize {
    debug_assert!(
        ranges.is_empty() || ranges[0].range_type == RangeType::Known,
        "ranges={:?}",
        ranges
    );
    let key = (counts.len(), ranges.len(), counts.get(0).cloned());
    if let Some(prev_ans) = mem.known.get(&key) {
        return *prev_ans;
    }
    let ans = match (counts, ranges) {
        ([], []) => 1,
        (_, []) | ([], _) => 0,
        ([c, ..], [r, ..]) if *c < r.len => 0,
        ([c, counts @ ..], [r, ..]) if *c == r.len => {
            if r.followed {
                process_unknown(counts, &ranges[1..], 1, true, mem)
            } else {
                process(counts, &ranges[1..], mem)
            }
        }
        (counts, [r, ..]) if counts[0] > r.len && r.followed => {
            counts[0] -= r.len;
            let ans = process_unknown(&mut counts[..], &ranges[1..], 0, false, mem);
            counts[0] += r.len;
            ans
        }
        ([c, ..], [r, ..]) if *c > r.len && !r.followed => 0,
        _ => panic!(),
    };
    mem.known.insert(key, ans);
    ans
}

fn process_unknown(
    counts: &mut [usize],
    ranges: &[Range],
    used: usize,
    can_skip: bool,
    mem: &mut Mem,
) -> usize {
    debug_assert!(ranges.is_empty() || used <= ranges[0].len);
    debug_assert!(
        ranges.is_empty() || ranges[0].range_type == RangeType::Unknown,
        "ranges={:?}",
        ranges
    );

    let key = (
        counts.len(),
        ranges.len(),
        counts.get(0).cloned(),
        used,
        can_skip,
    );
    if let Some(prev_ans) = mem.unknown.get(&key) {
        return *prev_ans;
    }
    let ans = match (counts, ranges) {
        ([], []) => 1,
        (_, []) => 0,
        ([], [_, ranges @ ..]) => process(&mut [], ranges, mem),
        (_, [r, ..]) if r.len < used => 0,
        (counts, [r, ranges @ ..]) if used == r.len => process(counts, ranges, mem),
        (counts, ranges) => {
            if can_skip {
                process_unknown(counts, ranges, used, false, mem)
                    + process_unknown(counts, ranges, used + 1, true, mem)
            } else if counts[0] + used == ranges[0].len && !ranges[0].followed {
                process(&mut counts[1..], &ranges[1..], mem)
            } else if counts[0] + used < ranges[0].len {
                let used = used + counts[0] + 1;
                process_unknown(&mut counts[1..], ranges, used, true, mem)
            } else if counts[0] + used > ranges[0].len && ranges[0].followed {
                counts[0] = counts[0] + used - ranges[0].len;
                let ans = process_known(counts, &ranges[1..], mem);
                counts[0] = counts[0] + ranges[0].len - used;
                ans
            } else {
                0
            }
        }
    };

    mem.unknown.insert(key, ans);
    ans
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum RangeType {
    Known,
    Unknown,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Range {
    range_type: RangeType,
    len: usize,
    followed: bool,
}

impl Range {
    fn new(range_type: RangeType) -> Self {
        Range {
            len: 1,
            followed: true,
            range_type,
        }
    }
}

fn parse_symbols(input: &str) -> Vec<Range> {
    let mut ranges = vec![];

    let mut current_range: Option<Range> = None;

    for c in input.chars() {
        current_range = match (c, current_range) {
            ('.', None) => None,
            ('.', Some(mut r)) => {
                r.followed = false;
                ranges.push(r);
                None
            }
            ('#', None) => Some(Range::new(RangeType::Known)),
            ('#', Some(mut r)) if r.range_type == RangeType::Known => {
                r.len += 1;
                Some(r)
            }
            ('#', Some(mut range)) => {
                range.followed = true;
                ranges.push(range);
                Some(Range::new(RangeType::Known))
            }
            ('?', None) => Some(Range::new(RangeType::Unknown)),
            ('?', Some(mut r)) if r.range_type == RangeType::Unknown => {
                r.len += 1;
                Some(r)
            }
            ('?', Some(mut r)) => {
                r.followed = true;
                ranges.push(r);
                Some(Range::new(RangeType::Unknown))
            }
            _ => panic!(),
        };
    }
    if let Some(mut r) = current_range {
        r.followed = false;
        ranges.push(r);
    }
    ranges
}
