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

impl Row {
    /// Creates a row with the given numbers and symbols and their locations.
    fn new(numbers: Vec<Number>, symbols: Vec<Symbol>) -> Row {
        Row { numbers, symbols }
    }
}

/// Returns the locations and values of all symbols in the row.
///
/// #Argument
///
/// 'text' - The schematic row to parse.
fn get_symbol_indices(text: &str) -> Vec<Symbol> {
    let mut result = vec![];

    for (i, c) in text.chars().enumerate() {
        if !c.is_numeric() && c != '.' {
            result.push(Symbol::new(c, i));
        }
    }

    result
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

/// Parsed a row from the schematic, return the values and locations of all symbols and numbers.
///
/// #Argument
///
/// 'text' - The schematic row to parse.
fn parse_row(text: &str) -> Row {
    Row::new(get_indexed_numbers(text), get_symbol_indices(text))
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

/// Calculates the sum of all numbers in the schematic that have an adjacent symbol.
///
/// #Argument
///
/// 'input' - The input schematic.
pub fn part_one(input: &str) -> u32 {
    let parsed: Vec<_> = input.lines().map(parse_row).collect();

    let empty_row = Row::new(vec![], vec![]);

    let mut total = 0;

    for (i, r) in parsed.iter().enumerate() {
        let last = if i == 0 { &empty_row } else { &parsed[i - 1] };
        let next = if i == parsed.len() - 1 {
            &empty_row
        } else {
            &parsed[i + 1]
        };

        total += calculate_sum(last, r, next);
    }

    total
}

/// Calculates the product of all gears in the schematic that have two adjacent numbers.
///
/// #Argument
///
/// 'input' - The input schematic.
pub fn part_two(input: &str) -> u32 {
    let parsed: Vec<_> = input.lines().map(parse_row).collect();

    let empty_row = Row::new(vec![], vec![]);

    let mut total = 0;

    for (i, r) in parsed.iter().enumerate() {
        let last = if i == 0 { &empty_row } else { &parsed[i - 1] };
        let next = if i == parsed.len() - 1 {
            &empty_row
        } else {
            &parsed[i + 1]
        };

        total += calculate_product(last, r, next);
    }

    total
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
                Symbol::new('$', 9)
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
            parse_row("...*......"),
            Row::new(vec![], vec![Symbol::new('*', 3)])
        );
        assert_eq!(
            parse_row("..35..633."),
            Row::new(vec![Number::new(35, 2, 4), Number::new(633, 6, 9)], vec![])
        );
        assert_eq!(
            parse_row("...$.*...."),
            Row::new(vec![], vec![Symbol::new('$', 3), Symbol::new('*', 5)])
        );
    }

    #[test]
    fn calculate_sum_for_three_adjacent_rows() {
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![], vec![]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![Number::new(35, 2, 4)], vec![Symbol::new('#', 3)]),
                &Row::new(vec![], vec![]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 3)]),
                &Row::new(vec![], vec![])
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 0)]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 1)]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![])
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 2)]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![])
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 3)]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![])
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 4)]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![])
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 5)]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 6)]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 7)]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 8)]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 9)]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![])
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![Symbol::new('#', 0)]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![Symbol::new('#', 1)]),
                &Row::new(vec![], vec![])
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![Symbol::new('#', 4)]),
                &Row::new(vec![], vec![])
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![Symbol::new('#', 5)]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![Symbol::new('#', 6)]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![Symbol::new('#', 7)]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![Symbol::new('#', 8)]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![Symbol::new('#', 9)]),
                &Row::new(vec![], vec![])
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 0)])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 1)])
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 2)])
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 3)])
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 4)])
            ),
            35
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 5)])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 6)])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 7)])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 8)])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(35, 2, 4)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 9)])
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 0)]),
                &Row::new(vec![Number::new(617, 0, 3)], vec![]),
                &Row::new(vec![], vec![])
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 1)]),
                &Row::new(vec![Number::new(617, 0, 3)], vec![]),
                &Row::new(vec![], vec![])
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 2)]),
                &Row::new(vec![Number::new(617, 0, 3)], vec![]),
                &Row::new(vec![], vec![])
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 3)]),
                &Row::new(vec![Number::new(617, 0, 3)], vec![]),
                &Row::new(vec![], vec![])
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 4)]),
                &Row::new(vec![Number::new(617, 0, 3)], vec![]),
                &Row::new(vec![], vec![])
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(617, 0, 3)], vec![Symbol::new('#', 3)]),
                &Row::new(vec![], vec![])
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(617, 0, 3)], vec![]),
                &Row::new(vec![], vec![])
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(617, 0, 3)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 0)])
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(617, 0, 3)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 1)])
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(617, 0, 3)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 2)])
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(617, 0, 3)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 3)])
            ),
            617
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(617, 0, 3)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 4)])
            ),
            0
        );

        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 5)]),
                &Row::new(vec![Number::new(123, 7, 10)], vec![]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 6)]),
                &Row::new(vec![Number::new(123, 7, 10)], vec![]),
                &Row::new(vec![], vec![])
            ),
            123
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 7)]),
                &Row::new(vec![Number::new(123, 7, 10)], vec![]),
                &Row::new(vec![], vec![])
            ),
            123
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 8)]),
                &Row::new(vec![Number::new(123, 7, 10)], vec![]),
                &Row::new(vec![], vec![])
            ),
            123
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![Symbol::new('#', 9)]),
                &Row::new(vec![Number::new(123, 7, 10)], vec![]),
                &Row::new(vec![], vec![])
            ),
            123
        );

        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(123, 7, 10)], vec![Symbol::new('#', 5)]),
                &Row::new(vec![], vec![])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(123, 7, 10)], vec![Symbol::new('#', 6)]),
                &Row::new(vec![], vec![])
            ),
            123
        );

        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(123, 7, 10)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 5)])
            ),
            0
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(123, 7, 10)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 6)])
            ),
            123
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(123, 7, 10)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 7)])
            ),
            123
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(123, 7, 10)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 8)])
            ),
            123
        );
        assert_eq!(
            calculate_sum(
                &Row::new(vec![], vec![]),
                &Row::new(vec![Number::new(123, 7, 10)], vec![]),
                &Row::new(vec![], vec![Symbol::new('#', 9)])
            ),
            123
        );
    }

    #[test]
    fn calculate_product_for_three_adjacent_rows() {
        assert_eq!(
            calculate_product(
                &parse_row("..35..633."),
                &parse_row("......#..."),
                &parse_row("617*......")
            ),
            0
        );
        assert_eq!(
            calculate_product(
                &parse_row("467..114.."),
                &parse_row("...*......"),
                &parse_row("..35..633.")
            ),
            16345
        );
        assert_eq!(
            calculate_product(
                &parse_row("......755."),
                &parse_row("...$.*...."),
                &parse_row(".664.598..")
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
