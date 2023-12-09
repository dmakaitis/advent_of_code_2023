fn get_appended(v: &[i32], d: i32) -> i32 {
    v[v.len() - 1] + d
}

fn get_prepended(v: &[i32], d: i32) -> i32 {
    v[0] - d
}

fn calc_end_value<F>(v: &[i32], f: &F) -> i32
where
    F: Fn(&[i32], i32) -> i32,
{
    // If everything is zero, then the next value is also zero
    if v.iter().all(|d| *d == 0) {
        return 0;
    }

    // Build next vector
    let mut next_vector = Vec::with_capacity(v.len() - 1);
    for i in 1..v.len() {
        next_vector.push(v[i] - v[i - 1]);
    }

    let next_diff = calc_end_value(&next_vector, f);

    f(v, next_diff)
}

fn calc_next_value(v: &[i32]) -> i32 {
    calc_end_value(v, &get_appended)
}

fn calc_previous_value(v: &[i32]) -> i32 {
    calc_end_value(v, &get_prepended)
}

fn to_vec(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i32 {
    input.lines().map(to_vec).map(|v| calc_next_value(&v)).sum()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(to_vec)
        .map(|v| calc_previous_value(&v))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day09::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            114
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            2
        );
    }
}
