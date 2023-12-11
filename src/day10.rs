use std::collections::{HashMap, VecDeque};

#[derive(Eq, PartialEq)]
enum State {
    Outside,
    OnTopEdge,
    OnBottomEdge,
    Inside,
}

#[derive(Debug, Eq, PartialEq)]
struct Pipe {
    up: Option<(usize, usize)>,
    down: Option<(usize, usize)>,
    left: Option<(usize, usize)>,
    right: Option<(usize, usize)>,
    distance: Option<usize>,
}

fn read_map(s: &str) -> ((usize, usize), HashMap<(usize, usize), Pipe>) {
    let mut map = HashMap::new();

    let mut start = None;

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '|' => {
                    if y > 0 {
                        map.insert(
                            (x, y),
                            Pipe {
                                up: Some((x, y - 1)),
                                down: Some((x, y + 1)),
                                left: None,
                                right: None,
                                distance: None,
                            },
                        );
                    }
                }
                '-' => {
                    if x > 0 {
                        map.insert(
                            (x, y),
                            Pipe {
                                up: None,
                                down: None,
                                left: Some((x - 1, y)),
                                right: Some((x + 1, y)),
                                distance: None,
                            },
                        );
                    }
                }
                'L' => {
                    if y > 0 {
                        map.insert(
                            (x, y),
                            Pipe {
                                up: Some((x, y - 1)),
                                down: None,
                                left: None,
                                right: Some((x + 1, y)),
                                distance: None,
                            },
                        );
                    }
                }
                'J' => {
                    if x > 0 && y > 0 {
                        map.insert(
                            (x, y),
                            Pipe {
                                up: Some((x, y - 1)),
                                down: None,
                                left: Some((x - 1, y)),
                                right: None,
                                distance: None,
                            },
                        );
                    }
                }
                '7' => {
                    if x > 0 {
                        map.insert(
                            (x, y),
                            Pipe {
                                up: None,
                                down: Some((x, y + 1)),
                                left: Some((x - 1, y)),
                                right: None,
                                distance: None,
                            },
                        );
                    }
                }
                'F' => {
                    map.insert(
                        (x, y),
                        Pipe {
                            up: None,
                            down: Some((x, y + 1)),
                            left: None,
                            right: Some((x + 1, y)),
                            distance: None,
                        },
                    );
                }
                'S' => {
                    start = Some((x, y));
                }
                _ => {}
            }
        }
    }

    // Now, figure out the shape of the starting cell
    let (sx, sy) = start.unwrap();

    let up = if sy > 0 {
        map.get(&(sx, sy - 1))
            .filter(|p| p.down.is_some())
            .map(|_| (sx, sy - 1))
    } else {
        None
    };
    let down = map
        .get(&(sx, sy + 1))
        .filter(|p| p.up.is_some())
        .map(|_| (sx, sy + 1));
    let left = if sx > 0 {
        map.get(&(sx - 1, sy))
            .filter(|p| p.right.is_some())
            .map(|_| (sx - 1, sy))
    } else {
        None
    };
    let right = map
        .get(&(sx + 1, sy))
        .filter(|p| p.left.is_some())
        .map(|_| (sx + 1, sy));

    map.insert(
        (sx, sy),
        Pipe {
            up,
            down,
            left,
            right,
            distance: None,
        },
    );

    ((sx, sy), map)
}

fn set_node_distances(start: (usize, usize), map: &mut HashMap<(usize, usize), Pipe>) {
    let mut nodes: VecDeque<((usize, usize), usize)> = VecDeque::new();
    nodes.push_back((start, 0));

    while let Some(node) = nodes.pop_front() {
        let cur_pos = node.0;
        let cur_dist = node.1;

        let pipe = map.get_mut(&cur_pos).unwrap();
        if pipe.distance.is_none() {
            pipe.distance = Some(cur_dist);

            let up = pipe.up;
            let down = pipe.down;
            let left = pipe.left;
            let right = pipe.right;

            if let Some(key) = up {
                nodes.push_back((key, cur_dist + 1));
            }
            if let Some(key) = down {
                nodes.push_back((key, cur_dist + 1));
            }
            if let Some(key) = left {
                nodes.push_back((key, cur_dist + 1));
            }
            if let Some(key) = right {
                nodes.push_back((key, cur_dist + 1));
            }
        }
    }
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> usize {
    let mut map = read_map(input);

    set_node_distances(map.0, &mut map.1);

    map.1
        .iter()
        .filter_map(|(_, p)| p.distance)
        .max()
        .unwrap_or(0)
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> usize {
    let mut map = read_map(input);
    set_node_distances(map.0, &mut map.1);

    // Find our maximum x and y coordinates...
    let width = map.1.keys().map(|pos| pos.0).max().unwrap_or(0) + 1;
    let height = map.1.keys().map(|pos| pos.1).max().unwrap_or(0) + 1;

    let mut count = 0;

    for y in 0..height {
        let mut state = State::Outside;

        for x in 0..width {
            if let Some(pipe) = map.1.get(&(x, y)) {
                if pipe.distance.is_some() {
                    if pipe.left.is_none() && pipe.right.is_none() {
                        // |
                        state = if state == State::Outside {
                            State::Inside
                        } else {
                            State::Outside
                        };
                    } else if pipe.left.is_none() && pipe.up.is_none() {
                        // F
                        state = if state == State::Outside {
                            State::OnTopEdge
                        } else {
                            State::OnBottomEdge
                        };
                    } else if pipe.left.is_none() && pipe.down.is_none() {
                        // L
                        state = if state == State::Outside {
                            State::OnBottomEdge
                        } else {
                            State::OnTopEdge
                        };
                    } else if pipe.right.is_none() && pipe.up.is_none() {
                        // 7
                        state = if state == State::OnTopEdge {
                            State::Outside
                        } else {
                            State::Inside
                        };
                    } else if pipe.right.is_none() && pipe.down.is_none() {
                        // J
                        state = if state == State::OnTopEdge {
                            State::Inside
                        } else {
                            State::Outside
                        };
                    }
                } else if state == State::Inside {
                    count += 1;
                }
            } else if state == State::Inside {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::day10::*;

    #[test]
    fn parse_map_input() {
        let map = read_map(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );

        assert_eq!(map.0, (1, 1));
        assert!(map.1.get(&(0, 0)).is_none());
        assert_eq!(
            *map.1.get(&(2, 1)).unwrap(),
            Pipe {
                up: None,
                down: None,
                left: Some((1, 1)),
                right: Some((3, 1)),
                distance: None,
            }
        );
        assert_eq!(
            *map.1.get(&(1, 2)).unwrap(),
            Pipe {
                up: Some((1, 1)),
                down: Some((1, 3)),
                left: None,
                right: None,
                distance: None,
            }
        );
        assert_eq!(
            *map.1.get(&(1, 3)).unwrap(),
            Pipe {
                up: Some((1, 2)),
                down: None,
                left: None,
                right: Some((2, 3)),
                distance: None,
            }
        );
        assert_eq!(
            *map.1.get(&(3, 3)).unwrap(),
            Pipe {
                up: Some((3, 2)),
                down: None,
                left: Some((2, 3)),
                right: None,
                distance: None,
            }
        );
        assert_eq!(
            *map.1.get(&(3, 1)).unwrap(),
            Pipe {
                up: None,
                down: Some((3, 2)),
                left: Some((2, 1)),
                right: None,
                distance: None,
            }
        );

        assert_eq!(
            *map.1.get(&(1, 1)).unwrap(),
            Pipe {
                up: None,
                down: Some((1, 2)),
                left: None,
                right: Some((2, 1)),
                distance: None,
            }
        );
    }

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                ".....
.S-7.
.|.|.
.L-J.
....."
            ),
            4
        );

        assert_eq!(
            part_one(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            ),
            8
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
.........."
            ),
            4
        );

        assert_eq!(
            part_two(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            8
        );

        assert_eq!(
            part_two(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            10
        )
    }
}
