use std::ops::{Add, Mul, Sub};
pub fn solve() {
    println!("---Day 24---");
    let input = std::fs::read_to_string("data/24.txt").unwrap();
    let lines = input.lines().map(|l| parse(l)).collect::<Vec<_>>();

    let start = 200000000000000.;
    let end = 400000000000000.;

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
    let bullet = find_bullet(&lines);
    println!("Part 2: {}", bullet.pos.x + bullet.pos.y + bullet.pos.z);
}

#[derive(Debug, Clone, Copy)]
struct Line {
    pos: Vec3,
    v: Vec3,
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Vec3 {
    x: i128,
    y: i128,
    z: i128,
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = i128;
    fn mul(self, b: Self) -> Self::Output {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
}
impl Mul<Vec3> for i128 {
    type Output = Vec3;
    fn mul(self, b: Vec3) -> Self::Output {
        Self::Output {
            x: self * b.x,
            y: self * b.y,
            z: self * b.z,
        }
    }
}

impl Line {
    fn intersect2d(&self, b: &Self) -> Option<(f64, f64)> {
        let a = self;
        let v_cross = cross(&a.v, &b.v);
        if v_cross.z != 0 {
            // directions are not parallel
            let t = cross(&b.pos, &b.v).z - cross(&a.pos, &b.v).z;
            let s = cross(&a.pos, &a.v).z - cross(&b.pos, &a.v).z;
            let f = if v_cross.z > 0 { 1 } else { -1 };
            if f * t < 0 || s * -f < 0 {
                return None;
            }
            return Some((
                a.pos.x as f64 + (t as f64 * a.v.x as f64) / (v_cross.z as f64),
                a.pos.y as f64 + (t as f64 * a.v.y as f64) / (v_cross.z as f64),
            ));
        } else {
            // directions are parallel
            let diff = Vec3 {
                x: a.pos.x - b.pos.x,
                y: a.pos.y - b.pos.y,
                z: 0,
            };
            if (diff.x == 0 && a.v.x != 0)
                || (diff.y == 0 && a.v.y != 0)
                || (a.v.x * diff.y != a.v.y * diff.x)
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
    let mut pos = pos.split(", ").map(|v| v.trim().parse::<i128>().unwrap());
    let mut speed = speed.split(", ").map(|v| v.trim().parse::<i128>().unwrap());

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

fn find_bullet(lines: &[Line]) -> Line {
    // Let A = (x,y,z) be the position we are looking for and B = (vx, vy, vz) the speed
    // Then in order for an intersection to exist with line 0, we must have:
    // l0.pos + t0 * l0.v = A + t0 * B
    // and similarly for other lines
    //
    // We can re-arrange the above into:
    // l0.pos - A = t0* (B - l0.v)
    // and by cross-producting both sides we get:
    // (l0.pos - A) x (B - l0.v) = 0
    // which when expanded:
    // (* is cross-product)
    // l0.pos*B - l0.pos*l0.v - A*B + A*l0.v = 0
    //
    // and similarly for line 1 l1 and line 2 l2:
    // l1.pos*B - l1.pos*l1.v - A*B + A*l1.v = 0
    // l2.pos*B - l1.pos*l2.v - A*B + A*l2.v = 0
    //
    // By expressing A*B from l0 equation:
    // A*B = l0.pos*B - l0.pos*l0.v + A*l0.v
    //
    // and plugging into line 1 and line 2 eqs:
    // (l1.pos-l0.pos)*B + (l0.v - l1.v)*A = l1.pos*l1.v - l0.pos*l0.v
    // (l2.pos-l0.pos)*B + (l0.v - l2.v)*A = l2.pos*l2.v - l0.pos*l0.v
    //
    // Let p1 = (l1.pos-l0.pos)
    // Let p2 = (l2.pos-l0.pos)
    // Let v1 = (l0.v - l1.v)
    // Let v2 = (l0.v - l2.v)
    // Let c1 = l1.pos*l1.v - l0.pos*l0.v
    // Let c2 = l2.pos*l2.v - l0.pos*l0.v
    //
    // Then we just need to construct the matrix M from p1, p2, v1, v2.
    // The x vector is [A.x, A.y, A.z, B.x, B.y, B.z]
    // and the RHS c is [c1.x, c1.y, c1.z, c2.x, c2.y, c2.z]
    // and finally solve: Mx = c
    let (l0, l1, l2) = (lines[0], lines[1], lines[2]);

    let p1 = l1.pos - l0.pos;
    let p2 = l2.pos - l0.pos;
    let v1 = l0.v - l1.v;
    let v2 = l0.v - l2.v;
    let c1 = cross(&l1.pos, &l1.v) - cross(&l0.pos, &l0.v);
    let c2 = cross(&l2.pos, &l2.v) - cross(&l0.pos, &l0.v);

    // The special pattern in the matrix is implementing the cross-product
    let m = [
        [v1.z, 0, -v1.x, p1.z, 0, -p1.x],
        [-v1.y, v1.x, 0, -p1.y, p1.x, 0],
        [0, -v2.z, v2.y, 0, -p2.z, p2.y],
        [v2.z, 0, -v2.x, p2.z, 0, -p2.x],
        [-v2.y, v2.x, 0, -p2.y, p2.x, 0],
        [0, -v1.z, v1.y, 0, -p1.z, p1.y], // first line was moved to end to make diagonalization simpler; we don't like zeros on the diagonal
    ];
    let c = [c1.y, c1.z, c2.x, c2.y, c2.z, c1.x]; // first value was moved to end to match the move of the row
    let mut m = m.map(|row| row.map(|x| x as f64));
    let mut c = c.map(|x| x as f64);

    // solve M * x = C by turning into upper triangular
    for col in 0..6 {
        // We clear column col
        // subtract row from all rows below to clear column col
        let a = m[col][col];
        for r in col + 1..6 {
            let b = m[r][col];
            m[r][col] = 0.;
            for c in col + 1..6 {
                m[r][c] = m[r][c] - m[col][c] * (b / a);
            }
            c[r] = c[r] - c[col] * (b / a);
        }
    }

    let mut ans = [0.; 6];
    for i in (0..6).rev() {
        let s = ans
            .iter()
            .zip(m[i].iter())
            .fold(0., |acc, (&a, &b)| acc + a * b);
        ans[i] = (c[i] - s) / m[i][i];
    }

    Line {
        pos: Vec3 {
            x: ans[0].round() as i128,
            y: ans[1].round() as i128,
            z: ans[2].round() as i128,
        },
        v: Vec3 {
            x: ans[3].round() as i128,
            y: ans[4].round() as i128,
            z: ans[5].round() as i128,
        },
    }
}
