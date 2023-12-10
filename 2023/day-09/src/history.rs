#[derive(Debug)]
pub struct History {
    values: Vec<i64>,
}

impl History {
    pub fn values(&self) -> &Vec<i64> {
        &self.values
    }

    pub fn iter(&self) -> HistoryIterator {
        HistoryIterator {
            history: self,
            index: 0,
        }
    }

    pub fn next_value(&self) -> i64 {
        find_next_value(self.values())
    }
}

fn find_next_value(values: &Vec<i64>) -> i64 {
    let last = values.last().unwrap().to_owned();
    let windows = values.windows(2).into_iter();
    let seq: Vec<i64> = windows.map(|window| window[1] - window[0]).collect();

    match seq.iter().all(|v| *v == 0) {
        true => last,
        false => last + find_next_value(&seq),
    }
}

pub struct HistoryIterator<'a> {
    history: &'a History,
    index: usize,
}

impl<'a> Iterator for HistoryIterator<'a> {
    type Item = &'a i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.history.values.len() {
            let result = Some(&self.history.values[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl From<&String> for History {
    fn from(values_str: &String) -> Self {
        let values: Vec<i64> = values_str
            .split_whitespace()
            .into_iter()
            .map(|value| value.parse::<i64>().unwrap())
            .collect();

        History { values }
    }
}

// impl From<&str> for History {
//     fn from(values_str: &str) -> Self {
//         History::from(&values_str.to_string())
//     }
// }

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::History;

    #[test]
    fn parse() {
        let s = "0 3 6 9 12 15";

        let history = History::from(&s.to_string());

        assert_eq!(history.values(), &vec![0, 3, 6, 9, 12, 15]);
    }

    #[rstest]
    #[case("0 3 6 9 12 15", 18)]
    #[case("1 3 6 10 15 21", 28)]
    #[case("10 13 16 21 30 45", 68)]
    fn next_value(#[case] values: &str, #[case] expected: i64) {
        let history = History::from(&values.to_string());
        assert_eq!(history.next_value(), expected);
    }
}
