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

fn iterate(map: &mut Map, start: HashSet<(usize, usize)>, steps: usize) -> HashSet<(usize, usize)> {
    let mut start_set = start;

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

    start_set
}

fn solve(input: &str, steps: usize) -> usize {
    let mut map = Map::from(input);

    let mut start_set = HashSet::new();
    start_set.insert(map.start);

    start_set = iterate(&mut map, start_set, steps);

    start_set.len()
}

fn solve_infinite(input: &str, steps: usize) -> usize {
    let mut map = Map::from(input);

    let size = map.width.max(map.height);
    let center = (size - 1) / 2;

    // println!("Map size: {size}");
    // println!("Center dist: {center}");

    // Test our observations that the map size is always odd...
    assert_eq!(size % 2, 1);
    // ...and the total steps will always land us on the far edge of a map tile from the center.
    assert_eq!((steps - center) % size, 0);

    // Find the counts for our even and odd parity full squares...
    let mut set = HashSet::new();
    set.insert(map.start);
    set = iterate(&mut map, set, size);
    assert!(!set.contains(&(center, center)));
    let full_odd = set.len();
    set = iterate(&mut map, set, 1);
    assert!(set.contains(&(center, center)));
    let full_even = set.len();

    // println!("Even: {full_even}");
    // println!("Odd: {full_odd}");

    // Find the counts for the final tiles is we walk the total number of steps in each of the four
    // cardinal directions
    set.clear();
    set.insert((0, center));
    set = iterate(&mut map, set, size - 1);
    assert!(set.contains(&(size - 1, center)));
    assert!(set.contains(&(center, 0)));
    assert!(set.contains(&(center, size - 1)));
    let east = set.len();

    set.clear();
    set.insert((size - 1, center));
    set = iterate(&mut map, set, size - 1);
    assert!(set.contains(&(0, center)));
    assert!(set.contains(&(center, 0)));
    assert!(set.contains(&(center, size - 1)));
    let west = set.len();

    set.clear();
    set.insert((center, size - 1));
    set = iterate(&mut map, set, size - 1);
    assert!(set.contains(&(center, 0)));
    assert!(set.contains(&(0, center)));
    assert!(set.contains(&(size - 1, center)));
    let north = set.len();

    set.clear();
    set.insert((center, 0));
    set = iterate(&mut map, set, size - 1);
    assert!(set.contains(&(center, size - 1)));
    assert!(set.contains(&(0, center)));
    assert!(set.contains(&(size - 1, center)));
    let south = set.len();

    // println!("North: {north}");
    // println!("South: {south}");
    // println!("East: {east}");
    // println!("West: {west}");

    // Find our counts for the northwest corners
    set.clear();
    set.insert((size - 1, size - 1));
    set = iterate(&mut map, set, center - 1);
    assert!(set.contains(&(center + 1, size - 1)));
    assert!(set.contains(&(size - 1, center + 1)));
    let nw_small = set.len();
    set = iterate(&mut map, set, size);
    assert!(set.contains(&(0, center)));
    assert!(set.contains(&(center, 0)));
    let nw_big = set.len();

    // println!("Northwest: {nw_small}, {nw_big}");

    // Find our counts for the northeast corners
    set.clear();
    set.insert((0, size - 1));
    set = iterate(&mut map, set, center - 1);
    let ne_small = set.len();
    set = iterate(&mut map, set, size);
    let ne_big = set.len();

    // println!("Northeast: {ne_small}, {ne_big}");

    // Find our counts for the southwest corners
    set.clear();
    set.insert((size - 1, 0));
    set = iterate(&mut map, set, center - 1);
    let sw_small = set.len();
    set = iterate(&mut map, set, size);
    let sw_big = set.len();

    // println!("Southwest: {sw_small}, {sw_big}");

    // Find our counts for the southeast corners
    set.clear();
    set.insert((0, 0));
    set = iterate(&mut map, set, center - 1);
    let se_small = set.len();
    set = iterate(&mut map, set, size);
    let se_big = set.len();

    // println!("Southeast: {se_small}, {se_big}");

    let factor = (steps - center) / size;
    assert_eq!(factor % 2, 0);
    let full_count = 2 * factor * factor - 2 * factor + 1;
    let odd_count = factor * factor - 2 * factor + 1;
    let even_count = full_count - odd_count;

    // println!("Factor: {factor}");
    // println!("Even Parity Count: {even_count}");
    // println!("Odd Parity Count: {odd_count}");

    odd_count * full_odd
        + even_count * full_even
        + north
        + south
        + east
        + west
        + factor * (nw_small + ne_small + se_small + sw_small)
        + (factor - 1) * (nw_big + ne_big + se_big + sw_big)
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
    solve_infinite(input, 26_501_365)
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
                6,
            ),
            16
        );
    }

    #[test]
    fn part_two_correct() {
        // The provided example won't let us make the required optimizations the actual input allows
        // so we'll come up with our own simplified test case...

        let input = ".......
.......
.......
...S...
.......
.......
.......";

        assert_eq!(solve_infinite(input, 17), 324);
    }
}
