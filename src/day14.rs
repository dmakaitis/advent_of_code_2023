use std::collections::hash_map::DefaultHasher;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct Rock {
    x: usize,
    y: usize,
    obstacle: bool,
}

struct Map {
    width: usize,
    height: usize,
    rocks: Vec<Rock>,
    buffer: Vec<usize>,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(rock) = self.rocks.iter().filter(|r| r.x == x && r.y == y).next() {
                    if rock.obstacle {
                        result.push('#');
                    } else {
                        result.push('O');
                    }
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }

        write!(f, "{result}")
    }
}

impl Map {
    fn calculate_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        for rock in &self.rocks {
            rock.hash(&mut hasher);
        }
        hasher.finish()
    }

    fn tilt_north(&mut self) {
        self.rocks.sort_unstable_by_key(|r| (r.x, r.y));
        self.buffer.iter_mut().for_each(|v| *v = 0);

        self.rocks.iter_mut().for_each(|r| {
            let next_y = &mut self.buffer[r.x];

            if r.obstacle {
                *next_y = r.y + 1;
            } else {
                r.y = *next_y;
                *next_y += 1;
            }
        });
    }

    fn tilt_south(&mut self) {
        self.rocks
            .sort_unstable_by_key(|r| (self.width - r.x, self.height - r.y));
        self.buffer.iter_mut().for_each(|v| *v = self.height - 1);

        self.rocks.iter_mut().for_each(|r| {
            let next_y = &mut self.buffer[r.x];

            if r.obstacle {
                if r.y > 0 {
                    *next_y = r.y - 1;
                }
            } else {
                r.y = *next_y;
                if *next_y > 0 {
                    *next_y -= 1;
                }
            }
        });
    }

    fn tilt_west(&mut self) {
        // self.rocks.sort_unstable_by_key(|r| (r.x, r.y));
        self.buffer.iter_mut().for_each(|v| *v = 0);

        self.rocks.iter_mut().for_each(|r| {
            let next_x = &mut self.buffer[r.y];

            if r.obstacle {
                *next_x = r.x + 1;
            } else {
                r.x = *next_x;
                *next_x += 1;
            }
        });
    }

    fn tilt_east(&mut self) {
        // self.rocks.sort_unstable_by_key(|r| (self.width - r.x, self.height - r.y));
        self.buffer.iter_mut().for_each(|v| *v = self.width - 1);

        self.rocks.iter_mut().for_each(|r| {
            let next_x = &mut self.buffer[r.y];

            if r.obstacle {
                if r.x > 0 {
                    *next_x = r.x - 1;
                }
            } else {
                r.x = *next_x;
                if *next_x > 0 {
                    *next_x -= 1;
                }
            }
        });
    }

    fn calculate_load(&self) -> usize {
        self.rocks
            .iter()
            .filter(|r| !r.obstacle)
            .map(|r| r.y)
            .map(|y| self.height - y)
            .sum()
    }
}

fn parse_map(s: &str) -> Map {
    let width = s.lines().nth(0).unwrap().len();
    let mut height = 0usize;

    let mut rocks = Vec::new();

    for (y, line) in s.lines().enumerate() {
        height += 1;
        for (x, c) in line.chars().enumerate() {
            match c {
                'O' => {
                    rocks.push(Rock {
                        x,
                        y,
                        obstacle: false,
                    });
                }
                '#' => {
                    rocks.push(Rock {
                        x,
                        y,
                        obstacle: true,
                    });
                }
                _ => {}
            }
        }
    }

    let largest = width.max(height);
    let buffer = vec![0; largest];

    Map {
        width,
        height,
        rocks,
        buffer,
    }
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> usize {
    let mut map = parse_map(input);
    // println!("{map}");
    map.tilt_north();
    // println!("{map}");
    map.calculate_load()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> usize {
    let mut map = parse_map(input);
    let mut map_hashes = vec![];

    let mut cycle = (0, 0);

    // Required iterations: 1,000,000,000
    for i in 0..10000 {
        map.tilt_north();
        map.tilt_west();
        map.tilt_south();
        map.tilt_east();

        let hash = map.calculate_hash();
        if let Some(pos) = map_hashes.iter().position(|h| *h == hash) {
            cycle = (pos + 1, i - pos);
            break;
        } else {
            map_hashes.push(hash);
        }
    }

    if cycle == (0, 0) {
        panic!("No cycle found in map progression");
    }

    let modulo = (1_000_000_000 - cycle.0) % cycle.1;
    for _ in 0..modulo {
        map.tilt_north();
        map.tilt_west();
        map.tilt_south();
        map.tilt_east();
    }

    map.calculate_load()
}

#[cfg(test)]
mod tests {
    use crate::day14::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            ),
            136
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            ),
            64
        );
    }
}
