use std::collections::HashMap;
pub fn solve() {
    println!("---Day 7:---");
    let input = std::fs::read_to_string("data/7.txt").unwrap();
    let mut hands_scores: Vec<_> = input.lines().map(|line| parse_line(line)).collect();
    hands_scores.sort_unstable();
    let mut ans = 0;
    for (i, (_, score)) in hands_scores.iter().enumerate() {
        ans += (i + 1) * score;
    }
    println!("Part 1: {ans}");

    // Part 2
    for ((hand_type, hand), _) in &mut hands_scores {
        let (new_hand, new_hand_type) = part2_rules(hand, hand_type);
        *hand_type = new_hand_type;
        *hand = new_hand;
    }
    hands_scores.sort_unstable();
    let mut ans = 0;
    for (i, (_, score)) in hands_scores.into_iter().enumerate() {
        ans += (i + 1) * score;
    }
    println!("Part 2: {ans}");
}

fn parse_line(line: &str) -> ((HandType, String), usize) {
    let (hand, score) = line.split_once(' ').unwrap();
    let score = score.parse::<usize>().unwrap();
    let card: String = hand
        .chars()
        .map(|c| match c {
            d @ '2'..='9' => d,
            'T' => 'a',
            'J' => 'b',
            'Q' => 'c',
            'K' => 'd',
            'A' => 'e',
            x => panic!("unknown symbol: {x}"),
        })
        .collect();

    return ((HandType::from(hand), card), score);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&str> for HandType {
    fn from(hand: &str) -> Self {
        let mut counts = HashMap::new();
        let mut max_count = 0;
        for c in hand.chars() {
            let count = counts.entry(c).or_default();
            *count += 1;
            max_count = max_count.max(*count);
        }
        match (counts.keys().len(), max_count) {
            (5, 1) => Self::High,
            (4, 2) => Self::OnePair,
            (3, 2) => Self::TwoPair,
            (3, 3) => Self::ThreeOfAKind,
            (2, 3) => Self::FullHouse,
            (2, 4) => Self::FourOfAKind,
            (1, 5) => Self::FiveOfAKind,
            _ => panic!(),
        }
    }
}

fn part2_rules(hand: &str, hand_type: &HandType) -> (String, HandType) {
    let mut new_hand = String::new();
    let mut joker_count = 0;
    for c in hand.chars() {
        if c == 'b' {
            joker_count += 1;
            new_hand.push('0');
        } else {
            new_hand.push(c);
        }
    }

    let new_hand_type = match (hand_type, joker_count) {
        (hand_type, 0) => *hand_type,
        (HandType::High, _) => HandType::OnePair,
        (HandType::OnePair, _) => HandType::ThreeOfAKind,
        (HandType::TwoPair, 1) => HandType::FullHouse,
        (HandType::TwoPair, 2) => HandType::FourOfAKind,
        (HandType::ThreeOfAKind, _) => HandType::FourOfAKind,
        (HandType::FullHouse | HandType::FourOfAKind | HandType::FiveOfAKind, _) => {
            HandType::FiveOfAKind
        }
        (hand_type, joker_count) => panic!("{hand_type:?} {joker_count}"),
    };
    (new_hand, new_hand_type)
}
