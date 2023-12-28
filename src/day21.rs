use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: (usize, usize),
    move_cache: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let map = value
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let width = map[0].len();
        let height = map.len();

        let mut start = (0, 0);

        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'S' {
                    start = (x, y);
                }
            }
        }

        Map {
            map,
            width,
            height,
            start,
            move_cache: HashMap::new(),
        }
    }
}

impl Map {
    fn get(&self, (x, y): (usize, usize)) -> char {
        let x = x % self.width;
        let y = y % self.height;

        self.map[y][x]
    }

    fn get_moves(&mut self, start: (usize, usize)) -> Vec<(usize, usize)> {
        if let Some(result) = self.move_cache.get(&start) {
            return result.clone();
        }

        let mut result = vec![];

        if start.0 > 0 {
            result.push((start.0 - 1, start.1));
        }
        if start.1 > 0 {
            result.push((start.0, start.1 - 1));
        }
        if start.0 + 1 < self.width {
            result.push((start.0 + 1, start.1));
        }
        if start.1 + 1 < self.height {
            result.push((start.0, start.1 + 1));
        }

        result.retain(|p| self.get(*p) != '#');

        self.move_cache.insert(start, result.clone());

        result
    }
}

fn solve(input: &str, steps: usize) -> usize {
    let mut map = Map::from(input);

    let mut start_set = HashSet::new();
    start_set.insert(map.start);

    for _ in 0..steps {
        let mut next_set = HashSet::new();

        for s in start_set {
            let next = map.get_moves(s);
            for n in next {
                next_set.insert(n);
            }
        }

        start_set = next_set;
    }

    start_set.len()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> usize {
    solve(input, 64)
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> usize {
    // solve(input, 26501365)
    0
}

#[cfg(test)]
mod tests {
    use crate::day21::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            solve(
                "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
                6
            ),
            16
        );
    }

    #[test]
    fn part_two_correct() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        assert_eq!(solve(input, 6), 16);
        // assert_eq!(solve(input, 10), 50);
        // assert_eq!(solve(input, 50, true), 1594);
        // assert_eq!(solve(input, 100, true), 6536);
        // assert_eq!(solve(input, 500, true), 167004);
        // assert_eq!(solve(input, 1000, true), 668697);
        // assert_eq!(solve(input, 5000, true), 16733044);
    }
}
