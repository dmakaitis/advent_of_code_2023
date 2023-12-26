use crate::day20::Type::{Broadcast, Conjunction, FlipFlop};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Message<'a> {
    source: &'a str,
    target: &'a str,
    high: bool,
}

#[derive(Debug, Eq, PartialEq)]
enum Type {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    kind: Type,
    targets: Vec<&'a str>,
    on: bool,
    last_input: HashMap<String, bool>,
}

impl<'a> From<&'a str> for Module<'a> {
    fn from(value: &'a str) -> Self {
        let (name, targets) = value.split(" -> ").collect_tuple().unwrap();

        let (name, kind) = match name.chars().next().unwrap() {
            '%' => (&name[1..], FlipFlop),
            '&' => (&name[1..], Conjunction),
            _ => (name, Broadcast),
        };

        let targets = targets.split(',').map(|s| s.trim()).collect_vec();

        Module {
            kind,
            name,
            targets,
            on: false,
            last_input: HashMap::new(),
        }
    }
}

impl<'a> Module<'a> {
    fn get_output(&mut self, msg: &Message) -> Vec<Message<'a>> {
        let mut queue = vec![];

        match self.kind {
            Broadcast => {
                for t in &self.targets {
                    queue.push(Message {
                        source: self.name,
                        target: t,
                        high: msg.high,
                    });
                }
            }
            FlipFlop => {
                if !msg.high {
                    self.on = !self.on;
                    let high = self.on;

                    for t in &self.targets {
                        queue.push(Message {
                            source: self.name,
                            target: t,
                            high,
                        });
                    }
                }
            }
            Conjunction => {
                self.last_input.insert(String::from(msg.source), msg.high);

                let high = !self.last_input.iter().all(|(_, h)| *h);

                for t in &self.targets {
                    queue.push(Message {
                        source: self.name,
                        target: t,
                        high,
                    });
                }
            }
        }

        queue
    }
}

fn parse_modules(input: &str) -> Vec<Module> {
    let mut result: Vec<Module> = input.lines().map(Module::from).collect();

    let mut conjunction_inputs = vec![];

    result
        .iter()
        .filter(|m| m.kind == Conjunction)
        .for_each(|c| {
            result
                .iter()
                .filter(|m| m.targets.contains(&c.name))
                .for_each(|m| {
                    conjunction_inputs.push((c.name, m.name));
                })
        });

    for (c, m) in conjunction_inputs {
        let c = result.iter_mut().find(|m| m.name == c).unwrap();
        c.last_input.insert(String::from(m), false);
    }

    result
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i32 {
    let mut modules = parse_modules(input);

    let mut high_count = 0;
    let mut low_count = 0;

    for _ in 0..1000 {
        let mut message_queue = VecDeque::new();
        message_queue.push_back(Message {
            source: "button",
            high: false,
            target: "broadcaster",
        });

        while let Some(msg) = message_queue.pop_front() {
            if msg.high {
                high_count += 1;
            } else {
                low_count += 1;
            }

            if let Some(module) = modules.iter_mut().find(|m| msg.target == m.name) {
                let new_messages = module.get_output(&msg);

                for m in new_messages {
                    message_queue.push_back(m);
                }
            }
        }
    }

    low_count * high_count
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i64 {
    let mut modules = parse_modules(input);

    // Find the module that output to "rx"...
    let last_module = modules
        .iter()
        .filter(|m| m.targets.contains(&"rx"))
        .collect_vec();
    assert_eq!(last_module.len(), 1);
    let last_module = last_module.first().unwrap();
    assert_eq!(last_module.kind, Conjunction);

    // Find the modules that all output to the last module...
    let final_modules = modules
        .iter()
        .filter(|m| m.targets.contains(&last_module.name))
        .collect_vec();
    let mut final_modules: HashMap<_, _> = final_modules.iter().map(|m| (m.name, 0i64)).collect();

    // We'll need to count how many button presses it takes to make each of those final modules output a high signal.
    // That will be the cycle time for each module.
    // The number of button presses for the system to output a low signal is equal to the LCM of all the cycle times
    // of all the final modules. We'll assume that we can just multiply them together...

    let mut btn_count = 0i64;
    let mut output = 0i64;

    while output == 0 {
        btn_count += 1;

        let mut message_queue = VecDeque::new();
        message_queue.push_back(Message {
            source: "button",
            high: false,
            target: "broadcaster",
        });

        while let Some(msg) = message_queue.pop_front() {
            if let Some(fm) = final_modules.get(msg.source) {
                if *fm == 0 && msg.high {
                    final_modules.insert(msg.source, btn_count);

                    output = final_modules.values().product();

                    // Output will remain at zero until we've found a cycle time for all of the final modules
                    if output != 0 {
                        break;
                    }
                }
            }

            if let Some(module) = modules.iter_mut().find(|m| msg.target == m.name) {
                let new_messages = module.get_output(&msg);

                for m in new_messages {
                    message_queue.push_back(m);
                }
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::day20::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
            ),
            32000000
        );

        assert_eq!(
            part_one(
                "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
            ),
            11687500
        );
    }

    // No provided test values for part 2
}
