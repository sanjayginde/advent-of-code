pub mod parts;

use std::{collections::HashMap, fs::read_to_string};

use parts::{Part, Workflow};

fn parse(lines: Vec<String>) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut parse_workflow = true;

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    for line in lines {
        if line.trim().len() == 0 {
            parse_workflow = false;
            continue;
        }

        match parse_workflow {
            true => {
                let workflow = Workflow::from(line);
                workflows.insert(workflow.id(), workflow);
            }
            false => {
                parts.push(Part::from(line));
            }
        }
    }

    (workflows, parts)
}

fn solve_part1(workflows: &HashMap<String, Workflow>, parts: &Vec<Part>) -> usize {
    let mut total = 0;

    let initial_workflow = workflows.get("in").unwrap();

    for part in parts {
        let mut next_workflow = Some(initial_workflow);

        while let Some(workflow) = next_workflow {
            match workflow.result(part) {
                parts::StepResult::NextStep(id) => next_workflow = workflows.get(id),
                parts::StepResult::Accepted => {
                    total += part.sum_of_ratings();
                    next_workflow = None;
                }
                parts::StepResult::Rejected => next_workflow = None,
            }
        }
    }

    total
}

fn solve_part2() -> usize {
    0
}

fn main() {
    let (workflows, mut parts) = parse(read_lines("input.txt"));

    println!(
        "Solution for part 1 is {}",
        solve_part1(&workflows, &mut parts)
    );
    println!("Solution for part 2 is {}", solve_part2());
}

#[cfg(test)]
mod test {

    use super::{parse, solve_part1};

    const EXAMPLE: [&str; 17] = [
        "px{a<2006:qkq,m>2090:A,rfg}",
        "pv{a>1716:R,A}",
        "lnx{m>1548:A,A}",
        "rfg{s<537:gd,x>2440:R,A}",
        "qs{s>3448:A,lnx}",
        "qkq{x<1416:A,crn}",
        "crn{x>2662:A,R}",
        "in{s<1351:px,qqz}",
        "qqz{s>2770:qs,m<1801:hdj,R}",
        "gd{a>3333:R,R}",
        "hdj{m>838:A,pv}",
        "",
        "{x=787,m=2655,a=1222,s=2876}",
        "{x=1679,m=44,a=2067,s=496}",
        "{x=2036,m=264,a=79,s=2244}",
        "{x=2461,m=1339,a=466,s=291}",
        "{x=2127,m=1623,a=2188,s=1013}",
    ];

    #[test]
    fn solve_example_part1() {
        let (workflows, mut parts) = parse(EXAMPLE.map(String::from).to_vec());
        assert_eq!(solve_part1(&workflows, &mut parts), 19114);
    }
}

// Utilities

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
