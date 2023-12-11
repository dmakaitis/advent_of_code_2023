fn parse_input(s: &str) -> Vec<(i32, i32)> {
    let mut result = vec![];

    for (y, line) in s.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                result.push((x as i32, y as i32));
            }
        }
    }

    result
}

fn expand_rows(factor: i32, map: &mut [(i32, i32)]) {
    let height = map.iter().map(|(_, y)| *y).max().unwrap_or(0) + 1;

    let mut rows_to_expand = vec![];

    for y in 0..height {
        let row_empty = map.iter().all(|(_, y2)| *y2 != y);

        if row_empty {
            rows_to_expand.push(y);
        }
    }

    while let Some(y) = rows_to_expand.pop() {
        for pos in map.iter_mut() {
            if pos.1 >= y {
                pos.1 += factor - 1;
            }
        }
    }
}

fn expand_columns(factor: i32, map: &mut [(i32, i32)]) {
    let width = map.iter().map(|(x, _)| *x).max().unwrap_or(0) + 1;

    let mut columns_to_expand = vec![];

    for x in 0..width {
        let column_empty = map.iter().all(|(x2, _)| *x2 != x);

        if column_empty {
            columns_to_expand.push(x);
        }
    }

    while let Some(x) = columns_to_expand.pop() {
        for pos in map.iter_mut() {
            if pos.0 >= x {
                pos.0 += factor - 1;
            }
        }
    }
}

fn expand_map(factor: i32, map: &mut [(i32, i32)]) {
    expand_rows(factor, map);
    expand_columns(factor, map);
}

fn sum_distances_with_expansion(input: &str, factor: i32) -> i64 {
    let mut map = parse_input(input);

    expand_map(factor, &mut map);

    let mut total_distance = 0;

    for a in 0..(map.len() - 1) {
        let map_a = map[a];

        for map_b in map.iter().skip(a + 1) {
            let dist_x = (map_b.0 - map_a.0).abs() as i64;
            let dist_y = (map_b.1 - map_a.1).abs() as i64;

            let dist = dist_x + dist_y;

            total_distance += dist;
        }
    }

    total_distance
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i64 {
    sum_distances_with_expansion(input, 2)
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i64 {
    sum_distances_with_expansion(input, 1000000)
}

#[cfg(test)]
mod tests {
    use crate::day11::*;

    #[test]
    fn test_parse_input() {
        let map = parse_input(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );

        assert_eq!(map.len(), 9);
        assert_eq!(map[0], (3, 0));
        assert_eq!(map[4], (1, 5));
        assert_eq!(map[8], (4, 9));
    }

    #[test]
    fn test_expand_map() {
        let mut map = parse_input(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        expand_map(2, &mut map);

        assert_eq!(map.len(), 9);
        assert_eq!(map[0], (4, 0));
        assert_eq!(map[4], (1, 6));
        assert_eq!(map[8], (5, 11));
    }

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            ),
            374
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            sum_distances_with_expansion(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                10
            ),
            1030
        );

        assert_eq!(
            sum_distances_with_expansion(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                100
            ),
            8410
        );
    }
}
