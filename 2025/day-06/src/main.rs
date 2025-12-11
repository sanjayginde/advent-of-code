use rust_aoc_utils::{
    parse_to_char_grid, parse_to_whitespaced_grid, read_lines_from_file, transpose,
};

#[derive(Debug, Clone)]
enum Element {
    Plus,
    Times,
    Number(usize),
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    size: usize,
}

impl Operation {
    pub fn calculate(&self, values: &[usize]) -> usize {
        self.operator.calculate(values)
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Times,
}

impl Operator {
    fn start_value(&self) -> usize {
        match self {
            Operator::Plus => 0,
            Operator::Times => 1,
        }
    }

    pub fn handle(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Operator::Plus => lhs + rhs,
            Operator::Times => lhs * rhs,
        }
    }

    pub fn calculate(&self, values: &[usize]) -> usize {
        values
            .iter()
            .fold(self.start_value(), |acc, value| self.handle(acc, *value))
    }
}

impl From<&str> for Element {
    fn from(s: &str) -> Self {
        match s {
            "+" => Element::Plus,
            "*" => Element::Times,
            _ => Element::Number(s.parse::<usize>().expect("Expected number")),
        }
    }
}

fn part1(lines: Vec<String>) -> usize {
    let grid = transpose(parse_to_whitespaced_grid(&lines, |s| Element::from(s)));

    let mut result = 0;
    for row in grid.iter() {
        let mut iter = row.iter().rev();
        let operator = iter.next().unwrap();
        result += match operator {
            Element::Plus => iter.fold(0, |acc, el| match el {
                Element::Number(num) => acc + num,
                _ => {
                    unreachable!("Expected number, but found {:?}", el)
                }
            }),
            Element::Times => iter.fold(1, |acc, el| match el {
                Element::Number(num) => acc * num,
                _ => {
                    unreachable!("Expected number, but found {:?}", el)
                }
            }),
            Element::Number(num) => {
                unreachable!("Expected operator, but found number {}", num);
            }
        };
    }

    result
}

fn part2(lines: Vec<String>) -> usize {
    let mut reverse = lines.to_vec();
    reverse.reverse();
    let mut iter = reverse.into_iter();

    let ops_str = iter.next().unwrap();

    let mut operations = vec![];
    let mut current_size = 0;
    let mut current_operator: Option<Operator> = None;
    for char in ops_str.chars() {
        match char {
            '+' | '*' => {
                if let Some(operator) = current_operator {
                    operations.push(Operation {
                        operator,
                        size: current_size,
                    });
                    current_size = 0;
                }

                current_operator = Some(match char {
                    '+' => Operator::Plus,
                    '*' => Operator::Times,
                    _ => unreachable!(),
                });
            }
            _ => {
                current_size += 1;
            }
        }
    }

    // Capture last Operation
    operations.push(Operation {
        operator: current_operator.expect("Missing final operator"),
        size: current_size + 1,
    });

    let values = transpose(parse_to_char_grid(&iter.collect::<Vec<_>>()))
        .into_iter()
        .filter_map(|mut row| {
            row.reverse();
            let s: String = row.into_iter().collect();
            s.trim().parse::<usize>().ok()
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    let mut values_iter = values.into_iter();
    for operation in operations.into_iter() {
        let mut op_values = vec![];
        for _ in 0..operation.size {
            op_values.push(values_iter.next().unwrap());
        }

        result += operation.calculate(&op_values);
    }

    result
}

// NOTE: input.txt needs to be 'right padded' to create a uniform grid!
fn main() {
    println!(
        "Solution for part 1 is {}",
        part1(read_lines_from_file("input.txt"))
    );
    println!(
        "Solution for part 2 is {}",
        part2(read_lines_from_file("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::part1;
    use super::part2;

    const EXAMPLE: [&str; 4] = [
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  ",
    ];

    const EXAMPLE_2: [&str; 5] = [
        "95 92 45 63      1 78 885",
        "41 29 61 65     99 57 924",
        " 1 4  22 9416  987  3 134",
        " 4 3  68 8629 4961  5 737",
        "*  *  *  +    +    *  +  ",
    ];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 4277556);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 3263827);
    }

    #[test]
    fn solve_example2_part2() {
        assert_eq!(part2(EXAMPLE_2.map(String::from).to_vec()), 25161998);
    }
}
