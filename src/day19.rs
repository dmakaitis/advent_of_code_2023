use crate::day19::Operator::{Greater, Less};
use crate::day19::Variable::{A, M, S, X};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
enum Variable {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Eq, PartialEq)]
enum Operator {
    Less,
    Greater,
}

#[derive(Debug)]
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

impl<'a> Rule<'a> {
    fn split_template(&self, template: &Template) -> (Option<Template>, Option<Template>) {
        let rule = template.rule.clone();
        let Template {
            rule: _,
            x,
            m,
            a,
            s,
        } = *template;

        match self.var {
            X => {
                let n;
                let r;

                if self.op == Less {
                    (n, r) = if template.x.1 < self.val {
                        (None, Some(Template { rule, x, m, a, s }))
                    } else {
                        (
                            Some(Template {
                                rule: String::from(self.result),
                                x: (template.x.0, self.val - 1),
                                m,
                                a,
                                s,
                            }),
                            Some(Template {
                                rule,
                                x: (self.val, template.x.1),
                                m,
                                a,
                                s,
                            }),
                        )
                    };
                } else {
                    (n, r) = if template.x.0 > self.val {
                        (None, Some(Template { rule, x, m, a, s }))
                    } else {
                        (
                            Some(Template {
                                rule: String::from(self.result),
                                x: (self.val + 1, template.x.1),
                                m,
                                a,
                                s,
                            }),
                            Some(Template {
                                rule,
                                x: (template.x.0, self.val),
                                m,
                                a,
                                s,
                            }),
                        )
                    };
                }
                return (n, r);
            }
            M => {
                let n;
                let r;

                if self.op == Less {
                    (n, r) = if template.m.1 < self.val {
                        (None, Some(Template { rule, x, m, a, s }))
                    } else {
                        (
                            Some(Template {
                                rule: String::from(self.result),
                                x,
                                m: (template.m.0, self.val - 1),
                                a,
                                s,
                            }),
                            Some(Template {
                                rule,
                                x,
                                m: (self.val, template.m.1),
                                a,
                                s,
                            }),
                        )
                    };
                } else {
                    (n, r) = if template.m.0 > self.val {
                        (None, Some(Template { rule, x, m, a, s }))
                    } else {
                        (
                            Some(Template {
                                rule: String::from(self.result),
                                x,
                                m: (self.val + 1, template.m.1),
                                a,
                                s,
                            }),
                            Some(Template {
                                rule,
                                x,
                                m: (template.m.0, self.val),
                                a,
                                s,
                            }),
                        )
                    };
                }
                return (n, r);
            }
            A => {
                let n;
                let r;

                if self.op == Less {
                    (n, r) = if template.a.1 < self.val {
                        (None, Some(Template { rule, x, m, a, s }))
                    } else {
                        (
                            Some(Template {
                                rule: String::from(self.result),
                                x,
                                m,
                                a: (template.a.0, self.val - 1),
                                s,
                            }),
                            Some(Template {
                                rule,
                                x,
                                m,
                                a: (self.val, template.a.1),
                                s,
                            }),
                        )
                    };
                } else {
                    (n, r) = if template.a.0 > self.val {
                        (None, Some(Template { rule, x, m, a, s }))
                    } else {
                        (
                            Some(Template {
                                rule: String::from(self.result),
                                x,
                                m,
                                a: (self.val + 1, template.a.1),
                                s,
                            }),
                            Some(Template {
                                rule,
                                x,
                                m,
                                a: (template.a.0, self.val),
                                s,
                            }),
                        )
                    };
                }
                return (n, r);
            }
            S => {
                let n;
                let r;

                if self.op == Less {
                    (n, r) = if template.s.1 < self.val {
                        (None, Some(Template { rule, x, m, a, s }))
                    } else {
                        (
                            Some(Template {
                                rule: String::from(self.result),
                                x,
                                m,
                                a,
                                s: (template.s.0, self.val - 1),
                            }),
                            Some(Template {
                                rule,
                                x,
                                m,
                                a,
                                s: (self.val, template.s.1),
                            }),
                        )
                    };
                } else {
                    (n, r) = if template.s.0 > self.val {
                        (None, Some(Template { rule, x, m, a, s }))
                    } else {
                        (
                            Some(Template {
                                rule: String::from(self.result),
                                x,
                                m,
                                a,
                                s: (self.val + 1, template.s.1),
                            }),
                            Some(Template {
                                rule,
                                x,
                                m,
                                a,
                                s: (template.s.0, self.val),
                            }),
                        )
                    };
                }
                return (n, r);
            }
        }
    }
}

#[derive(Debug)]
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

    fn split_template(&self, template: &Template) -> Vec<Template> {
        let mut t = template.clone();
        let mut result = vec![];

        for rule in &self.rules {
            let (a, b) = rule.split_template(&t);
            if let Some(a) = a {
                result.push(a);
            }

            if let Some(b) = b {
                t = b.clone();
            } else {
                break;
            }
        }
        t.rule = String::from(self.default);
        result.push(t);

        result
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

#[derive(Debug, Eq, PartialEq, Clone)]
struct Template {
    rule: String,
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

impl Template {
    fn new() -> Self {
        Template {
            rule: String::from("in"),
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn get_distinct_count(&self) -> u64 {
        let x = (self.x.1 + 1 - self.x.0) as u64;
        let m = (self.m.1 + 1 - self.m.0) as u64;
        let a = (self.a.1 + 1 - self.a.0) as u64;
        let s = (self.s.1 + 1 - self.s.0) as u64;

        x * m * a * s
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
        let mut cur = "in";

        loop {
            let result = workflows[cur].get_result(part);
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

    accepted.iter().map(|p| p.x + p.m + p.a + p.s).sum()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> u64 {
    let (workflows, _) = input.split("\n\n").collect_tuple().unwrap();
    let workflows: HashMap<&str, Workflow> = workflows.lines().map(|w| parse_workflow(w)).collect();

    let mut templates = vec![Template::new()];
    let mut accepted = vec![];

    while let Some(template) = templates.pop() {
        let rule = &workflows[template.rule.as_str()];
        let new_templates = rule.split_template(&template);

        for t in new_templates {
            if t.rule == "A" {
                accepted.push(t);
            } else if t.rule != "R" {
                templates.push(t);
            }
        }
    }

    accepted.iter().map(|a| a.get_distinct_count()).sum()
}

#[cfg(test)]
mod tests {
    use crate::day19::*;

    #[test]
    fn test_split_template_on_rule() {
        let template = Template::new();
        let rule = Rule {
            var: X,
            op: Less,
            val: 1000,
            result: "a",
        };
        let (a, b) = rule.split_template(&template);

        assert_eq!(
            a,
            Some(Template {
                rule: String::from("a"),
                x: (1, 999),
                m: (1, 4000),
                a: (1, 4000),
                s: (1, 4000)
            })
        );
        assert_eq!(
            b,
            Some(Template {
                rule: String::from("in"),
                x: (1000, 4000),
                m: (1, 4000),
                a: (1, 4000),
                s: (1, 4000)
            })
        );

        let rule = Rule {
            var: X,
            op: Greater,
            val: 1000,
            result: "a",
        };
        let (a, b) = rule.split_template(&template);

        assert_eq!(
            a,
            Some(Template {
                rule: String::from("a"),
                x: (1001, 4000),
                m: (1, 4000),
                a: (1, 4000),
                s: (1, 4000)
            })
        );
        assert_eq!(
            b,
            Some(Template {
                rule: String::from("in"),
                x: (1, 1000),
                m: (1, 4000),
                a: (1, 4000),
                s: (1, 4000)
            })
        );

        let rule = Rule {
            var: M,
            op: Less,
            val: 1000,
            result: "a",
        };
        let (a, b) = rule.split_template(&template);

        assert_eq!(
            a,
            Some(Template {
                rule: String::from("a"),
                x: (1, 4000),
                m: (1, 999),
                a: (1, 4000),
                s: (1, 4000)
            })
        );
        assert_eq!(
            b,
            Some(Template {
                rule: String::from("in"),
                x: (1, 4000),
                m: (1000, 4000),
                a: (1, 4000),
                s: (1, 4000)
            })
        );
    }
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

    #[test]
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
