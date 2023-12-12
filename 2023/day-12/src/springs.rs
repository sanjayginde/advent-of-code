use regex::Regex;

#[derive(Debug, Clone)]
pub struct Row {
    springs: String,
    damaged_groupings: Vec<usize>,
    arrangements_pattern: Regex,
}

impl Row {
    pub fn new(springs: String, damaged_groupings: Vec<usize>) -> Self {
        let last_index = damaged_groupings.len() - 1;
        let mut regex = "^[^#]*".to_string();
        for (i, grouping) in damaged_groupings.iter().enumerate() {
            regex.push_str(format!("[^.]{{{grouping}}}").as_str());

            if i != last_index {
                regex.push_str("[^#]+");
            }
        }

        regex.push_str("[^#]*$");

        // println!("regex: {regex}");

        let arrangements_pattern: Regex = match Regex::new(regex.as_str()) {
            Ok(regex) => regex,
            Err(err) => panic!("Error creating regex from {regex}: {err}"),
        };

        Row {
            springs,
            damaged_groupings,
            arrangements_pattern,
        }
    }

    pub fn springs(&self) -> &String {
        &self.springs
    }

    pub fn damaged_groupings(&self) -> &Vec<usize> {
        &self.damaged_groupings
    }

    pub fn calc_arrangements(&self) -> usize {
        let combinations = self.generate_combinations(String::new(), self.springs.clone());

        // println!("# combos: {}", combinations.len());
        // println!("combos: {:?}", combinations);

        combinations.len()
    }

    fn generate_combinations(&self, lhs: String, rhs: String) -> Vec<String> {
        if !self
            .arrangements_pattern
            .is_match(format!("{lhs}{rhs}").as_str())
        {
            return vec![];
        }

        match rhs.split_once("?") {
            None => {
                vec![format!("{lhs}{rhs}")]
            }
            Some((l, r)) => {
                let mut result: Vec<String> = Vec::new();
                for combo in self
                    .generate_combinations(format!("{lhs}{l}"), format!("#{r}"))
                    .into_iter()
                {
                    result.push(combo);
                }

                for combo in self
                    .generate_combinations(format!("{lhs}{l}"), format!(".{r}"))
                    .into_iter()
                {
                    result.push(combo);
                }

                result
            }
        }
    }

    pub fn convert_to_part2_row(&self) -> Row {
        let mut springs = String::with_capacity(self.springs().len() * 5);
        let mut damaged_groupings: Vec<usize> =
            Vec::with_capacity(self.damaged_groupings().len() * 5);
        for _ in (0..4).into_iter() {
            springs.push_str(self.springs().as_str());
            springs.push('?');

            damaged_groupings.append(&mut self.damaged_groupings().clone())
        }

        springs.push_str(self.springs().as_str());
        damaged_groupings.append(&mut self.damaged_groupings().clone());

        Row::new(springs, damaged_groupings)
    }
}

impl From<String> for Row {
    fn from(value: String) -> Self {
        let splits: Vec<_> = value.split_whitespace().collect();

        let springs = splits[0].to_string();
        let groupings: Vec<&str> = splits[1].split(",").collect();

        let damaged_groupings: Vec<usize> = groupings
            .iter()
            .map(|grouping| grouping.parse().unwrap())
            .collect();

        Row::new(springs, damaged_groupings)
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

        assert_eq!(row.springs(), "???.###");
        assert_eq!(row.damaged_groupings(), &vec![1, 1, 3]);
        assert_eq!(
            row.arrangements_pattern.as_str(),
            r"^[^#]*[^.]{1}[^#]+[^.]{1}[^#]+[^.]{3}[^#]*$"
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

    #[test]
    fn convert_to_part2_row() {
        let line = "???.### 1,1,3".to_string();
        let row = Row::from(line).convert_to_part2_row();

        assert_eq!(row.springs(), "???.###????.###????.###????.###????.###");
        assert_eq!(
            row.damaged_groupings(),
            &vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
        );
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    fn calc_arrangements_part2(#[case] row: &str, #[case] expected: usize) {
        let row = Row::from(row).convert_to_part2_row();

        assert_eq!(row.calc_arrangements(), expected);
    }
}
