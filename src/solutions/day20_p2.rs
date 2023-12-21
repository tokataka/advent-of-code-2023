use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq, Hash)]
enum Module<'a> {
    Broadcaster(Vec<&'a str>),
    FlipFlop(bool, Vec<&'a str>),
    Conjunction(Vec<&'a str>, Vec<&'a str>),
}

use Module::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Pulse {
    Low,
    High,
    Init,
}

impl Pulse {
    fn toggle(&self) -> Self {
        match self {
            Low => High,
            High => Low,
            Init => Init,
        }
    }
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
        HashMap::from_iter(modules.keys().map(|name| (*name, Init)));

    let mut conjunctions_needed: HashMap<&str, Pulse> = HashMap::new();

    let mut stack: Vec<(&str, Pulse)> = vec![("rx", High)];
    while let Some((name, required_pulse)) = stack.pop() {
        let mut has_child_conjunction = false;

        for module in &modules {
            if let (prev_name, Conjunction(_, nexts)) = module {
                if nexts.contains(&name) {
                    if !conjunctions_needed.contains_key(prev_name) {
                        stack.push((prev_name, required_pulse.toggle()));
                        has_child_conjunction = true;
                    }
                }
            }
        }

        if !has_child_conjunction {
            conjunctions_needed.insert(name, required_pulse);
        }
    }

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

    let mut conjunctions_needed_min_count: HashMap<&str, usize> = HashMap::new();

    for button_press_count in 1.. {
        let mut queue = VecDeque::from(vec![("broadcaster", Low)]);

        while let Some((module_name, input_pulse)) = queue.pop_front() {
            if module_name.eq("rx") && input_pulse == Low {
                return button_press_count.to_string();
            }

            if let Some(module) = modules.get_mut(module_name) {
                match module {
                    Broadcaster(next_modules) => {
                        for next_module in next_modules {
                            queue.push_back((next_module, Low));
                        }
                    }
                    FlipFlop(state, next_modules) => {
                        if let Low = input_pulse {
                            *state = !*state;

                            let next_pulse = match *state {
                                true => High,
                                false => Low,
                            };

                            *last_pulse.get_mut(module_name).unwrap() = next_pulse;

                            for next_module in next_modules {
                                queue.push_back((next_module, next_pulse));
                            }
                        }
                    }
                    Conjunction(prev_modules, next_modules) => {
                        let next_pulse = match prev_modules.iter().all(|p| last_pulse[p] == High) {
                            true => Low,
                            false => High,
                        };

                        *last_pulse.get_mut(module_name).unwrap() = next_pulse;

                        for next_module in next_modules {
                            queue.push_back((next_module, next_pulse));
                        }
                    }
                }
            }

            for (name, pulse) in &conjunctions_needed {
                if last_pulse[name] == *pulse {
                    if !conjunctions_needed_min_count.contains_key(name) {
                        conjunctions_needed_min_count.insert(name, button_press_count);
                    }
                }
            }

            if conjunctions_needed_min_count.len() == conjunctions_needed.len() {
                let mut common_mul = 1;

                for (_, count) in conjunctions_needed_min_count {
                    if common_mul == 1 {
                        common_mul = count;
                    }

                    for i in 1..=count {
                        if (common_mul * i) % count == 0 {
                            common_mul *= i;
                            break;
                        }
                    }
                }

                return common_mul.to_string();
            }
        }
    }

    0.to_string()
}
