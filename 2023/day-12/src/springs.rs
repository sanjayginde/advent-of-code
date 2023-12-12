use regex::Regex;

#[derive(Debug, Clone)]
pub struct Row {
    springs: String,
    arrangements_pattern: Regex,
}

fn generate_combinations(springs: String) -> Vec<String> {
    match springs.split_once("?") {
        None => vec![springs],
        Some((lhs, rhs)) => {
            let mut result: Vec<String> = Vec::new();
            for combo in generate_combinations(format!("#{rhs}")).into_iter() {
                result.push(format!("{lhs}{combo}"));
            }

            for combo in generate_combinations(format!(".{rhs}")).into_iter() {
                result.push(format!("{lhs}{combo}"));
            }

            result
        }
    }
}

impl Row {
    pub fn calc_arrangements(&self) -> usize {
        let combinations = generate_combinations(self.springs.clone());
        combinations.into_iter().fold(0, |acc, combo| {
            match self.arrangements_pattern.is_match(combo.as_str()) {
                true => acc + 1,
                false => acc,
            }
        })
    }
}

impl From<String> for Row {
    fn from(value: String) -> Self {
        let splits: Vec<_> = value.split_whitespace().collect();

        let springs = splits[0].to_string();
        let groupings: Vec<&str> = splits[1].split(",").collect();

        let last_index = groupings.len() - 1;
        let mut regex = "^\\.*".to_string();
        for (i, grouping) in groupings.iter().enumerate() {
            regex.push_str(format!("#{{{grouping}}}").as_str());

            if i != last_index {
                regex.push_str("\\.+");
            }
        }

        regex.push_str("\\.*$");

        // println!("regex: {regex}");

        let arrangements_pattern = match Regex::new(regex.as_str()) {
            Ok(regex) => regex,
            Err(err) => panic!("Error creating regex from {regex}: {err}"),
        };

        Row {
            springs,
            arrangements_pattern,
        }
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

    #[test]
    fn row_from_string() {
        let line = "???.### 1,1,3".to_string();

        let row = Row::from(line);

        assert_eq!(row.springs, "???.###");
        assert_eq!(
            row.arrangements_pattern.as_str(),
            r"^\.*#{1}\.+#{1}\.+#{3}\.*$"
        );
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
