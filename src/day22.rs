use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
struct Brick {
    blocks: Vec<(i32, i32, i32)>,
    supported_by: HashSet<usize>,
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let (a, b) = value.split('~').collect_tuple().unwrap();
        let (x1, y1, z1) = a
            .split(',')
            .flat_map(|x| x.parse::<i32>())
            .collect_tuple()
            .unwrap();
        let (x2, y2, z2) = b
            .split(',')
            .flat_map(|x| x.parse::<i32>())
            .collect_tuple()
            .unwrap();

        // Blocks should always be given to us in ascending order, but let's be sure...
        assert!(x1 <= x2);
        assert!(y1 <= y2);
        assert!(z1 <= z2);

        let mut blocks = vec![];

        for x in x1..=x2 {
            for y in y1..=y2 {
                for z in z1..=z2 {
                    blocks.push((x, y, z));
                }
            }
        }

        Brick {
            blocks,
            supported_by: HashSet::new(),
        }
    }
}

fn settle(bricks: &mut [Brick]) {
    let width = bricks
        .iter()
        .map(|b| b.blocks.iter().map(|(x, _, _)| *x).max().unwrap())
        .max()
        .unwrap();
    let depth = bricks
        .iter()
        .map(|b| b.blocks.iter().map(|(_, y, _)| *y).max().unwrap())
        .max()
        .unwrap();

    let mut map = vec![vec![(0, None); (depth + 1) as usize]; (width + 1) as usize];

    for (i, brick) in bricks.iter_mut().enumerate() {
        let max_height_below = brick
            .blocks
            .iter()
            .map(|(x, y, _)| map[*x as usize][*y as usize].0)
            .max()
            .unwrap();

        brick.supported_by.clear();
        for (x, y, _) in &brick.blocks {
            let m = &map[*x as usize][*y as usize];
            if m.0 == max_height_below {
                if let Some(v) = m.1 {
                    brick.supported_by.insert(v);
                }
            }
        }

        let min_block_height = brick.blocks.iter().map(|(_, _, z)| *z).min().unwrap();

        let diff = (min_block_height - max_height_below) - 1;

        for b in &mut brick.blocks {
            b.2 -= diff;

            map[b.0 as usize][b.1 as usize] = (b.2, Some(i));
        }
    }
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i32 {
    let mut bricks = input
        .lines()
        .map(Brick::from)
        .sorted_by_key(|b| b.blocks.iter().map(|(_, _, z)| *z).min())
        .collect_vec();

    settle(&mut bricks);

    let mut safe_count = 0;

    for (i, _) in bricks.iter().enumerate() {
        let mut filter = HashSet::new();
        filter.insert(i);

        let any_match = bricks.iter().any(|b| b.supported_by == filter);

        if !any_match {
            // println!("Destroying brick {i} cause 0 bricks to fall");
            safe_count += 1;
        }
    }

    safe_count
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i32 {
    let mut bricks = input
        .lines()
        .map(Brick::from)
        .sorted_by_key(|b| b.blocks.iter().map(|(_, _, z)| *z).min())
        .collect_vec();

    settle(&mut bricks);

    let mut total_fall_count = 0;

    for (i, _) in bricks.iter().enumerate() {
        let mut destroyed = HashSet::new();
        destroyed.insert(i);

        let mut fall_count = 0;

        for (j, b) in bricks.iter().enumerate() {
            if j > i {
                let all_supports_destroyed = !b.supported_by.is_empty()
                    && b.supported_by.iter().all(|s| destroyed.contains(s));

                if all_supports_destroyed {
                    // This brick will fall...
                    fall_count += 1;
                    destroyed.insert(j);
                }
            }
        }

        total_fall_count += fall_count;
    }

    total_fall_count
}

#[cfg(test)]
mod tests {
    use crate::day22::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
            ),
            5
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
            ),
            7
        );
    }
}
