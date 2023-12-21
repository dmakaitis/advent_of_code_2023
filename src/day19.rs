use crate::day19::Operator::{Greater, Less};
use crate::day19::Variable::{A, M, S, X};
use itertools::Itertools;
use std::collections::HashMap;

enum Variable {
    X,
    M,
    A,
    S,
}

enum Operator {
    Less,
    Greater,
}

struct Rule<'a> {
    var: Variable,
    op: Operator,
    val: u32,
    result: &'a str,
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(value: &'a str) -> Self {
        let var = match &value[0..1] {
            "x" => Some(X),
            "m" => Some(M),
            "a" => Some(A),
            "s" => Some(S),
            _ => None,
        }
        .unwrap();

        let op = match &value[1..2] {
            "<" => Some(Less),
            ">" => Some(Greater),
            _ => None,
        }
        .unwrap();

        let (val, result) = value[2..].split(":").collect_tuple().unwrap();

        let val = val.parse::<u32>().unwrap();

        Rule {
            var,
            op,
            val,
            result,
        }
    }
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    default: &'a str,
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(value: &'a str) -> Self {
        let mut parts = value.split(",").collect_vec();
        let default = parts[parts.len() - 1];

        parts.remove(parts.len() - 1);

        let rules = parts.iter().map(|r| Rule::from(*r)).collect_vec();

        Workflow { rules, default }
    }
}

impl<'a> Workflow<'a> {
    fn get_result(&self, part: &Part) -> &'a str {
        for rule in &self.rules {
            let var = match rule.var {
                X => part.x,
                M => part.m,
                A => part.a,
                S => part.s,
            };

            let val = rule.val;

            let success = match rule.op {
                Less => var < val,
                Greater => var > val,
            };

            if success {
                return rule.result;
            }
        }

        return self.default;
    }
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let values = value
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .split(",")
            .collect_vec();
        let mut i = values.iter();

        let x = i
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let m = i
            .next()
            .unwrap()
            .strip_prefix("m=")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let a = i
            .next()
            .unwrap()
            .strip_prefix("a=")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let s = i
            .next()
            .unwrap()
            .strip_prefix("s=")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        Part { x, m, a, s }
    }
}

fn parse_workflow(s: &str) -> (&str, Workflow) {
    let (name, rules) = s
        .strip_suffix("}")
        .unwrap()
        .split("{")
        .collect_tuple()
        .unwrap();

    let workflow = Workflow::from(rules);

    (name, workflow)
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> u32 {
    let (workflows, parts) = input.split("\n\n").collect_tuple().unwrap();

    let workflows: HashMap<&str, Workflow> = workflows.lines().map(|w| parse_workflow(w)).collect();

    let parts = parts.lines().map(|line| Part::from(line)).collect_vec();

    let mut accepted = vec![];

    for part in &parts {
        // println!("Checking new part...");
        let mut cur = "in";

        loop {
            let result = workflows[cur].get_result(part);
            // println!("    {cur} => {result}");
            if result == "A" {
                accepted.push(part);
                break;
            } else if result == "R" {
                break;
            } else {
                cur = result;
            }
        }
    }

    // println!("Accepted part count: {}", accepted.len());

    accepted.iter().map(|p| p.x + p.m + p.a + p.s).sum()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day19::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
            ),
            19114
        );
    }

    // #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
            ),
            167409079868000
        );
    }
}
