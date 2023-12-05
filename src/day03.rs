use std::str::FromStr;

/// A symbol located at a indexed location
#[derive(Debug, PartialEq)]
struct Symbol {
    value: char,
    index: usize,
}

impl Symbol {
    /// Creates a new symbol with the given value and location.
    fn new(value: char, index: usize) -> Symbol {
        Symbol { value, index }
    }
}

/// A number at an indexed location
#[derive(Debug, PartialEq)]
struct Number {
    value: u32,
    start: usize,
    end: usize,
}

impl Number {
    /// Creates a number with the given value and location.
    fn new(value: u32, start: usize, end: usize) -> Number {
        Number { value, start, end }
    }
}

/// A single row from the schematic containing all numbers and symbols and their locations.
#[derive(Debug, PartialEq)]
struct Row {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Row {
            numbers: get_indexed_numbers(s),
            symbols: get_symbol_indices(s),
        })
    }
}

/// Returns the locations and values of all symbols in the row.
///
/// #Argument
///
/// 'text' - The schematic row to parse.
fn get_symbol_indices(text: &str) -> Vec<Symbol> {
    text.chars()
        .enumerate()
        .filter(|(_, c)| !c.is_numeric() && *c != '.')
        .map(|(i, c)| Symbol::new(c, i))
        .collect()
}

/// Returns the locations and values of all numbers in the row.
///
/// #Argument
///
/// 'text' - The schematic row to parse.
fn get_indexed_numbers(text: &str) -> Vec<Number> {
    let mut result = vec![];
    let mut total = 0;
    let mut start = usize::MAX;

    for (i, c) in text.chars().enumerate() {
        if c.is_numeric() {
            total = 10 * total + c.to_digit(10).unwrap();
            if start == usize::MAX {
                start = i
            }
        } else if start < usize::MAX {
            result.push(Number::new(total, start, i));
            start = usize::MAX;
            total = 0;
        }
    }

    // Make sure we've inserted any number into the buffer...
    if start < usize::MAX {
        result.push(Number::new(total, start, text.len()));
    }

    result
}

/// Calculates the sum of all numbers in the middle row that have symbols adjacent to them in
/// any of the given rows.
///
/// #Argument
///
/// 'a' - The row above row 'b'.
/// 'b' - The row with the numbers to identify any sum.
/// 'c' - The row below row 'b'.
fn calculate_sum(a: &Row, b: &Row, c: &Row) -> u32 {
    let all_symbols: Vec<_> = a
        .symbols
        .iter()
        .chain(b.symbols.iter())
        .chain(c.symbols.iter())
        .collect();

    b.numbers
        .iter()
        .filter(|n| {
            all_symbols
                .iter()
                .filter(|s| n.start <= s.index + 1)
                .any(|s| n.end >= s.index)
        })
        .map(|n| n.value)
        .sum()
}

/// Calculates the gear ratio for all '*'s in row 'b' using the adjacent numbers in all
/// three provided rows.
///
/// #Argument
///
/// 'a' - The row above row 'b'.
/// 'b' - The row with the '*' symbols to locate and calculate.
/// 'c' - The row below row 'b'.
fn calculate_product(a: &Row, b: &Row, c: &Row) -> u32 {
    let all_numbers: Vec<_> = a
        .numbers
        .iter()
        .chain(b.numbers.iter())
        .chain(c.numbers.iter())
        .collect();

    b.symbols
        .iter()
        .filter(|s| s.value == '*')
        .map(|s| {
            let result: Vec<_> = all_numbers
                .iter()
                .filter(|n| n.start <= s.index + 1)
                .filter(|n| n.end >= s.index)
                .collect();
            result
        })
        .filter(|n| n.len() == 2)
        .map(|n| n[0].value * n[1].value)
        .sum()
}

/// Scans the entire schematic and calculates a value based upon it based on the provided
/// calculation function.
///
/// # Argumetns
///
/// 'schematic' - The schematic to scan.
///
/// 'func' - The calculation function to be used to produce a value from the schematic.
fn scan_schematic<F>(schematic: &str, func: F) -> u32
where
    F: Fn(&Row, &Row, &Row) -> u32,
{
    let parsed: Vec<_> = schematic
        .lines()
        .map(Row::from_str)
        .map(Result::unwrap)
        .collect();

    let empty_row = "".parse().unwrap();

    let mut total = 0;

    for (i, r) in parsed.iter().enumerate() {
        let last = if i == 0 { &empty_row } else { &parsed[i - 1] };
        let next = if i == parsed.len() - 1 {
            &empty_row
        } else {
            &parsed[i + 1]
        };

        total += func(last, r, next);
    }

    total
}

/// Calculates the sum of all numbers in the schematic that have an adjacent symbol.
///
/// #Argument
///
/// 'input' - The input schematic.
pub fn part_one(input: &str) -> u32 {
    scan_schematic(input, calculate_sum)
}

/// Calculates the product of all gears in the schematic that have two adjacent numbers.
///
/// #Argument
///
/// 'input' - The input schematic.
pub fn part_two(input: &str) -> u32 {
    scan_schematic(input, calculate_product)
}

#[cfg(test)]
mod tests {
    use crate::day03::*;

    #[test]
    fn find_symbols() {
        assert_eq!(get_symbol_indices("...*......"), vec![Symbol::new('*', 3)]);
        assert_eq!(get_symbol_indices("..35..633."), vec![]);
        assert_eq!(
            get_symbol_indices("...$.*...."),
            vec![Symbol::new('$', 3), Symbol::new('*', 5)]
        );
        assert_eq!(
            get_symbol_indices("...$.*...$"),
            vec![
                Symbol::new('$', 3),
                Symbol::new('*', 5),
                Symbol::new('$', 9),
            ]
        );
    }

    #[test]
    fn find_numbers() {
        assert_eq!(get_indexed_numbers("...*......"), vec![]);
        assert_eq!(
            get_indexed_numbers("..35..633."),
            vec![Number::new(35, 2, 4), Number::new(633, 6, 9)]
        );
        assert_eq!(get_indexed_numbers("...$.*...."), vec![]);
        assert_eq!(
            get_indexed_numbers("35.....633"),
            vec![Number::new(35, 0, 2), Number::new(633, 7, 10)]
        );
    }

    #[test]
    fn row_parsing() {
        assert_eq!(
            Row::from_str("...*......").unwrap(),
            Row {
                numbers: vec![],
                symbols: vec![Symbol::new('*', 3)]
            }
        );
        assert_eq!(
            Row::from_str("..35..633.").unwrap(),
            Row {
                numbers: vec![Number::new(35, 2, 4), Number::new(633, 6, 9)],
                symbols: vec![]
            }
        );
        assert_eq!(
            Row::from_str("...$.*....").unwrap(),
            Row {
                numbers: vec![],
                symbols: vec![Symbol::new('$', 3), Symbol::new('*', 5)]
            }
        );
    }

    #[test]
    fn calculate_sum_for_three_adjacent_rows() {
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("...#......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::from_str("#.........").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str(".#........").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..#.......").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("...#......").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("....#.....").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str(".....#....").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("......#...").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str(".......#..").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("........#.").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str(".........#").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("#.35......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str(".#35......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35#.....").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35.#....").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35..#...").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35...#..").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35....#.").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35.....#").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("#.........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str(".#........").unwrap(),
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("..#.......").unwrap(),
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("...#......").unwrap(),
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("....#.....").unwrap(),
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str(".....#....").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("......#...").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str(".......#..").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str("........#.").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("..35......").unwrap(),
                &Row::from_str(".........#").unwrap(),
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::from_str("#.........").unwrap(),
                &Row::from_str("617.......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str(".#........").unwrap(),
                &Row::from_str("617.......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..#.......").unwrap(),
                &Row::from_str("617.......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("...#......").unwrap(),
                &Row::from_str("617.......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("....#.....").unwrap(),
                &Row::from_str("617.......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("617#......").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("617.#.....").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("617.......").unwrap(),
                &Row::from_str("#.........").unwrap(),
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("617.......").unwrap(),
                &Row::from_str(".#........").unwrap(),
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("617.......").unwrap(),
                &Row::from_str("..#.......").unwrap(),
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("617.......").unwrap(),
                &Row::from_str("...#......").unwrap(),
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("617.......").unwrap(),
                &Row::from_str("....#.....").unwrap(),
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::from_str(".....#....").unwrap(),
                &Row::from_str(".......123").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("......#...").unwrap(),
                &Row::from_str(".......123").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            123
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str(".......#..").unwrap(),
                &Row::from_str(".......123").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            123
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("........#.").unwrap(),
                &Row::from_str(".......123").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            123
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str(".........#").unwrap(),
                &Row::from_str(".......123").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            123
        );

        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str(".....#.123").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str("......#123").unwrap(),
                &Row::from_str("..........").unwrap(),
            ),
            123
        );

        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str(".......123").unwrap(),
                &Row::from_str(".....#....").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str(".......123").unwrap(),
                &Row::from_str("......#...").unwrap(),
            ),
            123
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str(".......123").unwrap(),
                &Row::from_str(".......#..").unwrap(),
            ),
            123
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str(".......123").unwrap(),
                &Row::from_str("........#.").unwrap(),
            ),
            123
        );
        assert_eq!(
            calculate_sum(
                &Row::from_str("..........").unwrap(),
                &Row::from_str(".......123").unwrap(),
                &Row::from_str(".........#").unwrap(),
            ),
            123
        );
    }

    #[test]
    fn calculate_product_for_three_adjacent_rows() {
        assert_eq!(
            calculate_product(
                &Row::from_str("..35..633.").unwrap(),
                &Row::from_str("......#...").unwrap(),
                &Row::from_str("617*......").unwrap(),
            ),
            0
        );
        assert_eq!(
            calculate_product(
                &Row::from_str("467..114..").unwrap(),
                &Row::from_str("...*......").unwrap(),
                &Row::from_str("..35..633.").unwrap(),
            ),
            16345
        );
        assert_eq!(
            calculate_product(
                &Row::from_str("......755.").unwrap(),
                &Row::from_str("...$.*....").unwrap(),
                &Row::from_str(".664.598..").unwrap(),
            ),
            451490
        );
    }

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            4361
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            467835
        );
    }
}
