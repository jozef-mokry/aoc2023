pub fn solve() {
    println!("---Day 24---");
    let input = std::fs::read_to_string("data/24.txt").unwrap();
    let lines = input.lines().map(|l| parse(l)).collect::<Vec<_>>();

    let start = 200000000000000f64;
    let end = 400000000000000f64;

    let mut ans = 0;
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if let Some((x, y)) = lines[i].intersect2d(&lines[j]) {
                if start <= x && x <= end && start <= y && y <= end {
                    ans += 1;
                }
            }
        }
    }
    println!("Part 1: {ans}");
}

#[derive(Debug)]
struct Line {
    pos: Vec3,
    v: Vec3,
}

#[derive(Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Line {
    fn intersect2d(&self, b: &Self) -> Option<(f64, f64)> {
        let a = self;
        let v_cross = cross(&a.v, &b.v);
        if v_cross.z != 0. {
            // directions are not parallel
            let t = (cross(&b.pos, &b.v).z - cross(&a.pos, &b.v).z) / v_cross.z;
            let s = (cross(&a.pos, &a.v).z - cross(&b.pos, &a.v).z) / -v_cross.z;
            if t < 0. || s < 0. {
                return None;
            }
            return Some((a.pos.x + t * a.v.x, a.pos.y + t * a.v.y));
        } else {
            // directions are parallel
            let diff = Vec3 {
                x: a.pos.x - b.pos.x,
                y: a.pos.y - b.pos.y,
                z: 0.,
            };
            if (diff.x == 0. && a.v.x != 0.)
                || (diff.y == 0. && a.v.y != 0.)
                || (a.v.x / diff.x != a.v.y / diff.y)
            {
                // vector from a.pos to b.pos is not parallel to directions
                return None;
            }

            // infinitely many intersections
            panic!("unhandled parallel lines: {self:?} {b:?}");
        }
    }
}

fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - b.y * a.z,
        y: -a.x * b.z + b.x * a.z,
        z: a.x * b.y - b.x * a.y,
    }
}
fn parse(l: &str) -> Line {
    let (pos, speed) = l.split_once(" @ ").unwrap();
    let mut pos = pos.split(", ").map(|v| v.trim().parse::<f64>().unwrap());
    let mut speed = speed.split(", ").map(|v| v.trim().parse::<f64>().unwrap());

    Line {
        pos: Vec3 {
            x: pos.next().unwrap(),
            y: pos.next().unwrap(),
            z: pos.next().unwrap(),
        },
        v: Vec3 {
            x: speed.next().unwrap(),
            y: speed.next().unwrap(),
            z: speed.next().unwrap(),
        },
    }
}
