use num::integer::lcm;
use std::collections::HashMap;
use std::io::Write;

fn parse_node(s: &str) -> (&str, (&str, &str)) {
    let mut parts = s.split('=');
    let key = parts.next().unwrap().trim();

    let mut parts = parts
        .next()
        .unwrap()
        .trim()
        .strip_prefix('(')
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .split(',');

    let left = parts.next().unwrap().trim();
    let right = parts.next().unwrap().trim();

    (key, (left, right))
}

fn parse_input(s: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut parts = s.split("\n\n");

    let directions = parts.next().unwrap();

    let map: HashMap<_, _> = parts.next().unwrap().lines().map(parse_node).collect();

    (directions, map)
}

fn calc_steps_to_exit<'a, F>(
    start: (&'a str, usize),
    directions: &Vec<char>,
    branches: &'a HashMap<&str, (&str, &str)>,
    is_exit: F,
) -> ((&'a str, usize), i32)
where
    F: Fn(&str) -> bool,
{
    let mut node = start.0;
    let mut count = 0;
    let mut index = start.1;

    std::io::stdout().flush().unwrap();

    match directions[index] {
        'L' => node = branches.get(node).unwrap().0,
        'R' => node = branches.get(node).unwrap().1,
        _ => panic!("Invalid direction"),
    }

    std::io::stdout().flush().unwrap();

    count += 1;
    index = (index + 1) % directions.len();

    while !is_exit(node) {
        match directions[index] {
            'L' => node = branches.get(node).unwrap().0,
            'R' => node = branches.get(node).unwrap().1,
            _ => panic!("Invalid direction"),
        }

        std::io::stdout().flush().unwrap();

        count += 1;
        index = (index + 1) % directions.len();
    }

    ((node, index), count)
}

fn is_zzz(s: &str) -> bool {
    s == "ZZZ"
}

fn is_exit_node(s: &str) -> bool {
    s.ends_with('Z')
}

fn calc_lcm(a: u64, b: u64) -> u64 {
    lcm(a, b)
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i32 {
    let model = parse_input(input);

    let directions: Vec<_> = model.0.chars().collect();
    let branches = model.1;

    calc_steps_to_exit(("AAA", 0), &directions, &branches, is_zzz).1
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> u64 {
    let model = parse_input(input);

    let directions: Vec<_> = model.0.chars().collect();
    let branches = model.1;
    let nodes: Vec<_> = branches
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|s| (*s, 0usize))
        .collect();

    // It's not expressed in the problem, but each starting node leads to a fixed sized loop
    // leading repeatedly to an exit node, so we can optimize this solution to look for how many
    // times each path needs to be looped before everything converges to an equal number of total
    // moves...

    // We'll start by going through each starting node and mapping it to a list of loop sizes.
    // We'll also verify that every path does, indeed, result in a loop as a sanity check...

    let mut min_path_length = 0u64;

    for node in nodes {
        let mut result = calc_steps_to_exit(node, &directions, &branches, is_exit_node);
        let first_result = result;

        // Verify that continuing the path results in a loop
        for _ in 0..directions.len() {
            let next_result = calc_steps_to_exit(result.0, &directions, &branches, is_exit_node);

            assert_eq!(result.0 .0, next_result.0 .0);
            assert_eq!(result.1, next_result.1);

            if first_result.0 .1 == next_result.0 .1 {
                // We've found our loop
                break;
            }

            result = next_result;
        }

        // We've verified the loop size, so calculate the minimum path length (LCM) of this loop
        // along with all the previous loops

        if min_path_length == 0 {
            min_path_length = result.1 as u64;
        } else {
            min_path_length = calc_lcm(min_path_length, result.1 as u64);
        }
    }

    min_path_length
}

#[cfg(test)]
mod tests {
    use crate::day08::*;
    use std::collections::HashMap;

    #[test]
    fn parse_node_correctly() {
        assert_eq!(parse_node("AAA = (BBB, CCC)"), ("AAA", ("BBB", "CCC")));
    }

    #[test]
    fn parse_input_correctly() {
        let mut expected_map = HashMap::new();
        expected_map.insert("AAA", ("BBB", "BBB"));
        expected_map.insert("BBB", ("AAA", "ZZZ"));
        expected_map.insert("ZZZ", ("ZZZ", "ZZZ"));

        assert_eq!(
            parse_input(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            ("LLR", expected_map)
        );
    }

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            2
        );

        assert_eq!(
            part_one(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }
}
