use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Path {
    start: (usize, usize),
    end: (usize, usize),
    cost: usize,
}

fn get_move_options(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut moves = vec![];

    if x > 0 {
        match map[y][x - 1] {
            '#' | '>' => {}
            _ => {
                moves.push((x - 1, y));
            }
        }
    }

    if y > 0 && y < map.len() - 1 {
        match map[y - 1][x] {
            '#' | 'v' => {}
            _ => {
                moves.push((x, y - 1));
            }
        }
    }

    if (x + 1) < map[y].len() {
        match map[y][x + 1] {
            '#' | '<' => {}
            _ => {
                moves.push((x + 1, y));
            }
        }
    }

    if (y + 1) < map.len() {
        match map[y + 1][x] {
            '#' | '^' => {}
            _ => {
                moves.push((x, y + 1));
            }
        }
    }

    moves
}

fn find_next_node(
    map: &Vec<Vec<char>>,
    (x, y): (usize, usize),
    (lx, ly): (usize, usize),
    cost: usize,
) -> ((usize, usize), usize) {
    let mut options = get_move_options(map, (x, y));
    options.retain(|(a, b)| (*a, *b) != (lx, ly));

    if options.len() != 1 {
        ((x, y), cost)
    } else {
        let opt = options[0];
        find_next_node(map, opt, (x, y), cost + 1)
    }
}

fn find_paths(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<Path> {
    let mut paths = vec![];
    let moves = get_move_options(map, (x, y));

    for m in moves {
        let ((nx, ny), c) = find_next_node(map, m, (x, y), 1);
        paths.push(Path {
            start: (x, y),
            end: (nx, ny),
            cost: c,
        });
    }

    paths
}

fn build_graph(input: &str) -> Vec<Path> {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut searched = HashSet::new();
    let mut nodes = VecDeque::new();
    let mut paths = vec![];
    nodes.push_back((1, 0));

    let mut count = 7;

    while let Some(node) = nodes.pop_front() {
        searched.insert(node);
        let new_paths = find_paths(&map, node);

        for path in new_paths {
            if !searched.contains(&path.end) && !nodes.contains(&path.end) {
                nodes.push_back(path.end);
            }
            paths.push(path);
        }

        count -= 1;
        if count == 0 {
            // break;
        }
    }

    paths
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i32 {
    let paths = build_graph(input);

    // The start node is the node with the lowest y value
    // and the end node is the node with the highest y value

    let start = paths
        .iter()
        .map(|p| p.start)
        .sorted_by_key(|(_, y)| *y)
        .next()
        .unwrap();

    let end = paths
        .iter()
        .map(|p| p.end)
        .sorted_by_key(|(_, y)| usize::MAX - y)
        .next()
        .unwrap();

    let mut nodes = VecDeque::new();
    nodes.push_back(start);

    let mut costs = HashMap::new();
    for p in &paths {
        costs.insert(p.start, i32::MAX);
        costs.insert(p.end, i32::MAX);
    }
    costs.insert(start, 0);

    while let Some(n) = nodes.pop_front() {
        if n != end {
            let start_cost = *costs.get(&n).unwrap();

            let options = paths.iter().filter(|p| p.start == n).collect_vec();

            for p in options {
                let path_total_cost = start_cost - (p.cost as i32);
                if path_total_cost < *costs.get(&p.end).unwrap() {
                    costs.insert(p.end, path_total_cost);
                    nodes.push_back(p.end);
                }
            }
        }
    }

    costs.get(&end).unwrap().abs()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> usize {
    let input = input.replace(['>', '<', '^', 'v'], ".");

    let paths = build_graph(input.as_str());

    let start = paths
        .iter()
        .map(|p| p.start)
        .sorted_by_key(|(_, y)| *y)
        .next()
        .unwrap();

    let end = paths
        .iter()
        .map(|p| p.end)
        .sorted_by_key(|(_, y)| usize::MAX - y)
        .next()
        .unwrap();

    let mut path_heads = VecDeque::new();
    path_heads.push_back((vec![start], 0));

    let mut max_cost = 0;

    while let Some(ph) = path_heads.pop_front() {
        let tail = ph.0.last().unwrap();

        if *tail == end {
            max_cost = max_cost.max(ph.1);
        } else {
            paths
                .iter()
                .filter(|p| p.start == *tail)
                .filter(|p| !ph.0.contains(&p.end))
                .for_each(|p| {
                    let mut new_path_head = ph.0.clone();
                    new_path_head.push(p.end);
                    path_heads.push_back((new_path_head, ph.1 + p.cost));
                });
        }
    }

    max_cost
}

#[cfg(test)]
mod tests {
    use crate::day23::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
            ),
            94
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
            ),
            154
        );
    }
}
