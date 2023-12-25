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

fn is_reflection_with_smudge(left: &[i32], right: &[i32]) -> bool {
    let len = if left.len() < right.len() {
        left.len()
    } else {
        right.len()
    };

    let mut diff_count = 0;
    let mut diff_value = 0;

    for index in 0..len {
        let lvalue = left[left.len() - 1 - index];
        let rvalue = right[index];

        if lvalue != rvalue {
            diff_count += 1;
            if diff_count > 1 {
                // If we have more than one difference, then this isn't a valid reflection
                return false;
            }

            diff_value = lvalue ^ rvalue;
        }
    }

    if diff_count != 1 {
        // Must be exactly one difference:
        return false;
    }

    // Difference must be by exactly one bit:
    let diff_less_one = diff_value - 1;
    return diff_value & diff_less_one == 0;
}

fn find_reflection(values: &[i32], smudge: bool) -> Option<usize> {
    let mut left: &[i32];
    let mut right: &[i32];
    let mut index = 1;

    while index < values.len() {
        left = &values[0..index];
        right = &values[index..];

        if !smudge && is_reflection(left, right) {
            return Some(index);
        } else if smudge && is_reflection_with_smudge(left, right) {
            return Some(index);
        }

        index += 1;
    }
    None
}

fn get_reflection_value(s: &str, smudge: bool) -> i32 {
    let values = encode_by_row(s);
    let reflection = find_reflection(&values, smudge);

    if reflection.is_some() {
        return 100 * (reflection.unwrap() as i32);
    }

    let values = encode_by_column(s);
    let reflection = find_reflection(&values, smudge);

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
        .map(|line| get_reflection_value(line, false))
        .sum()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|line| get_reflection_value(line, true))
        .sum()
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
            vec![
                0b101100110,
                0b001011010,
                0b110000001,
                0b110000001,
                0b001011010,
                0b001100110,
                0b101011010
            ]
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
            vec![
                0b1011001, 0b0011000, 0b1100111, 0b1000010, 0b0100101, 0b0100101, 0b1000010,
                0b1100111, 0b0011000
            ]
        );
    }

    #[test]
    fn test_find_reflection() {
        assert_eq!(
            find_reflection(
                &vec![
                    0b101100110,
                    0b001011010,
                    0b110000001,
                    0b110000001,
                    0b001011010,
                    0b001100110,
                    0b101011010
                ],
                false
            ),
            None
        );

        assert_eq!(
            find_reflection(
                &vec![
                    0b1011001, 0b0011000, 0b1100111, 0b1000010, 0b0100101, 0b0100101, 0b1000010,
                    0b1100111, 0b0011000
                ],
                false
            ),
            Some(5)
        );

        assert_eq!(
            find_reflection(
                &vec![
                    0b101100110,
                    0b001011010,
                    0b110000001,
                    0b110000001,
                    0b001011010,
                    0b001100110,
                    0b101011010
                ],
                true
            ),
            Some(3)
        );

        assert_eq!(
            find_reflection(
                &vec![
                    0b100011001,
                    0b100001001,
                    0b001100111,
                    0b111110110,
                    0b111110110,
                    0b001100111,
                    0b100001001
                ],
                true
            ),
            Some(1)
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
        assert_eq!(
            part_two(
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
            400
        );

        assert_eq!(
            part_two(
                ".......#.####
#..#.##.#####
.#...#..#....
#..#.###.....
.##.#..##.##.
#####.#..####
....######..#"
            ),
            2
        );
    }
}
