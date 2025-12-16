use rust_aoc_utils::read_lines_from_file;
use std::collections::HashMap;

fn part1(lines: Vec<String>) -> usize {
    let graph = parse(lines);
    find_path("you", &graph)
}

fn find_path(current: &str, graph: &HashMap<String, Vec<String>>) -> usize {
    if current == "out" {
        return 1;
    }

    graph
        .get(current)
        .unwrap_or_else(|| panic!("No ouputs found for {}", current))
        .iter()
        .map(|next| find_path(next, graph))
        .sum::<usize>()
}

fn part2(lines: Vec<String>) -> usize {
    let graph = parse(lines);
    let mut cache: HashMap<(String, bool, bool), usize> = HashMap::new();
    find_svr_path("svr".to_string(), false, false, &graph, &mut cache)
}

// #[memoize]
fn find_svr_path(
    current: String,
    dac: bool,
    fft: bool,
    graph: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    if current == "out" {
        let result = match dac && fft {
            true => 1,
            false => 0,
        };

        return result;
    }

    match graph.get(&current) {
        Some(nexts) => nexts
            .iter()
            .map(|next| {
                let visited_dac = dac || current == "dac";
                let visited_fft = fft || current == "fft";
                let key = (next.to_string(), visited_dac, visited_fft);
                match cache.get(&key) {
                    Some(value) => *value,

                    None => {
                        let value =
                            find_svr_path(next.to_string(), visited_dac, visited_fft, graph, cache);
                        cache.insert(key, value);
                        value
                    }
                }
            })
            .sum::<usize>(),
        None => {
            unreachable!("Reached end at {}", current)
        }
    }
}

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

// Utilities

fn parse(lines: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let key = parts[0].to_string();
        let values = parts[1].split(" ").map(String::from).collect();
        map.insert(key, values);
    }
    map
}

#[cfg(test)]
mod test {
    use super::part1;
    use super::part2;

    const EXAMPLE_1: [&str; 10] = [
        "aaa: you hhh",
        "you: bbb ccc",
        "bbb: ddd eee",
        "ccc: ddd eee fff",
        "ddd: ggg",
        "eee: out",
        "fff: out",
        "ggg: out",
        "hhh: ccc fff iii",
        "iii: out",
    ];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE_1.map(String::from).to_vec()), 5);
    }

    const EXAMPLE_2: [&str; 13] = [
        "svr: aaa bbb",
        "aaa: fft",
        "fft: ccc",
        "bbb: tty",
        "tty: ccc",
        "ccc: ddd eee",
        "ddd: hub",
        "hub: fff",
        "eee: dac",
        "dac: fff",
        "fff: ggg hhh",
        "ggg: out",
        "hhh: out",
    ];

    #[test]
    fn solve_example_part2() {
        assert_eq!(part2(EXAMPLE_2.map(String::from).to_vec()), 2);
    }
}
