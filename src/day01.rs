/// Returns the value of the first digit found in the given string.
///
/// # Arguments
///
/// 'text' - The string to search for the first digit.
///
/// # Panic
///
/// Panics if no digit is found in the input string.
fn get_first_digit(text: &str) -> u32 {
    for c in text.chars() {
        match c.to_digit(10) {
            Some(digit) => {
                return digit;
            }
            None => {
                // ignore
            }
        }
    }

    panic!("No digit found")
}

/// Returns the value of the last digit found in the given string.
///
/// # Arguments
///
/// 'text' - The string to search for the last digit.
///
/// # Panic
///
/// Panics if no digit is found in the input string.
fn get_last_digit(text: &str) -> u32 {
    let mut last = 999;

    for c in text.chars() {
        match c.to_digit(10) {
            Some(digit) => {
                last = digit;
            }
            None => {
                // ignore
            }
        }
    }

    if last > 9 {
        panic!("No digit found")
    }

    last
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

/// Returns the value of the first digit or name of a digit found in the given string. A digit name
/// may be one of 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', or 'nine'.
///
/// # Arguments
///
/// 'text' - The string to search for the first digit or digit name.
///
/// # Panic
///
/// Panics if no digit or digit name is found in the input string.
fn get_first_digit_or_name(text: &str) -> u32 {
    let mut first_index = u32::MAX;
    let mut first_value = 0;

    for (name, value) in NAME_VALUE_MAP {
        if let Some(found_index) = text.find(name) {
            if (found_index as u32) < first_index {
                first_index = found_index as u32;
                first_value = *value;
            }
        }
    }

    first_value
}

/// Returns the value of the last digit or name of a digit found in the given string. A digit name
/// may be one of 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', or 'nine'.
///
/// # Arguments
///
/// 'text' - The string to search for the last digit or digit name.
///
/// # Panic
///
/// Panics if no digit or digit name is found in the input string.
fn get_last_digit_or_name(text: &str) -> u32 {
    let mut last_index = 0;
    let mut last_value = 0;

    for (name, value) in NAME_VALUE_MAP {
        if let Some(found_index) = text.rfind(name) {
            if (found_index as u32) >= last_index {
                last_index = found_index as u32;
                last_value = *value;
            }
        }
    }

    last_value
}

/// Returns the sum of the calibration values found in each line of the input.
pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| (get_first_digit(line), get_last_digit(line)))
        .map(|(f, l)| 10 * f + l)
        .sum()
}

/// Returns the sum of the calibration values found in each line of the input, taking into account
/// that the calibration values may be spelled out by name rather than using digit characters.
pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| (get_first_digit_or_name(line), get_last_digit_or_name(line)))
        .map(|(f, l)| 10 * f + l)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day01::*;

    #[test]
    fn can_retrieve_first_digit() {
        assert_eq!(get_first_digit("1abc2"), 1);
        assert_eq!(get_first_digit("pqr3stu8vwx"), 3);
        assert_eq!(get_first_digit("a1b2c3d4e5f"), 1);
        assert_eq!(get_first_digit("treb7uchet"), 7);
    }

    #[test]
    fn can_retrieve_last_digit() {
        assert_eq!(get_last_digit("1abc2"), 2);
        assert_eq!(get_last_digit("pqr3stu8vwx"), 8);
        assert_eq!(get_last_digit("a1b2c3d4e5f"), 5);
        assert_eq!(get_last_digit("treb7uchet"), 7);
    }

    #[test]
    fn can_retrieve_first_digit_or_digit_name_value() {
        assert_eq!(get_first_digit_or_name("two1nine"), 2);
        assert_eq!(get_first_digit_or_name("eightwothree"), 8);
        assert_eq!(get_first_digit_or_name("abcone2threexyz"), 1);
        assert_eq!(get_first_digit_or_name("xtwone3four"), 2);
        assert_eq!(get_first_digit_or_name("4nineeightseven2"), 4);
        assert_eq!(get_first_digit_or_name("zoneight234"), 1);
        assert_eq!(get_first_digit_or_name("7pqrstsixteen"), 7);
    }

    #[test]
    fn can_retrieve_last_digit_or_digit_name_value() {
        assert_eq!(get_last_digit_or_name("two1nine"), 9);
        assert_eq!(get_last_digit_or_name("eightwothree"), 3);
        assert_eq!(get_last_digit_or_name("abcone2threexyz"), 3);
        assert_eq!(get_last_digit_or_name("xtwone3four"), 4);
        assert_eq!(get_last_digit_or_name("4nineeightseven2"), 2);
        assert_eq!(get_last_digit_or_name("zoneight234"), 4);
        assert_eq!(get_last_digit_or_name("7pqrstsixteen"), 6);
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
