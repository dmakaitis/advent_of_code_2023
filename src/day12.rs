use crate::day12::DfaState::*;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
enum DfaState {
    RepDotOrHash,
    Hash,
    Dot,
    RepDotOrAccept,
}

fn build_dfa(checksum: &Vec<i32>) -> Vec<DfaState> {
    let mut result = vec![];
    let mut first = true;

    for c in checksum {
        if !first {
            result.push(Dot);
        }
        first = false;

        result.push(RepDotOrHash);
        for _ in 0..(c - 1) {
            result.push(Hash);
        }
    }

    result.push(RepDotOrAccept);

    result
}

fn get_valid_possibility_count(s: &str) -> i64 {
    let (a, b) = s.split_whitespace().collect_tuple().unwrap();

    let checksum = b.split(',').flat_map(|s| s.parse::<i32>()).collect_vec();

    let dfa = build_dfa(&checksum);
    let mut heads = vec![0; dfa.len()];
    heads[0] = 1;

    // println!("Heads: {:?}", heads);
    for c in a.chars() {
        let mut next_heads = vec![0; dfa.len()];

        for i in 0..heads.len() {
            let index = dfa.len() - (i + 1);
            match dfa[index] {
                RepDotOrHash => match c {
                    '.' => {
                        next_heads[index] += heads[index];
                    }
                    '#' => {
                        next_heads[index + 1] += heads[index];
                    }
                    '?' => {
                        next_heads[index] += heads[index];
                        next_heads[index + 1] += heads[index];
                    }
                    _ => {
                        panic!("Invalid character in spring pattern");
                    }
                },
                Hash => {
                    match c {
                        '#' | '?' => {
                            next_heads[index + 1] += heads[index];
                        }
                        '.' => {} // reject
                        _ => {
                            panic!("Invalid character in spring pattern");
                        }
                    }
                }
                Dot => {
                    match c {
                        '.' | '?' => {
                            next_heads[index + 1] += heads[index];
                        }
                        '#' => {} // reject
                        _ => {
                            panic!("Invalid character in spring pattern");
                        }
                    }
                }
                RepDotOrAccept => {
                    match c {
                        '.' | '?' => {
                            next_heads[index] += heads[index];
                        }
                        '#' => {} // reject
                        _ => {
                            panic!("Invalid character in spring pattern");
                        }
                    }
                }
            }
        }

        heads = next_heads;
    }

    // The last node is our only accept node, so just return how many heads made it there:
    heads[heads.len() - 1]
}

fn expand_input(s: &str) -> String {
    let (a, b) = s.split_whitespace().collect_tuple().unwrap();

    format!("{a}?{a}?{a}?{a}?{a} {b},{b},{b},{b},{b}")
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i64 {
    input.lines().map(get_valid_possibility_count).sum()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i64 {
    input
        .lines()
        .map(expand_input)
        .map(|line| get_valid_possibility_count(line.as_str()))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day12::*;

    #[test]
    fn test_build_dfa() {
        let dfa = build_dfa(&vec![3, 2, 1]);

        assert_eq!(
            dfa,
            vec![
                RepDotOrHash,
                Hash,
                Hash,
                Dot,
                RepDotOrHash,
                Hash,
                Dot,
                RepDotOrHash,
                RepDotOrAccept
            ]
        );
    }

    #[test]
    fn test_get_valid_possibility_count() {
        assert_eq!(get_valid_possibility_count("???.### 1,1,3"), 1);
        assert_eq!(get_valid_possibility_count(".??..??...?##. 1,1,3"), 4);
        assert_eq!(get_valid_possibility_count("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(get_valid_possibility_count("????.#...#... 4,1,1"), 1);
        assert_eq!(get_valid_possibility_count("????.######..#####. 1,6,5"), 4);
        assert_eq!(get_valid_possibility_count("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn test_expand() {
        assert_eq!(expand_input(".# 1"), ".#?.#?.#?.#?.# 1,1,1,1,1");
        assert_eq!(
            expand_input("???.### 1,1,3"),
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
        );
    }

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            ),
            21
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            ),
            525152
        );
    }
}
