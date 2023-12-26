use crate::day16::Direction::*;
use crate::day16::Entity::*;
use itertools::Itertools;

#[derive(Eq, PartialEq)]
enum Entity {
    Empty,
    UpRightMirror,
    DownRightMirror,
    HSplitter,
    VSplitter,
}

#[derive(Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Cell {
    entity: Entity,
    energized: bool,
    traced_up: bool,
    traced_down: bool,
    traced_left: bool,
    traced_right: bool,
}

impl Cell {
    fn new(entity: Entity) -> Cell {
        Cell {
            entity,
            energized: false,
            traced_up: false,
            traced_down: false,
            traced_left: false,
            traced_right: false,
        }
    }
}

fn parse_map(s: &str) -> Vec<Vec<Cell>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '/' => UpRightMirror,
                    '\\' => DownRightMirror,
                    '-' => HSplitter,
                    '|' => VSplitter,
                    _ => Empty,
                })
                .map(Cell::new)
                .collect_vec()
        })
        .collect_vec()
}

fn trace_beam(map: &mut Vec<Vec<Cell>>, start: (i32, i32), direction: Direction) {
    let mut x = start.0;
    let mut y = start.1;

    let vector = match direction {
        Up => (0, -1),
        Down => (0, 1),
        Left => (-1, 0),
        Right => (1, 0),
    };

    // println!("Starting trace from ({x}, {y}) with vector ({}, {})...", vector.0, vector.1);
    loop {
        if x < 0 || y < 0 {
            return;
        }
        if y as usize >= map.len() {
            return;
        }
        if x as usize >= map[y as usize].len() {
            return;
        }

        let cell = &mut map[y as usize][x as usize];

        match direction {
            Up => {
                if cell.traced_up {
                    return;
                }
                cell.traced_up = true;
            }
            Down => {
                if cell.traced_down {
                    return;
                }
                cell.traced_down = true;
            }
            Left => {
                if cell.traced_left {
                    return;
                }
                cell.traced_left = true;
            }
            Right => {
                if cell.traced_right {
                    return;
                }
                cell.traced_right = true;
            }
        }

        // println!("   Energizing cell ({x}, {y})");
        cell.energized = true;

        match cell.entity {
            Empty => {
                // Let it pass
            }
            VSplitter => {
                if direction == Left || direction == Right {
                    trace_beam(map, (x, y - 1), Up);
                    trace_beam(map, (x, y + 1), Down);
                    return;
                }
            }
            HSplitter => {
                if direction == Up || direction == Down {
                    trace_beam(map, (x - 1, y), Left);
                    trace_beam(map, (x + 1, y), Right);
                    return;
                }
            }
            UpRightMirror => {
                match direction {
                    Up => trace_beam(map, (x + 1, y), Right),
                    Down => trace_beam(map, (x - 1, y), Left),
                    Left => trace_beam(map, (x, y + 1), Down),
                    Right => trace_beam(map, (x, y - 1), Up),
                };
                return;
            }
            DownRightMirror => {
                match direction {
                    Up => trace_beam(map, (x - 1, y), Left),
                    Down => trace_beam(map, (x + 1, y), Right),
                    Left => trace_beam(map, (x, y - 1), Up),
                    Right => trace_beam(map, (x, y + 1), Down),
                };
                return;
            }
        }

        x += vector.0;
        y += vector.1;
    }
}

fn reset(map: &mut Vec<Vec<Cell>>) {
    for row in map {
        for cell in row {
            cell.energized = false;
            cell.traced_up = false;
            cell.traced_down = false;
            cell.traced_left = false;
            cell.traced_right = false;
        }
    }
}

fn calculate_energy(map: &[Vec<Cell>]) -> i32 {
    map.iter()
        .map(|l| {
            l.iter()
                .map(|c| if c.energized { 1 } else { 0 })
                .sum::<i32>()
        })
        .sum()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i32 {
    let mut map = parse_map(input);
    trace_beam(&mut map, (0, 0), Right);

    calculate_energy(&map)
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i32 {
    let mut map = parse_map(input);

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let mut max_energy = 0;

    for x in 0..width {
        trace_beam(&mut map, (x, 0), Down);
        let energy = calculate_energy(&map);
        if energy > max_energy {
            max_energy = energy;
        }
        reset(&mut map);

        trace_beam(&mut map, (x, height - 1), Up);
        let energy = calculate_energy(&map);
        if energy > max_energy {
            max_energy = energy;
        }
        reset(&mut map);
    }

    for y in 0..height {
        trace_beam(&mut map, (0, y), Right);
        let energy = calculate_energy(&map);
        if energy > max_energy {
            max_energy = energy;
        }
        reset(&mut map);

        trace_beam(&mut map, (width - 1, y), Left);
        let energy = calculate_energy(&map);
        if energy > max_energy {
            max_energy = energy;
        }
        reset(&mut map);
    }

    max_energy
}

#[cfg(test)]
mod tests {
    use crate::day16::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."
            ),
            46
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."
            ),
            51
        );
    }
}
