use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Default)]
pub struct OrderingRules {
    ordering_rules: HashMap<u64, Vec<u64>>,
}

impl OrderingRules {
    pub fn add_rule(&mut self, before_page: u64, after_page: u64) {
        self.ordering_rules
            .entry(before_page)
            .or_default()
            .push(after_page);
    }

    pub fn is_in_valid_order(&self, before_page: u64, after_pages: &[u64]) -> bool {
        for after_page in after_pages {
            if self
                .ordering_rules
                .get(after_page)
                .unwrap_or(&vec![])
                .contains(&before_page)
            {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
pub struct InputData {
    ordering_rules: OrderingRules,
    updates: Vec<Vec<u64>>,
}

impl From<Vec<String>> for InputData {
    fn from(rows: Vec<String>) -> Self {
        let mut ordering_rules: OrderingRules = OrderingRules::default();
        let mut updates: Vec<Vec<u64>> = vec![];

        for row in rows {
            if row.is_empty() {
                continue;
            }
            if row.contains('|') {
                let mut parts = row.split("|");
                let first = parts.next().unwrap().parse::<u64>().unwrap();
                let second = parts.next().unwrap().parse::<u64>().unwrap();
                ordering_rules.add_rule(first, second);
            } else {
                let parts = row.split(",");
                let update = parts
                    .map(|part| part.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                if !update.is_empty() {
                    updates.push(update);
                }
            }
        }

        Self {
            ordering_rules,
            updates,
        }
    }
}

impl InputData {
    pub fn get_updates(&self) -> &Vec<Vec<u64>> {
        &self.updates
    }

    pub fn get_ordering_rules(&self) -> &OrderingRules {
        &self.ordering_rules
    }
}

fn is_valid_update(update: &[u64], ordering_rules: &OrderingRules) -> bool {
    for (i, &page) in update.iter().enumerate() {
        let subslice = &update[i + 1..];
        if !ordering_rules.is_in_valid_order(page, subslice) {
            return false;
        }
    }

    true
}

fn fix_update<'a>(update: &'a mut Vec<u64>, ordering_rules: &OrderingRules) -> &'a mut Vec<u64> {
    for i in (0..update.len()).step_by(1) {
        let page = update[i];
        let next_page = safe_subslice(update, i + 1, i + 2);
        if !ordering_rules.is_in_valid_order(page, next_page) {
            // Swap the pages
            update[i] = next_page[0];
            update[i + 1] = page;
        }
    }

    if is_valid_update(update, ordering_rules) {
        update
    } else {
        fix_update(update, ordering_rules)
    }
}

fn get_middle_element<T>(vec: &[T]) -> Option<&T> {
    if vec.is_empty() {
        None
    } else {
        Some(&vec[vec.len() / 2])
    }
}

fn solve(lines: Vec<String>) -> (u64, u64) {
    let mut part1_result = 0;
    let mut part2_result = 0;

    let input_data: InputData = InputData::from(lines);
    for update in input_data.get_updates() {
        if is_valid_update(update, input_data.get_ordering_rules()) {
            part1_result += get_middle_element(update).unwrap();
        } else {
            let mut update_clone = update.clone();
            let fixed_update = fix_update(&mut update_clone, input_data.get_ordering_rules());

            part2_result += get_middle_element(fixed_update).unwrap();
        }
    }

    (part1_result, part2_result)
}
fn main() {
    let solution = solve(read_lines("input.txt"));

    println!("Solution for part 1 is {}", solution.0);
    println!("Solution for part 2 is {}", solution.1);
}

// Utilities

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn safe_subslice<T>(vec: &[T], start: usize, end: usize) -> &[T] {
    if start >= vec.len() || start >= end {
        return &[];
    }
    let end = end.min(vec.len());
    &vec[start..end]
}

#[cfg(test)]
mod test {
    use super::solve;

    const EXAMPLE: [&str; 28] = [
        "47|53",
        "97|13",
        "97|61",
        "97|47",
        "75|29",
        "61|13",
        "75|53",
        "29|13",
        "97|29",
        "53|29",
        "61|53",
        "97|53",
        "61|29",
        "47|13",
        "75|47",
        "97|75",
        "47|61",
        "75|61",
        "47|29",
        "75|13",
        "53|13",
        "",
        "75,47,61,53,29",
        "97,61,53,29,13",
        "75,29,13",
        "75,97,47,61,53",
        "61,13,29",
        "97,13,75,29,47",
    ];

    #[test]
    fn solve_example_part1() {
        assert_eq!(solve(EXAMPLE.map(String::from).to_vec()), (143, 123));
    }
}
