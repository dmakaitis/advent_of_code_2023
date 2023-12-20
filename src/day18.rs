use crate::day18::State::{Bottom, Inside, Outside, Top};
use itertools::Itertools;

struct Command {
    vector: (i64, i64),
    vector_large: (i64, i64),
}

fn decode_large_vector(value: &str) -> (i64, i64) {
    let code = value.strip_prefix("(#").unwrap();

    let size_code = &code[0..5];
    let dir = &code[5..6];

    let size = i64::from_str_radix(size_code, 16).unwrap();

    match dir {
        "0" => Some((size, 0)),
        "1" => Some((0, size)),
        "2" => Some((-size, 0)),
        "3" => Some((0, -size)),
        _ => None,
    }
    .unwrap()
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let (dir, dist, vector_large) = value.split_whitespace().collect_tuple().unwrap();
        let dist = dist.parse::<i64>().unwrap();

        let vector = match dir {
            "R" => Some((dist, 0)),
            "L" => Some((-dist, 0)),
            "U" => Some((0, -dist)),
            "D" => Some((0, dist)),
            _ => None,
        }
        .unwrap();

        let vector_large = decode_large_vector(vector_large);

        Command {
            vector,
            vector_large,
        }
    }
}

struct Edge {
    a: (i64, i64),
    b: (i64, i64),
}

struct Map {
    x: i64,
    y: i64,
    edges: Vec<Edge>,
}

impl Map {
    fn new() -> Map {
        Map {
            x: 0,
            y: 0,
            edges: vec![],
        }
    }

    fn execute(&mut self, cmd: &Command) {
        let new_x = self.x + cmd.vector.0;
        let new_y = self.y + cmd.vector.1;

        self.edges.push(Edge {
            a: (self.x, self.y),
            b: (new_x, new_y),
        });

        self.x = new_x;
        self.y = new_y;
    }

    fn find_bounds(&self) -> ((i64, i64), (i64, i64)) {
        let min_x = self
            .edges
            .iter()
            .map(|edge| edge.a.0.min(edge.b.0))
            .min()
            .unwrap();
        let max_x = self
            .edges
            .iter()
            .map(|edge| edge.a.0.max(edge.b.0))
            .max()
            .unwrap();

        let min_y = self
            .edges
            .iter()
            .map(|edge| edge.a.1.min(edge.b.1))
            .min()
            .unwrap();
        let max_y = self
            .edges
            .iter()
            .map(|edge| edge.a.1.max(edge.b.1))
            .max()
            .unwrap();

        ((min_x, min_y), (max_x, max_y))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Outside,
    Top,
    Bottom,
    Inside,
}

fn calculate_volume(commands: Vec<Command>) -> i64 {
    let mut map = Map::new();

    for cmd in commands {
        map.execute(&cmd);
    }

    let ((_, min_y), (_, max_y)) = map.find_bounds();

    // TODO: This can be optimized by scanning spans where edge boundaries occur to avoid duplicating work for adjacent y values:
    let mut count = 0;
    for y in min_y..=max_y {
        let edges = map
            .edges
            .iter()
            .filter(|edge| {
                let min_y = edge.a.1.min(edge.b.1);
                let max_y = edge.a.1.max(edge.b.1);
                y >= min_y && y <= max_y
            })
            .filter(|edge| edge.a.0 == edge.b.0)
            .sorted_by_key(|edge| {
                let min_x = edge.a.0.min(edge.b.0);
                min_x
            })
            .collect_vec();

        let mut enter_x = i64::MIN;
        let mut state = Outside;

        for edge in edges {
            if edge.a.1 != y && edge.b.1 != y {
                match state {
                    Outside => {
                        enter_x = edge.a.0;
                        state = Inside;
                    }
                    Inside => {
                        count += edge.a.0 + 1 - enter_x;
                        state = Outside;
                    }
                    _ => {}
                }
            } else if edge.a.1 < y || edge.b.1 < y {
                match state {
                    Outside => {
                        enter_x = edge.a.0;
                        state = Bottom;
                    }
                    Bottom => {
                        count += edge.a.0 + 1 - enter_x;
                        state = Outside;
                    }
                    Top => {
                        state = Inside;
                    }
                    Inside => {
                        state = Top;
                    }
                }
            } else {
                match state {
                    Outside => {
                        enter_x = edge.a.0;
                        state = Top;
                    }
                    Top => {
                        count += edge.a.0 + 1 - enter_x;
                        state = Outside;
                    }
                    Bottom => {
                        state = Inside;
                    }
                    Inside => {
                        state = Bottom;
                    }
                }
            }
        }
    }

    count
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i64 {
    let commands = input.lines().map(|line| Command::from(line)).collect_vec();

    calculate_volume(commands)
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i64 {
    let commands = input
        .lines()
        .map(|line| Command::from(line))
        .map(|cmd| Command {
            vector: cmd.vector_large,
            vector_large: cmd.vector,
        })
        .collect_vec();

    calculate_volume(commands)
}

#[cfg(test)]
mod tests {
    use crate::day18::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
            ),
            62
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
            ),
            952408144115
        );
    }
}
