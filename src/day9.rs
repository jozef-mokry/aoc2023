pub fn solve() {
    let input = std::fs::read_to_string("data/9.txt").unwrap();
    let mut ans = 0;
    let mut ans2 = 0;
    println!("---Day 9---");
    for line in input.lines() {
        let nums: Vec<_> = line
            .split_ascii_whitespace()
            .map(|v| v.parse::<isize>().unwrap())
            .collect();
        let (next, prev) = get_next_and_prev(nums);
        ans += next;
        ans2 += prev;
    }
    println!("Part 1: {ans}");
    println!("Part 2: {ans2}");
}

fn get_next_and_prev(mut nums: Vec<isize>) -> (isize, isize) {
    let mut last_vals = vec![];
    let mut first_vals = vec![];
    let mut toggle_factor = 1;
    while !nums.iter().all(|&v| v == 0) {
        first_vals.push(toggle_factor * nums[0]);
        toggle_factor *= -1;
        for i in 0..nums.len() - 1 {
            nums[i] = nums[i + 1] - nums[i];
        }
        last_vals.push(nums.pop().unwrap());
    }
    (
        last_vals.into_iter().sum::<isize>(),
        first_vals.into_iter().sum::<isize>(),
    )
}
