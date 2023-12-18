pub fn solve() {
    println!("---Day 18---");
    let input = std::fs::read_to_string("data/18.txt").unwrap();

    println!("Part 1: {}", calc_area(&input, false));
    println!("Part 2: {}", calc_area(&input, true));
}

fn calc_area(input: &str, part2_parse: bool) -> isize {
    let mut x = 0;
    let mut y = 0;
    let mut area: isize = 0;
    let mut boundary_len = 0;
    for line in input.lines() {
        let mut vals = line.split_ascii_whitespace();
        let (dir, dist) = if !part2_parse {
            let dir = vals.next().unwrap();
            let dist = vals.next().unwrap().parse::<isize>().unwrap();
            (dir, dist)
        } else {
            let val = vals.last().unwrap();
            let dist = isize::from_str_radix(&val[2..val.len() - 2], 16).unwrap();
            let dir = match &val[val.len() - 2..val.len() - 1] {
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                _ => panic!(),
            };
            (dir, dist)
        };

        let (new_x, new_y) = match dir {
            "R" => (x + dist, y),
            "L" => (x - dist, y),
            "U" => (x, y - dist),
            "D" => (x, y + dist),
            _ => panic!(),
        };
        // Shoelace formula:
        // 2*A = sum( (y_i + y_i+1) * (x_i - x_i+1) )
        // but because we want the area of the squares that the line goes through, we need to add
        // line_len / 2 + 1
        // This is because for each straight line segment of length 1, we need to add 1/2 unit of area.
        // Corners are trickier. For each RIGHT corner we need to add/subtract 1/4 unit of area and
        // for each LEFT corner we need to subtract/add 1/4 unit of area. But fortunately the
        // difference between LEFT and RIGHT turns is always 4 (because we end up where we
        // started). And so we add 1/4*4 = 1 unit of arealeft corner we need to subtract/add 1/4
        // unit of area.

        area += (y + new_y) * (x - new_x);
        boundary_len += dist;
        x = new_x;
        y = new_y;
    }
    debug_assert!(area % 2 == 0);
    debug_assert!(boundary_len % 2 == 0);
    area / 2 + boundary_len / 2 + 1
}
