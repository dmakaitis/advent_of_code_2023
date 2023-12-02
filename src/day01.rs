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

    0
}

fn get_last_digit(text: &str) -> u32 {
    let mut last = 0;

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

pub fn part_one(input: &str) -> u32 {
    input.lines()
        .map(|line| (get_first_digit(line), get_last_digit(line)))
        .map(|(f, l)| 10 * f + l)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input.lines()
        .map(|line| (get_first_digit_or_name(line), get_last_digit_or_name(line)))
        .map(|(f, l)| 10 * f + l)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day01::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(part_one("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"), 142);
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(part_two("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"), 281);
    }
}
