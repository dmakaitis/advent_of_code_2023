use itertools::Itertools;
use rulinalg::matrix::Matrix;
use std::ops::Mul;

#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl From<&str> for Vector {
    fn from(value: &str) -> Self {
        let (x, y, z) = value
            .split(',')
            .map(|s| s.trim())
            .flat_map(|s| s.parse::<f64>())
            .collect_tuple()
            .unwrap();

        Vector { x, y, z }
    }
}

#[derive(Debug)]
struct Hailstone {
    pos: Vector,
    vec: Vector,
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let (pos, vec) = value.split('@').map(Vector::from).collect_tuple().unwrap();

        Hailstone { pos, vec }
    }
}

fn get_intersect_x_y(a: &Hailstone, b: &Hailstone) -> Option<(f64, f64)> {
    let den = a.vec.y * b.vec.x - a.vec.x * b.vec.y;
    if den == 0.0 {
        return None;
    }

    let num = b.vec.x * (b.pos.y - a.pos.y) - b.vec.y * (b.pos.x - a.pos.x);
    let c = num / den;
    if c < 0.0 {
        return None;
    }

    let num = a.vec.x * (b.pos.y - a.pos.y) - a.vec.y * (b.pos.x - a.pos.x);
    let d = num / den;
    if d < 0.0 {
        return None;
    }

    Some((a.pos.x + c * a.vec.x, a.pos.y + c * a.vec.y))
}

fn solve_part_one(input: &str, bounds: (f64, f64)) -> i32 {
    let stones = input.lines().map(Hailstone::from).collect_vec();

    let mut count = 0;

    for (i, a) in stones.iter().enumerate() {
        for b in stones.iter().skip(i + 1) {
            if let Some((x, y)) = get_intersect_x_y(a, b) {
                if x >= bounds.0 && x <= bounds.1 && y >= bounds.0 && y <= bounds.1 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn get_coefficients(h1: &Hailstone, h2: &Hailstone) -> (Vec<f64>, Vec<f64>) {
    let a = h2.vec.y - h1.vec.y;
    let b = h1.vec.x - h2.vec.x;
    let d = h1.pos.y - h2.pos.y;
    let e = h2.pos.x - h1.pos.x;

    let v = h1.vec.x * h1.pos.y - h2.vec.x * h2.pos.y + h2.pos.x * h2.vec.y - h1.pos.x * h1.vec.y;

    (vec![a, b, d, e], vec![v])
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i32 {
    solve_part_one(input, (200000000000000.0, 400000000000000.0))
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i64 {
    let stones = input.lines().map(Hailstone::from).collect_vec();

    let mut coefficients = (vec![], vec![]);

    let h1 = &stones[0];
    for h2 in stones.iter().take(5).skip(1) {
        let mut c = get_coefficients(h1, h2);

        coefficients.0.append(&mut c.0);
        coefficients.1.append(&mut c.1);
    }

    let a = Matrix::new(4, 4, coefficients.0);
    let x = Matrix::new(4, 1, coefficients.1);

    let result = a.inverse().unwrap().mul(x).into_vec();
    let (a, b, d, e) = result.iter().map(|v| *v).collect_tuple().unwrap();

    let t1 = (a - h1.pos.x) / (h1.vec.x - d);

    let h2 = &stones[1];
    let t2 = (a - h2.pos.x) / (h2.vec.x - d);

    let f = ((h1.pos.z - h2.pos.z) + t1 * h1.vec.z - t2 * h2.vec.z) / (t1 - t2);
    let c = h1.pos.z + t1 * (h1.vec.z - f);

    // Convert everything to integer values to get rid of decimal imprecisions...
    let (a, b, c, _d, _e, _f) = vec![a, b, c, d, e, f]
        .iter()
        .map(|v| *v as i64)
        .collect_tuple()
        .unwrap();

    a + b + c
}

#[cfg(test)]
mod tests {
    use crate::day24::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            solve_part_one(
                "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3",
                (7.0, 27.0)
            ),
            2
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"
            ),
            47
        );
    }
}
