pub fn solve() {
    let input = std::fs::read_to_string("data/6.txt").unwrap();
    println!("----Day 6:----");
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|v| v.parse::<usize>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|v| v.parse::<usize>().unwrap());

    let mut ans: usize = 1;
    for (t, d) in times.zip(distances) {
        ans *= num_of_ways(t, d);
    }
    println!("Part 1: {ans}");
}

fn part2(input: &str) {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    println!("Part 2: {}", num_of_ways(time, distance));
}

fn num_of_ways(t: usize, d: usize) -> usize {
    // We want (t - x)*x > d
    // tx - x^2 - d > 0
    // x^2 - tx + d < 0
    // hence (t - sqrt(t^2 - 4d))/2 < x < (t + sqrt(t^2 - 4d))/2
    // because of symmetry, the number of ways that do NOT work are:
    // 2 * (floor((t - sqrt(t^2 - 4d))/2) + 1)
    // Total number of ways: t + 1

    let discriminant = ((t * t - 4 * d) as f64).sqrt();
    ((t + 1) as f64 - 2. * (((t as f64 - discriminant) / 2.).floor() + 1.)) as usize
}
