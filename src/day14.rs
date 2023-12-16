use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    rocks: HashSet<(i32, i32)>,
    obstacles: HashSet<(i32, i32)>,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.obstacles.contains(&(x as i32, y as i32)) {
                    result.push('#');
                } else if self.rocks.contains(&(x as i32, y as i32)) {
                    result.push('O');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }

        write!(f, "{result}")
    }
}

fn parse_map(s: &str) -> Map {
    let width = s.lines().nth(0).unwrap().len();
    let mut height = 0usize;

    let mut rocks = HashSet::new();
    let mut obstacles = HashSet::new();

    for (y, line) in s.lines().enumerate() {
        height += 1;
        for (x, c) in line.chars().enumerate() {
            match c {
                'O' => {
                    rocks.insert((x as i32, y as i32));
                }
                '#' => {
                    obstacles.insert((x as i32, y as i32));
                }
                _ => {}
            }
        }
    }

    Map {
        width,
        height,
        rocks,
        obstacles,
    }
}

fn tilt_north(map: Map) -> Map {
    let mut rocks = HashSet::with_capacity(map.rocks.len());

    for x in 0..map.width {
        let all = &map
            .rocks
            .iter()
            .filter(|(rx, _)| *rx == x as i32)
            .map(|(_, y)| (false, *y))
            .chain(
                map.obstacles
                    .iter()
                    .filter(|(ox, _)| *ox == x as i32)
                    .map(|(_, y)| (true, *y)),
            )
            .sorted_by(|(_, a), (_, b)| a.cmp(b))
            .collect_vec();

        let mut move_to = 0;

        for item in all {
            if item.0 {
                // Item is an obstacle...
                move_to = item.1 + 1
            } else {
                // Item is a rock...
                rocks.insert((x as i32, move_to));
                move_to += 1;
            }
        }
    }

    Map {
        width: map.width,
        height: map.height,
        obstacles: map.obstacles,
        rocks,
    }
}

fn tilt_west(map: Map) -> Map {
    let mut rocks = HashSet::with_capacity(map.rocks.len());

    for y in 0..map.height {
        let all = &map
            .rocks
            .iter()
            .filter(|(_, ry)| *ry == y as i32)
            .map(|(x, _)| (false, *x))
            .chain(
                map.obstacles
                    .iter()
                    .filter(|(_, oy)| *oy == y as i32)
                    .map(|(x, _)| (true, *x)),
            )
            .sorted_by(|(_, a), (_, b)| a.cmp(b))
            .collect_vec();

        let mut move_to = 0;

        for item in all {
            if item.0 {
                // Item is an obstacle...
                move_to = item.1 + 1
            } else {
                // Item is a rock...
                rocks.insert((move_to, y as i32));
                move_to += 1;
            }
        }
    }

    Map {
        width: map.width,
        height: map.height,
        obstacles: map.obstacles,
        rocks,
    }
}

fn tilt_south(map: Map) -> Map {
    let mut rocks = HashSet::with_capacity(map.rocks.len());

    for x in 0..map.width {
        let all = &map
            .rocks
            .iter()
            .filter(|(rx, _)| *rx == x as i32)
            .map(|(_, y)| (false, *y))
            .chain(
                map.obstacles
                    .iter()
                    .filter(|(ox, _)| *ox == x as i32)
                    .map(|(_, y)| (true, *y)),
            )
            .sorted_by(|(_, a), (_, b)| a.cmp(b))
            .rev()
            .collect_vec();

        let mut move_to = (map.height as i32) - 1;

        for item in all {
            if item.0 {
                // Item is an obstacle...
                move_to = item.1 - 1
            } else {
                // Item is a rock...
                rocks.insert((x as i32, move_to));
                move_to -= 1;
            }
        }
    }

    Map {
        width: map.width,
        height: map.height,
        obstacles: map.obstacles,
        rocks,
    }
}

fn tilt_east(map: Map) -> Map {
    let mut rocks = HashSet::with_capacity(map.rocks.len());

    for y in 0..map.height {
        let all = &map
            .rocks
            .iter()
            .filter(|(_, ry)| *ry == y as i32)
            .map(|(x, _)| (false, *x))
            .chain(
                map.obstacles
                    .iter()
                    .filter(|(_, oy)| *oy == y as i32)
                    .map(|(x, _)| (true, *x)),
            )
            .sorted_by(|(_, a), (_, b)| a.cmp(b))
            .rev()
            .collect_vec();

        let mut move_to = (map.width as i32) - 1;

        for item in all {
            if item.0 {
                // Item is an obstacle...
                move_to = item.1 - 1
            } else {
                // Item is a rock...
                rocks.insert((move_to, y as i32));
                move_to -= 1;
            }
        }
    }

    Map {
        width: map.width,
        height: map.height,
        obstacles: map.obstacles,
        rocks,
    }
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i32 {
    let mut map = parse_map(input);
    map = tilt_north(map);

    map.rocks.iter().map(|(_, y)| map.height as i32 - *y).sum()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i32 {
    let mut map = parse_map(input);

    for i in 0..1000000000 {
        map = tilt_north(map);
        map = tilt_west(map);
        map = tilt_south(map);
        map = tilt_east(map);
    }

    map.rocks.iter().map(|(_, y)| map.height as i32 - *y).sum()
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

    // #[test]
    //     fn part_two_correct() {
    //         assert_eq!(part_two("O....#....
    // O.OO#....#
    // .....##...
    // OO.#O....O
    // .O.....O#.
    // O.#..O.#.#
    // ..O..#O..O
    // .......O..
    // #....###..
    // #OO..#...."), 64);
    //     }
}
