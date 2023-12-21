use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq, Hash)]
enum Module<'a> {
    Broadcaster(Vec<&'a str>),
    FlipFlop(bool, Vec<&'a str>),
    Conjunction(Vec<&'a str>, Vec<&'a str>),
}

use Module::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

use Pulse::*;

pub fn solution(lines: Vec<&str>) -> String {
    let mut conjunctions: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut modules: HashMap<&str, Module> = lines
        .iter()
        .map(|line| {
            let (input, output) = line.split_once("->").unwrap();
            let output: Vec<&str> = output.split(',').map(|x| x.trim()).collect();
            if input.starts_with("broadcaster") {
                ("broadcaster", Broadcaster(output))
            } else if input.starts_with('%') {
                (input[1..].trim(), FlipFlop(false, output))
            } else if input.starts_with('&') {
                conjunctions.insert(input[1..].trim(), vec![]);
                (input[1..].trim(), Conjunction(vec![], output))
            } else {
                unreachable!()
            }
        })
        .collect();

    let mut last_pulse: HashMap<&str, Pulse> =
        HashMap::from_iter(modules.keys().map(|name| (*name, Low)));

    for (name, module) in &modules {
        let next_modules = match module {
            Broadcaster(next_modules) => next_modules,
            FlipFlop(_, next_modules) => next_modules,
            Conjunction(_, next_modules) => next_modules,
        };

        for next_module in next_modules {
            if let Some(Conjunction(_, _)) = modules.get(next_module) {
                conjunctions.get_mut(next_module).unwrap().push(name);
            }
        }
    }

    for (conjunction, prevs) in conjunctions {
        if let Conjunction(prev_modules, _) = modules.get_mut(conjunction).unwrap() {
            *prev_modules = prevs;
        }
    }

    let mut high_count: usize = 0;
    let mut low_count: usize = 0;

    for _ in 0..1000 {
        // button -> broadcaster
        low_count += 1;

        let mut queue = VecDeque::from(vec![("broadcaster", Low)]);

        while let Some((module_name, input_pulse)) = queue.pop_front() {
            if let Some(module) = modules.get_mut(module_name) {
                match module {
                    Broadcaster(next_modules) => {
                        low_count += next_modules.len();
                        for next_module in next_modules {
                            queue.push_back((next_module, Low));
                        }
                    }
                    FlipFlop(state, next_modules) => {
                        if let Low = input_pulse {
                            *state = !*state;

                            let next_pulse = match *state {
                                true => {
                                    high_count += next_modules.len();
                                    High
                                }
                                false => {
                                    low_count += next_modules.len();
                                    Low
                                }
                            };

                            *last_pulse.get_mut(module_name).unwrap() = next_pulse;

                            for next_module in next_modules {
                                queue.push_back((next_module, next_pulse));
                            }
                        }
                    }
                    Conjunction(prev_modules, next_modules) => {
                        let next_pulse = match prev_modules.iter().all(|p| last_pulse[p] == High) {
                            true => {
                                low_count += next_modules.len();
                                Low
                            }
                            false => {
                                high_count += next_modules.len();
                                High
                            }
                        };

                        *last_pulse.get_mut(module_name).unwrap() = next_pulse;

                        for next_module in next_modules {
                            queue.push_back((next_module, next_pulse));
                        }
                    }
                }
            }
        }
    }

    (high_count * low_count).to_string()
}
