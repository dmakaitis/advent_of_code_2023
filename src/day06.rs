#[derive(Debug)]
struct Outcome {
    _min: i64,
    _max: i64,
    count: i64,
}

fn solve(t: i64, d: i64) -> Outcome {
    let tf = t as f64;
    let df = d as f64;

    let s = (tf * tf - 4.0 * df).sqrt();

    let min = (tf - s) / 2.0;
    let max = (tf + s) / 2.0;

    let mut min = min.ceil() as i64;
    let mut max = max.floor() as i64;

    while min * (t - min) <= d {
        min += 1;
    }
    while max * (t - max) <= d {
        max -= 1;
    }

    let count = max - min + 1;

    Outcome {
        _min: min,
        _max: max,
        count,
    }
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i64 {
    let mut parts = input.lines();
    let times = parts.next().unwrap().strip_prefix("Time:").unwrap();
    let times: Result<Vec<_>, _> = times.split_whitespace().map(|n| n.parse::<i64>()).collect();
    let times = times.unwrap();

    let distances = parts.next().unwrap().strip_prefix("Distance:").unwrap();
    let distances: Result<Vec<_>, _> = distances
        .split_whitespace()
        .map(|n| n.parse::<i64>())
        .collect();
    let distances = distances.unwrap();

    let product = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| solve(*t, *d))
        .map(|o| o.count)
        .product();

    product
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i64 {
    let mut parts = input.lines();
    let times = parts
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .replace(' ', "");
    let time: i64 = times.parse().unwrap();

    let distances = parts
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .replace(' ', "");
    let distance: i64 = distances.parse().unwrap();

    let outcome = solve(time, distance);

    outcome.count
}

#[cfg(test)]
mod tests {
    use crate::day06::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            288
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            71503
        );
    }
}
