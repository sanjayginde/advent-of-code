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

fn validate_update(update: &[u64], ordering_rules: &OrderingRules) -> bool {
    for (i, &page) in update.iter().enumerate() {
        let subslice = &update[i + 1..];
        if !ordering_rules.is_in_valid_order(page, subslice) {
            return false;
        }
    }

    true
}

fn get_middle_element<T>(vec: &[T]) -> Option<&T> {
    if vec.is_empty() {
        None
    } else {
        Some(&vec[vec.len() / 2]) // Middle index is `len / 2`
    }
}

fn part1(lines: Vec<String>) -> u64 {
    let mut result = 0;

    let input_data: InputData = InputData::from(lines);

    let updates = input_data.get_updates();
    for update in updates {
        if validate_update(update, input_data.get_ordering_rules()) {
            result += get_middle_element(update).unwrap();
        }
    }

    result
}

// fn part2(_lines: Vec<String>) -> u64 {
//     todo!()
// }

fn main() {
    println!("Solution for part 1 is {}", part1(read_lines("input.txt")));
    // println!("Solution for part 2 is {}", part2(read_lines("input.txt")));
}

// Utilities

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod test {
    use super::part1;
    // use super::part2;

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
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 143);
    }

    // #[test]
    // fn solve_example_part2() {
    //     assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 31);
    // }
}
