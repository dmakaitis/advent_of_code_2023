use crate::day17::Direction::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    cost: usize,
    last_direction: Direction,
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

struct Map {
    width: usize,
    height: usize,
    heat: Vec<Vec<usize>>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let heat = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| (c as usize) - ('0' as usize))
                    .collect_vec()
            })
            .collect_vec();

        let width = heat[0].len();
        let height = heat.len();

        Map {
            width,
            height,
            heat,
        }
    }
}

impl Map {
    fn create_nodes_in_direction(
        &self,
        x: usize,
        y: usize,
        dir: Direction,
        old_cost: usize,
        min: usize,
        max: usize,
    ) -> Vec<Node> {
        let mut result = vec![];

        match dir {
            Left => {
                let mut new_x = x;
                let mut cost = old_cost;
                for i in 0..max {
                    if new_x == 0 {
                        break;
                    }
                    new_x -= 1;
                    cost += self.heat[y][new_x];
                    if i + 1 >= min {
                        result.push(Node {
                            x: new_x,
                            y,
                            last_direction: dir,
                            cost,
                        });
                    }
                }
            }
            Right => {
                let mut new_x = x;
                let mut cost = old_cost;
                for i in 0..max {
                    new_x += 1;
                    if new_x >= self.width {
                        break;
                    }
                    cost += self.heat[y][new_x];
                    if i + 1 >= min {
                        result.push(Node {
                            x: new_x,
                            y,
                            last_direction: dir,
                            cost,
                        });
                    }
                }
            }
            Up => {
                let mut new_y = y;
                let mut cost = old_cost;
                for i in 0..max {
                    if new_y == 0 {
                        break;
                    }
                    new_y -= 1;
                    cost += self.heat[new_y][x];
                    if i + 1 >= min {
                        result.push(Node {
                            x,
                            y: new_y,
                            last_direction: dir,
                            cost,
                        });
                    }
                }
            }
            Down => {
                let mut new_y = y;
                let mut cost = old_cost;
                for i in 0..max {
                    new_y += 1;
                    if new_y >= self.height {
                        break;
                    }
                    cost += self.heat[new_y][x];
                    if i + 1 >= min {
                        result.push(Node {
                            x,
                            y: new_y,
                            last_direction: dir,
                            cost,
                        });
                    }
                }
            }
        }

        result
    }

    fn get_next_nodes(&self, node: &Node, min: usize, max: usize) -> Vec<Node> {
        let mut result = vec![];

        match node.last_direction {
            Up => {
                result.append(
                    &mut self.create_nodes_in_direction(node.x, node.y, Left, node.cost, min, max),
                );
                result.append(
                    &mut self.create_nodes_in_direction(node.x, node.y, Right, node.cost, min, max),
                );
            }
            Down => {
                result.append(
                    &mut self.create_nodes_in_direction(node.x, node.y, Left, node.cost, min, max),
                );
                result.append(
                    &mut self.create_nodes_in_direction(node.x, node.y, Right, node.cost, min, max),
                );
            }
            Left => {
                result.append(
                    &mut self.create_nodes_in_direction(node.x, node.y, Up, node.cost, min, max),
                );
                result.append(
                    &mut self.create_nodes_in_direction(node.x, node.y, Down, node.cost, min, max),
                );
            }
            Right => {
                result.append(
                    &mut self.create_nodes_in_direction(node.x, node.y, Up, node.cost, min, max),
                );
                result.append(
                    &mut self.create_nodes_in_direction(node.x, node.y, Down, node.cost, min, max),
                );
            }
        };

        result
    }
}

fn solve_with_min_max(input: &str, min: usize, max: usize) -> usize {
    let map = Map::from(input);

    // Keep two cost values for each node, depending on if we reach it from a horizontal direction vs. vertical
    let mut costs = vec![vec![(usize::MAX, usize::MAX); map.width]; map.height];
    costs[0][0] = (0, 0);

    let dest_x = map.width - 1;
    let dest_y = map.height - 1;

    let mut heap = BinaryHeap::new();
    let mut start_nodes = map.create_nodes_in_direction(0, 0, Right, 0, min, max);
    start_nodes.append(&mut map.create_nodes_in_direction(0, 0, Down, 0, min, max));
    for node in start_nodes {
        heap.push(node);
    }

    while let Some(node) = heap.pop() {
        let old_cost = match node.last_direction {
            Up | Down => &mut costs[node.y][node.x].0,
            Left | Right => &mut costs[node.y][node.x].1,
        };

        if node.cost < *old_cost {
            *old_cost = node.cost;

            let mut new_nodes = map.get_next_nodes(&node, min, max);
            while let Some(node) = new_nodes.pop() {
                heap.push(node);
            }
        }
    }

    let cost = costs[dest_y][dest_x];
    cost.0.min(cost.1)
}
///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> usize {
    solve_with_min_max(input, 1, 3)
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> usize {
    solve_with_min_max(input, 4, 10)
}

#[cfg(test)]
mod tests {
    use crate::day17::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            ),
            102
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            ),
            94
        );

        assert_eq!(
            part_two(
                "111111111111
999999999991
999999999991
999999999991
999999999991"
            ),
            71
        )
    }
}
