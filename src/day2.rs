pub fn solve() {
    let input = std::fs::read_to_string("data/2.txt").unwrap();

    let mut ans = 0;
    let mut ans2 = 0;
    for (i, line) in input.lines().enumerate() {
        let mut vals = line
            .split([',', ';', ' '])
            .skip(2) // skips "Game" "X:"
            .filter(|x| x.len() != 0);
        let (mut r, mut g, mut b) = (0, 0, 0);
        let mut feasible = true;
        loop {
            match (
                vals.next().map(|v| v.parse::<usize>().unwrap()),
                vals.next(),
            ) {
                (Some(x), Some("red")) => r = r.max(x),
                (Some(x), Some("green")) => g = g.max(x),
                (Some(x), Some("blue")) => b = b.max(x),
                (None, None) => break,
                _ => panic!(),
            };

            if r > 12 || g > 13 || b > 14 {
                feasible = false;
            }
        }

        if feasible {
            ans += i + 1; // i is 0 indexed
        }
        ans2 += r * g * b;
    }
    println!("----Day 2----");
    println!("Part 1: {ans}");
    println!("Part 2: {ans2}");
}
