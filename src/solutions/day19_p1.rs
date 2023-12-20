use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
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
    Gt(Part, i32, WorkflowResult<'a>),
    Lt(Part, i32, WorkflowResult<'a>),
    Noop(WorkflowResult<'a>),
}

impl<'a> Workflow<'a> {
    fn next(&self, part: &HashMap<Part, i32>) -> Option<&WorkflowResult> {
        match self {
            Workflow::Gt(part_name, value, result) => {
                if part.get(part_name).unwrap() > value {
                    Some(result)
                } else {
                    None
                }
            }
            Workflow::Lt(part_name, value, result) => {
                if part.get(part_name).unwrap() < value {
                    Some(result)
                } else {
                    None
                }
            }
            Workflow::Noop(result) => Some(result),
        }
    }
}

pub fn solution(lines: Vec<&str>) -> String {
    let mut line_iter = lines.split(|line| line.is_empty());
    let workflows = line_iter.next().unwrap();
    let parts = line_iter.next().unwrap();

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
                                value.parse::<i32>().unwrap(),
                                WorkflowResult::from(result),
                            )
                        } else {
                            let (part, value) = compare.split_once('>').unwrap();
                            Workflow::Gt(
                                Part::from(part),
                                value.parse::<i32>().unwrap(),
                                WorkflowResult::from(result),
                            )
                        }
                    }
                    None => Workflow::Noop(WorkflowResult::from(workflow)),
                })
                .collect::<Vec<_>>();

            (name, workflows)
        }));

    let parts: Vec<HashMap<Part, i32>> = parts
        .iter()
        .map(|line| {
            let (_, part) = line.split_once('{').unwrap();
            let (part, _) = part.split_once('}').unwrap();
            HashMap::from_iter(part.split(',').map(|p| {
                let (ch, value) = p.split_once('=').unwrap();
                (Part::from(ch), value.parse::<i32>().unwrap())
            }))
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    for part in parts {
        let mut cur_workflow = &workflows_map["in"];
        let mut idx = 0;

        loop {
            match cur_workflow[idx].next(&part) {
                Some(WorkflowResult::Accept) => {
                    sum += part.values().sum::<i32>();
                    break;
                }
                Some(WorkflowResult::Reject) => {
                    break;
                }
                Some(WorkflowResult::Next(next)) => {
                    cur_workflow = &workflows_map[next];
                    idx = 0;
                }
                None => {
                    idx += 1;
                }
            }
        }
    }

    sum.to_string()
}
