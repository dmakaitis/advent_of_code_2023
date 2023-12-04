use std::cmp::Ordering;

/// Returns the value of the first digit found in the given string.
///
/// # Arguments
///
/// 'text' - The string to search for the first digit.
fn get_first_digit(text: &str) -> Option<u32> {
    text.chars().filter_map(|c| c.to_digit(10)).next()
}

/// Returns the value of the last digit found in the given string.
///
/// # Arguments
///
/// 'text' - The string to search for the last digit.
fn get_last_digit(text: &str) -> Option<u32> {
    text.chars().rev().filter_map(|c| c.to_digit(10)).next()
}

const NAME_VALUE_MAP: &[(&str, u32)] = &[
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("0", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_sorted_digit_or_name<F>(text: &str, sorter: F) -> Option<u32>
where
    F: Fn(&(u32, usize), &(u32, usize)) -> Ordering,
{
    let mut vec: Vec<_> = NAME_VALUE_MAP
        .iter()
        .map(|(name, value)| (*value, text.find(name)))
        .filter(|(_, found)| found.is_some())
        .map(|(value, found)| (value, found.unwrap()))
        .collect();

    vec.sort_by(sorter);

    vec.iter().map(|(value, _)| *value).next()
}

/// Returns the value of the first digit or name of a digit found in the given string. A digit name
/// may be one of 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', or 'nine'.
///
/// # Arguments
///
/// 'text' - The string to search for the first digit or digit name.
fn get_first_digit_or_name(text: &str) -> Option<u32> {
    get_sorted_digit_or_name(text, |(_, a), (_, b)| a.cmp(b))
}

/// Returns the value of the last digit or name of a digit found in the given string. A digit name
/// may be one of 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', or 'nine'.
///
/// # Arguments
///
/// 'text' - The string to search for the last digit or digit name.
fn get_last_digit_or_name(text: &str) -> Option<u32> {
    get_sorted_digit_or_name(text, |(_, b), (_, a)| a.cmp(b))
}

/// Returns the sum of the calibration values found in each line of the input.
pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            (
                get_first_digit(line).unwrap(),
                get_last_digit(line).unwrap(),
            )
        })
        .map(|(f, l)| 10 * f + l)
        .sum()
}

/// Returns the sum of the calibration values found in each line of the input, taking into account
/// that the calibration values may be spelled out by name rather than using digit characters.
pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            (
                get_first_digit_or_name(line).unwrap(),
                get_last_digit_or_name(line).unwrap(),
            )
        })
        .map(|(f, l)| 10 * f + l)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day01::*;

    #[test]
    fn can_retrieve_first_digit() {
        assert_eq!(get_first_digit("1abc2"), Some(1));
        assert_eq!(get_first_digit("pqr3stu8vwx"), Some(3));
        assert_eq!(get_first_digit("a1b2c3d4e5f"), Some(1));
        assert_eq!(get_first_digit("treb7uchet"), Some(7));
        assert_eq!(get_first_digit("trebuchet"), None);
    }

    #[test]
    fn can_retrieve_last_digit() {
        assert_eq!(get_last_digit("1abc2"), Some(2));
        assert_eq!(get_last_digit("pqr3stu8vwx"), Some(8));
        assert_eq!(get_last_digit("a1b2c3d4e5f"), Some(5));
        assert_eq!(get_last_digit("treb7uchet"), Some(7));
        assert_eq!(get_last_digit("trebuchet"), None);
    }

    #[test]
    fn can_retrieve_first_digit_or_digit_name_value() {
        assert_eq!(get_first_digit_or_name("two1nine"), Some(2));
        assert_eq!(get_first_digit_or_name("eightwothree"), Some(8));
        assert_eq!(get_first_digit_or_name("abcone2threexyz"), Some(1));
        assert_eq!(get_first_digit_or_name("xtwone3four"), Some(2));
        assert_eq!(get_first_digit_or_name("4nineeightseven2"), Some(4));
        assert_eq!(get_first_digit_or_name("zoneight234"), Some(1));
        assert_eq!(get_first_digit_or_name("7pqrstsixteen"), Some(7));
        assert_eq!(get_first_digit_or_name("pqrstsaxteen"), None);
    }

    #[test]
    fn can_retrieve_last_digit_or_digit_name_value() {
        assert_eq!(get_last_digit_or_name("two1nine"), Some(9));
        assert_eq!(get_last_digit_or_name("eightwothree"), Some(3));
        assert_eq!(get_last_digit_or_name("abcone2threexyz"), Some(3));
        assert_eq!(get_last_digit_or_name("xtwone3four"), Some(4));
        assert_eq!(get_last_digit_or_name("4nineeightseven2"), Some(2));
        assert_eq!(get_last_digit_or_name("zoneight234"), Some(4));
        assert_eq!(get_last_digit_or_name("7pqrstsixteen"), Some(6));
        assert_eq!(get_first_digit_or_name("7pqrstsaxteen"), Some(7));
        assert_eq!(get_first_digit_or_name("pqrstsaxteen"), None);
    }

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        );
    }
}
