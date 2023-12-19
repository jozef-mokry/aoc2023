use rustc_hash::FxHashMap;
pub fn solve() {
    println!("---Day 19---");
    let input = std::fs::read_to_string("data/19.txt").unwrap();
    let (rules, parts) = parse_input(&input);
    let mut ans = 0;
    for part in parts {
        if matches(&part, &rules) {
            ans += part.x + part.m + part.a + part.s;
        }
    }
    println!("Part 1: {ans}");
    println!(
        "Part 2: {}",
        count(
            "in",
            PartWithRanges {
                x_min: 1,
                x_max: 4001,
                m_min: 1,
                m_max: 4001,
                a_min: 1,
                a_max: 4001,
                s_min: 1,
                s_max: 4001
            },
            &rules
        )
    );
}

type Rules<'a> = FxHashMap<&'a str, Vec<Condition<'a>>>;

fn matches(part: &Part, rules: &Rules) -> bool {
    let mut curr = "in";
    while curr != "R" && curr != "A" {
        for cond in &rules[curr] {
            curr = match cond {
                Condition::XMore(x, to) if part.x > *x => to,
                Condition::MMore(m, to) if part.m > *m => to,
                Condition::AMore(a, to) if part.a > *a => to,
                Condition::SMore(s, to) if part.s > *s => to,
                Condition::XLess(x, to) if part.x < *x => to,
                Condition::MLess(m, to) if part.m < *m => to,
                Condition::ALess(a, to) if part.a < *a => to,
                Condition::SLess(s, to) if part.s < *s => to,
                Condition::Direct(to) => to,
                _ => continue,
            };
            break;
        }
    }
    curr == "A"
}

fn count(state: &str, part: PartWithRanges, rules: &Rules) -> usize {
    if state == "R" {
        return 0;
    } else if state == "A" {
        let PartWithRanges {
            x_min,
            x_max,
            m_min,
            m_max,
            a_min,
            a_max,
            s_min,
            s_max,
        } = part;
        return (x_max - x_min) * (m_max - m_min) * (a_max - a_min) * (s_max - s_min);
    }

    let mut ans = 0;
    let mut part = part;
    for cond in &rules[state] {
        let (matching_part, non_matching_part) = part.intersect(cond);
        if let Some(p) = matching_part {
            use Condition::*;
            ans += match cond {
                Direct(to)
                | XMore(_, to)
                | MMore(_, to)
                | AMore(_, to)
                | SMore(_, to)
                | XLess(_, to)
                | MLess(_, to)
                | ALess(_, to)
                | SLess(_, to) => count(to, p, rules),
            };
        }
        if let Some(p) = non_matching_part {
            part = p;
        } else {
            break;
        }
    }
    return ans;
}

#[derive(Debug)]
enum Condition<'a> {
    XMore(usize, &'a str),
    MMore(usize, &'a str),
    AMore(usize, &'a str),
    SMore(usize, &'a str),
    XLess(usize, &'a str),
    MLess(usize, &'a str),
    ALess(usize, &'a str),
    SLess(usize, &'a str),
    Direct(&'a str),
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug)]
struct PartWithRanges {
    x_min: usize,
    x_max: usize,
    m_min: usize,
    m_max: usize,
    a_min: usize,
    a_max: usize,
    s_min: usize,
    s_max: usize,
}

impl PartWithRanges {
    fn intersect(&self, c: &Condition) -> (Option<PartWithRanges>, Option<PartWithRanges>) {
        use Condition::*;
        let &PartWithRanges {
            x_min,
            x_max,
            m_min,
            m_max,
            a_min,
            a_max,
            s_min,
            s_max,
        } = self;
        match c {
            Direct(_) => (Some(PartWithRanges { ..*self }), None),
            XLess(x, _) => {
                let (smaller, larger) = split(x_min, x_max, *x, false);
                (
                    smaller.map(|(x_min, x_max)| self.with_x(x_min, x_max)),
                    larger.map(|(x_min, x_max)| self.with_x(x_min, x_max)),
                )
            }
            XMore(x, _) => {
                let (smaller, larger) = split(x_min, x_max, *x, true);
                (
                    larger.map(|(x_min, x_max)| self.with_x(x_min, x_max)),
                    smaller.map(|(x_min, x_max)| self.with_x(x_min, x_max)),
                )
            }
            MLess(m, _) => {
                let (smaller, larger) = split(m_min, m_max, *m, false);
                (
                    smaller.map(|(m_min, m_max)| self.with_m(m_min, m_max)),
                    larger.map(|(m_min, m_max)| self.with_m(m_min, m_max)),
                )
            }
            MMore(m, _) => {
                let (smaller, larger) = split(m_min, m_max, *m, true);
                (
                    larger.map(|(m_min, m_max)| self.with_m(m_min, m_max)),
                    smaller.map(|(m_min, m_max)| self.with_m(m_min, m_max)),
                )
            }
            ALess(a, _) => {
                let (smaller, larger) = split(a_min, a_max, *a, false);
                (
                    smaller.map(|(a_min, a_max)| self.with_a(a_min, a_max)),
                    larger.map(|(a_min, a_max)| self.with_a(a_min, a_max)),
                )
            }
            AMore(a, _) => {
                let (smaller, larger) = split(a_min, a_max, *a, true);
                (
                    larger.map(|(a_min, a_max)| self.with_a(a_min, a_max)),
                    smaller.map(|(a_min, a_max)| self.with_a(a_min, a_max)),
                )
            }
            SLess(s, _) => {
                let (smaller, larger) = split(s_min, s_max, *s, false);
                (
                    smaller.map(|(s_min, s_max)| self.with_s(s_min, s_max)),
                    larger.map(|(s_min, s_max)| self.with_s(s_min, s_max)),
                )
            }
            SMore(s, _) => {
                let (smaller, larger) = split(s_min, s_max, *s, true);
                (
                    larger.map(|(s_min, s_max)| self.with_s(s_min, s_max)),
                    smaller.map(|(s_min, s_max)| self.with_s(s_min, s_max)),
                )
            }
        }
    }

    fn with_x(&self, x_min: usize, x_max: usize) -> Self {
        Self {
            x_min,
            x_max,
            ..*self
        }
    }
    fn with_m(&self, m_min: usize, m_max: usize) -> Self {
        Self {
            m_min,
            m_max,
            ..*self
        }
    }
    fn with_a(&self, a_min: usize, a_max: usize) -> Self {
        Self {
            a_min,
            a_max,
            ..*self
        }
    }
    fn with_s(&self, s_min: usize, s_max: usize) -> Self {
        Self {
            s_min,
            s_max,
            ..*self
        }
    }
}

// Split the range [min, max) by value x.
fn split(
    min: usize,
    max: usize,
    x: usize,
    x_stays_in_smaller: bool,
) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let smaller = {
        let cut_at = if x_stays_in_smaller { x + 1 } else { x };
        if cut_at <= min {
            None
        } else {
            Some((min, cut_at.min(max)))
        }
    };

    // max is open interval (ie not included)
    let larger = {
        let cut_at = if !x_stays_in_smaller { x - 1 } else { x };
        if cut_at + 1 >= max {
            None
        } else {
            Some(((cut_at + 1).max(min), max))
        }
    };
    (smaller, larger)
}

fn parse_input(input: &str) -> (Rules, Vec<Part>) {
    let mut rules = FxHashMap::<&str, Vec<Condition>>::default();
    let mut parts = vec![];
    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut vals = line.split(['{', ',', '}']);
        let name = vals.next().unwrap();
        let mut conditions = vec![];
        for val in vals {
            if val.is_empty() {
                break;
            }
            if let Some((cond, target)) = val.split_once(':') {
                let num: usize = cond[2..].parse().unwrap();
                let cond = match &cond[..2] {
                    "x>" => Condition::XMore(num, target),
                    "m>" => Condition::MMore(num, target),
                    "a>" => Condition::AMore(num, target),
                    "s>" => Condition::SMore(num, target),
                    "x<" => Condition::XLess(num, target),
                    "m<" => Condition::MLess(num, target),
                    "a<" => Condition::ALess(num, target),
                    "s<" => Condition::SLess(num, target),
                    _ => panic!(),
                };
                conditions.push(cond);
            } else {
                conditions.push(Condition::Direct(val));
                break;
            }
        }
        rules.insert(name, conditions);
    }

    for line in lines {
        let mut vals = line.split([',', '=', '}']).skip(1);
        let x = vals.next().unwrap().parse().unwrap();
        vals.next();
        let m = vals.next().unwrap().parse().unwrap();
        vals.next();
        let a = vals.next().unwrap().parse().unwrap();
        vals.next();
        let s = vals.next().unwrap().parse().unwrap();
        parts.push(Part { x, m, a, s });
    }
    (rules, parts)
}
