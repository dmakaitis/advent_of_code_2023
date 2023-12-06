#[derive(Debug)]
struct Outcome {
    _min: i64,
    _max: i64,
    count: i64,
}

fn solve(t: i64, d: i64) -> Outcome {
    // The solutions for this puzzle are all integer values 'x' that satisfy:
    //
    // (time - s) / 2 < x < (time + s) / 2
    //
    // where s = sqrt(time^2 - 4 * distance)

    let tf = t as f64;
    let df = d as f64;

    let s = (tf * tf - 4.0 * df).sqrt();

    let min = (tf - s) / 2.0;
    let max = (tf + s) / 2.0;

    // At this point, min and max are the two real values for x where the total distance will
    // exactly equal the target, so we need to convert to integers:

    let mut min = min.ceil() as i64;
    let mut max = max.floor() as i64;

    // In the edge case where the real value for x above was already an integer, then we need
    // to ignore that value since we need our distance to be greater than the target, not equal,
    // so the following filters out any edge cases that might have been included above
    // (each while loop is expected to execute zero or one times):
    while min * (t - min) <= d {
        min += 1;
    }
    while max * (t - max) <= d {
        max -= 1;
    }

    // The number of solutions is now the number of integer values within the range:
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
