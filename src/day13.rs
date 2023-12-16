fn encode_by_row(s: &str) -> Vec<i32> {
    let mut result = vec![];

    for line in s.lines() {
        let mut total = 0;

        for c in line.chars() {
            total <<= 1;
            if c == '#' {
                total += 1;
            }
        }

        result.push(total);
    }

    result
}

fn encode_by_column(s: &str) -> Vec<i32> {
    let width = s.lines().nth(0).unwrap().len();

    let mut result = vec![];

    for i in 0..width {
        let mut total = 0;

        for line in s.lines() {
            total <<= 1;

            let c = &line[i..i + 1];
            if c == "#" {
                total += 1;
            }
        }

        result.push(total);
    }

    result
}

fn is_reflection(left: &[i32], right: &[i32]) -> bool {
    let len = if left.len() < right.len() {
        left.len()
    } else {
        right.len()
    };

    for index in 0..len {
        if left[left.len() - 1 - index] != right[index] {
            return false;
        }
    }

    true
}

fn find_reflection(values: &[i32]) -> Option<usize> {
    let mut left: &[i32];
    let mut right: &[i32];
    let mut index = 1;

    while index < values.len() {
        left = &values[0..index];
        right = &values[index..];

        if is_reflection(left, right) {
            return Some(index);
        }

        index += 1;
    }
    None
}

fn get_reflection_value(s: &str) -> i32 {
    let values = encode_by_row(s);
    let reflection = find_reflection(&values);

    if reflection.is_some() {
        return 100 * (reflection.unwrap() as i32);
    }

    let values = encode_by_column(s);
    let reflection = find_reflection(&values);

    return reflection.unwrap() as i32;
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|line| get_reflection_value(line))
        .sum()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day13::*;

    #[test]
    fn test_encode_by_row() {
        assert_eq!(
            encode_by_row(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            ),
            vec![358, 90, 385, 385, 90, 102, 346]
        );
    }

    #[test]
    fn test_encode_by_column() {
        assert_eq!(
            encode_by_column(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            ),
            vec![89, 24, 103, 66, 37, 37, 66, 103, 24]
        );
    }

    #[test]
    fn test_find_reflection() {
        assert_eq!(
            find_reflection(&vec![358, 90, 385, 385, 90, 102, 346]),
            None
        );

        assert_eq!(
            find_reflection(&vec![89, 24, 103, 66, 37, 37, 66, 103, 24]),
            Some(5)
        );
    }
    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            405
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(part_two(""), 0);
    }
}
