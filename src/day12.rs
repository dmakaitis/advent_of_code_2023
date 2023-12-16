use itertools::Itertools;

fn get_possibilities(s: &str) -> Vec<String> {
    if s.len() <= 1 {
        if s == "?" {
            return vec![String::from("."), String::from("#")];
        } else {
            return vec![String::from(s)];
        }
    }

    let head = &s[0..1];
    let tail = &s[1..];

    let tail_possiblities = get_possibilities(tail);

    let mut result = vec![];

    if head == "." || head == "?" {
        for p in &tail_possiblities {
            result.push(format!(".{p}"));
        }
    }
    if head == "#" || head == "?" {
        for p in &tail_possiblities {
            result.push(format!("#{p}"));
        }
    }

    result
}

fn calculate_checksum(s: &str) -> Vec<i32> {
    let mut result = vec![];
    let mut count = 0;

    for c in s.chars() {
        if c == '.' {
            if count > 0 {
                result.push(count);
                count = 0;
            }
        } else {
            count += 1;
        }
    }

    if count > 0 {
        result.push(count);
    }

    result
}

fn get_valid_possibility_count(s: &str) -> i32 {
    let (a, b) = s.split_whitespace().collect_tuple().unwrap();

    let possibilties = get_possibilities(a);
    let checksum = b
        .split(',')
        .map(|s| s.parse::<i32>())
        .flatten()
        .collect_vec();

    possibilties
        .iter()
        .filter(|p| calculate_checksum(p.as_str()) == checksum)
        .count() as i32
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
pub fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|line| get_valid_possibility_count(line))
        .sum()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|line| expand_input(line))
        .inspect(|line| print!("{line} => "))
        .map(|line| get_valid_possibility_count(line.as_str()))
        .inspect(|v| println!("{v}"))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day12::*;

    #[test]
    fn test_get_possibilities() {
        assert_eq!(get_possibilities(".###"), vec![String::from(".###"),]);

        assert_eq!(
            get_possibilities("?.###"),
            vec![String::from("..###"), String::from("#.###"),]
        );

        assert_eq!(
            get_possibilities("??.###"),
            vec![
                String::from("...###"),
                String::from(".#.###"),
                String::from("#..###"),
                String::from("##.###"),
            ]
        );

        assert_eq!(
            get_possibilities("???.###"),
            vec![
                String::from("....###"),
                String::from("..#.###"),
                String::from(".#..###"),
                String::from(".##.###"),
                String::from("#...###"),
                String::from("#.#.###"),
                String::from("##..###"),
                String::from("###.###"),
            ]
        );

        assert_eq!(
            get_possibilities("??"),
            vec![
                String::from(".."),
                String::from(".#"),
                String::from("#."),
                String::from("##"),
            ]
        )
    }

    #[test]
    fn test_calculate_checksum() {
        assert_eq!(calculate_checksum("#.#.###"), vec![1, 1, 3]);
        assert_eq!(calculate_checksum(".#...#....###."), vec![1, 1, 3]);
        assert_eq!(calculate_checksum(".#.###.#.######"), vec![1, 3, 1, 6]);
        assert_eq!(calculate_checksum("####.#...#..."), vec![4, 1, 1]);
        assert_eq!(calculate_checksum("#....######..#####."), vec![1, 6, 5]);
        assert_eq!(calculate_checksum(".###.##....#"), vec![3, 2, 1]);
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

    //     #[test]
    //     fn part_two_correct() {
    //         assert_eq!(part_two("???.### 1,1,3
    // .??..??...?##. 1,1,3
    // ?#?#?#?#?#?#?#? 1,3,1,6
    // ????.#...#... 4,1,1
    // ????.######..#####. 1,6,5
    // ?###???????? 3,2,1"), 525152);
    //     }
}
