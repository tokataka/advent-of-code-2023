use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

#[derive(Clone, Copy)]
enum Part {
    X,
    M,
    A,
    S,
}

impl Part {
    fn from(part: &str) -> Self {
        match part {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
struct PartRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl PartRange {
    fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn new_invalid() -> Self {
        Self {
            x: (1, 0),
            m: (1, 0),
            a: (1, 0),
            s: (1, 0),
        }
    }

    fn product(&self) -> i64 {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }

    fn is_valid(&self) -> bool {
        self.x.1 >= self.x.0 && self.m.1 >= self.m.0 && self.a.1 >= self.a.0 && self.s.1 >= self.s.0
    }
}

impl Index<Part> for PartRange {
    type Output = (i64, i64);

    fn index(&self, index: Part) -> &Self::Output {
        match index {
            Part::X => &self.x,
            Part::M => &self.m,
            Part::A => &self.a,
            Part::S => &self.s,
        }
    }
}

impl IndexMut<Part> for PartRange {
    fn index_mut(&mut self, index: Part) -> &mut Self::Output {
        match index {
            Part::X => &mut self.x,
            Part::M => &mut self.m,
            Part::A => &mut self.a,
            Part::S => &mut self.s,
        }
    }
}

enum WorkflowResult<'a> {
    Accept,
    Reject,
    Next(&'a str),
}

impl<'a> WorkflowResult<'a> {
    fn from(result: &'a str) -> Self {
        match result {
            "A" => Self::Accept,
            "R" => Self::Reject,
            x => Self::Next(x),
        }
    }
}

enum Workflow<'a> {
    Gt(Part, i64, WorkflowResult<'a>),
    Lt(Part, i64, WorkflowResult<'a>),
    Noop(WorkflowResult<'a>),
}

pub fn solution(lines: Vec<&str>) -> String {
    let mut line_iter = lines.split(|line| line.is_empty());
    let workflows = line_iter.next().unwrap();

    let workflows_map: HashMap<&str, Vec<Workflow>> =
        HashMap::from_iter(workflows.iter().map(|line| {
            let (name, workflows) = line.split_once('{').unwrap();

            let (workflows, _) = workflows.split_once('}').unwrap();

            let workflows = workflows
                .split(',')
                .map(|workflow| match workflow.split_once(':') {
                    Some((compare, result)) => {
                        if compare.contains('<') {
                            let (part, value) = compare.split_once('<').unwrap();
                            Workflow::Lt(
                                Part::from(part),
                                value.parse::<i64>().unwrap(),
                                WorkflowResult::from(result),
                            )
                        } else {
                            let (part, value) = compare.split_once('>').unwrap();
                            Workflow::Gt(
                                Part::from(part),
                                value.parse::<i64>().unwrap(),
                                WorkflowResult::from(result),
                            )
                        }
                    }
                    None => Workflow::Noop(WorkflowResult::from(workflow)),
                })
                .collect::<Vec<_>>();

            (name, workflows)
        }));

    let mut stack: Vec<(&str, usize, PartRange)> = vec![("in", 0, PartRange::new())];

    let mut accept_count = 0;

    while let Some((workflows_name, workflow_idx, part_range)) = stack.pop() {
        let (part_range_true, part_range_false, result) =
            match &workflows_map[workflows_name][workflow_idx] {
                Workflow::Gt(part, value, result) => {
                    let mut part_range_true = part_range;
                    part_range_true[*part].0 = *value + 1;

                    let mut part_range_false = part_range;
                    part_range_false[*part].1 = *value;

                    (part_range_true, part_range_false, result)
                }
                Workflow::Lt(part, value, result) => {
                    let mut part_range_true = part_range;
                    part_range_true[*part].1 = *value - 1;

                    let mut part_range_false = part_range;
                    part_range_false[*part].0 = *value;

                    (part_range_true, part_range_false, result)
                }
                Workflow::Noop(result) => (part_range, PartRange::new_invalid(), result),
            };

        if part_range_true.is_valid() {
            match result {
                WorkflowResult::Accept => {
                    accept_count += part_range_true.product();
                }
                WorkflowResult::Reject => {}
                WorkflowResult::Next(next_workflows_name) => {
                    stack.push((next_workflows_name, 0, part_range_true));
                }
            }
        }

        if part_range_false.is_valid() {
            stack.push((workflows_name, workflow_idx + 1, part_range_false));
        }
    }

    accept_count.to_string()
}
