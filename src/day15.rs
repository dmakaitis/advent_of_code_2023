use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: i32,
}

#[derive(Debug)]
enum Operation {
    Remove,
    Add(i32),
}

#[derive(Debug)]
struct Step<'a> {
    label: &'a str,
    operation: Operation,
}

impl<'a> From<&'a str> for Step<'a> {
    fn from(value: &'a str) -> Self {
        let index = value.find('-');
        if let Some(end) = index {
            return Step {
                label: &value[0..end],
                operation: Operation::Remove,
            };
        }

        let mut parts = value.split('=');
        let label = parts.next().unwrap();
        let focal_length = parts.next().unwrap().parse::<i32>().unwrap();

        Step {
            label,
            operation: Operation::Add(focal_length),
        }
    }
}

fn calculate_hash(s: &str) -> i32 {
    let mut hash = 0;

    for c in s.chars() {
        hash += c as i32;
        hash *= 17;
        hash &= 0xff;
    }

    hash
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i32 {
    input.split(',').map(calculate_hash).sum()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i32 {
    let mut boxes = vec![VecDeque::<Lens>::new(); 256];

    let steps = input.split(',').map(Step::from).collect_vec();

    for step in steps {
        let hash = calculate_hash(step.label);
        let lens_box = &mut boxes[hash as usize];

        match step.operation {
            Operation::Add(focal_length) => {
                if let Some(lens) = lens_box.iter_mut().find(|l| l.label == step.label) {
                    lens.focal_length = focal_length;
                } else {
                    lens_box.push_back(Lens {
                        label: step.label,
                        focal_length,
                    });
                }
            }
            Operation::Remove => {
                lens_box.retain(|l| l.label != step.label);
            }
        }
    }

    let mut total = 0;

    for (bi, b) in boxes.iter().enumerate() {
        for (li, l) in b.iter().enumerate() {
            total += ((bi + 1) as i32) * ((li + 1) as i32) * l.focal_length;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::day15::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145
        );
    }
}
