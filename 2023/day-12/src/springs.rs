#![allow(dead_code)]

use std::str::FromStr;

use regex::Regex;
use strum_macros::EnumString;

#[derive(Debug, EnumString, Clone, Copy, Hash, Eq, PartialEq)]
pub enum SpringState {
    #[strum(serialize = ".")]
    Operational,

    #[strum(serialize = "#")]
    Damaged,

    #[strum(serialize = "?")]
    Uknown,
}

impl SpringState {
    pub fn is_unkown(&self) -> bool {
        *self == SpringState::Uknown
    }

    pub fn is_known(&self) -> bool {
        !self.is_unkown()
    }
}

impl SpringState {
    fn from_char(value: char) -> Self {
        SpringState::from_str(value.to_string().as_str()).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Row {
    springs: Vec<SpringState>,
    damaged_groupings: Vec<usize>,
    damaged_regex: Regex,
}

// "???.### 1,1,3"
fn generate_combinations(springs: Vec<SpringState>) -> Vec<Vec<SpringState>> {
    let first_unknown = springs.iter().position(|s| s.is_unkown());
    if first_unknown.is_none() {
        return vec![springs];
    }

    let pos = first_unknown.unwrap();

    let splits = springs.split_at(pos);
    let lhs = splits.0.to_vec();
    let rhs = splits.1.to_vec();

    let mut damaged = rhs.clone();
    damaged[0] = SpringState::Damaged;

    let mut operational = rhs.clone();
    operational[0] = SpringState::Operational;

    let mut result: Vec<Vec<SpringState>> = Vec::new();
    for combo in generate_combinations(damaged).iter_mut() {
        let mut joined = lhs.clone();
        joined.append(combo);
        result.push(joined);
    }
    for combo in generate_combinations(operational).iter_mut() {
        let mut joined = lhs.clone();
        joined.append(combo);
        result.push(joined);
    }

    result
}

impl Row {
    pub fn calc_arrangements(&self) -> usize {
        let combinations = generate_combinations(self.springs.clone());
        combinations
            .into_iter()
            .fold(0, |acc, combo| match self.matches(combo) {
                true => acc + 1,
                false => acc,
            })
    }

    fn matches(&self, springs: Vec<SpringState>) -> bool {
        let mut str = "".to_string();
        for spring in springs.iter() {
            str.push(match spring {
                SpringState::Operational => '.',
                SpringState::Damaged => '#',
                SpringState::Uknown => panic!("uknown spring found {springs:?}"),
            });
        }
        self.damaged_regex.is_match(str.as_str())
    }
}

impl From<String> for Row {
    fn from(value: String) -> Self {
        let mut springs: Vec<SpringState> = Vec::new();
        let mut damaged_groupings: Vec<usize> = Vec::new();

        let splits: Vec<_> = value.split_whitespace().collect();

        for ch in splits[0].chars().into_iter() {
            springs.push(SpringState::from_char(ch));
        }

        let groupings: Vec<_> = splits[1].split(",").collect();

        for s in groupings.into_iter() {
            damaged_groupings.push(s.parse().unwrap());
        }

        let damaged_regex = damaged_groupings_regex(&damaged_groupings);
        Row {
            springs,
            damaged_groupings,
            damaged_regex,
        }
    }
}

fn damaged_groupings_regex(groupings: &Vec<usize>) -> Regex {
    let mut result = "^\\.*".to_string();
    let last_index = groupings.len() - 1;
    for (i, grouping) in groupings.iter().enumerate() {
        result.push_str(format!("#{{{grouping}}}").as_str());
        if i != last_index {
            result.push_str("\\.+");
        }
    }

    result.push_str("\\.*$");

    println!("regex: {result}");

    match Regex::new(result.as_str()) {
        Ok(regex) => regex,
        Err(err) => panic!("Error creating regex from {result}: {err}"),
    }
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Row::from(value.to_string())
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::Row;
    use super::SpringState::*;

    #[test]
    fn row_from_string() {
        let line = "???.### 1,1,3".to_string();

        let row = Row::from(line);

        assert_eq!(
            row.springs,
            vec![
                Uknown,
                Uknown,
                Uknown,
                Operational,
                Damaged,
                Damaged,
                Damaged
            ]
        );

        assert_eq!(row.damaged_groupings, vec![1, 1, 3]);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn calc_arrangements(#[case] row: &str, #[case] expected: usize) {
        let row = Row::from(row);

        assert_eq!(row.calc_arrangements(), expected);
    }
}
