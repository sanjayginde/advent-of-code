use std::fs::read_to_string;

const CAPACITY: usize = 256;

fn parse(lines: Vec<String>) -> Vec<String> {
    match lines.get(0) {
        Some(line) => {
            let steps: Vec<&str> = line.split(",").collect();
            steps.into_iter().map(String::from).collect()
        }
        None => {
            panic!("No input data")
        }
    }
}

fn hash(value: &String) -> usize {
    let mut result: usize = 0;
    for ch in value.chars().into_iter() {
        let code = ch as usize;

        // Increase the current value by the ASCII code you just determined.
        result += code;

        // Set the current value to itself multiplied by 17.
        result *= 17;

        // Set the current value to the remainder of dividing itself by 256.
        result %= CAPACITY;
    }

    result
}

fn solve_part1(steps: &Vec<String>) -> usize {
    let mut total: usize = 0;

    for step in steps.into_iter() {
        total += hash(&step);
    }

    total
}

struct Lens {
    label: String,
    focal_length: usize,
}

impl Lens {
    pub fn get_hash(&self) -> usize {
        hash(&self.label)
    }
}

impl From<&String> for Lens {
    fn from(value: &String) -> Self {
        let splits: Vec<_> = value.split("=").collect();

        let label: String = splits[0].to_string();
        let focal_length: usize = splits[1].parse::<usize>().unwrap();

        Lens {
            label,
            focal_length,
        }
    }
}

fn solve_part2(steps: &Vec<String>) -> usize {
    let mut boxes: Vec<Vec<Lens>> = Vec::with_capacity(CAPACITY);
    (0..CAPACITY).for_each(|_| {
        boxes.push(Vec::new());
    });

    for step in steps {
        match step.contains("=") {
            true => {
                let lens = Lens::from(step);
                let lenses: &mut Vec<Lens> = &mut boxes[lens.get_hash()];
                match lenses.iter_mut().find(|item| item.label == lens.label) {
                    Some(item) => item.focal_length = lens.focal_length,
                    None => lenses.push(lens),
                }
            }
            false => {
                let split: Vec<&str> = step.split("-").collect();
                let label = split[0].to_string();
                let lenses: &mut Vec<Lens> = &mut boxes[hash(&label)];
                lenses.retain(|lens| lens.label != label)
            }
        }
    }

    let mut total: usize = 0;
    for (pos, lenses) in boxes.into_iter().enumerate() {
        let box_num = pos + 1;
        for (lens_pos, lens) in lenses.into_iter().enumerate() {
            total += (box_num) * (lens_pos + 1) * lens.focal_length;
        }
    }

    total
}

fn main() {
    let steps: Vec<String> = parse(read_lines("input.txt"));

    println!("Solution for part 1 is {}", solve_part1(&steps));
    println!("Solution for part 2 is {}", solve_part2(&steps));
}

#[cfg(test)]
mod test {

    use super::{parse, solve_part1, solve_part2};

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn solve_example_part1() {
        let steps: Vec<String> = parse(vec![EXAMPLE.to_string()]);
        assert_eq!(solve_part1(&steps), 1320);
    }

    #[test]
    fn solve_example_part2() {
        let steps: Vec<String> = parse(vec![EXAMPLE.to_string()]);
        assert_eq!(solve_part2(&steps), 145);
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
