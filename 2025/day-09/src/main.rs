use std::fmt::Display;

use itertools::Itertools;
use rust_aoc_utils::read_lines_from_file;

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

impl From<&String> for Point {
    fn from(s: &String) -> Self {
        let parts = s
            .split(",")
            .map(|part| part.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Point::new(parts[0], parts[1])
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn tile_area(a: &Point, b: &Point) -> usize {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

#[derive(Debug, Clone)]
struct Rectangle {
    a: Point,
    b: Point,
    area: usize,
}

#[derive(Clone, Debug)]
struct Edge {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn new(a: Point, b: Point) -> Self {
        let area = tile_area(&a, &b);
        Rectangle { a, b, area }
    }
}

impl From<Vec<&Point>> for Rectangle {
    fn from(tiles: Vec<&Point>) -> Self {
        assert!(
            tiles.len() == 2,
            "Can only create a Rectangle from two Tiles"
        );

        Rectangle::new(tiles[0].clone(), tiles[1].clone())
    }
}

impl Edge {
    fn new(p1: Point, p2: Point) -> Self {
        Edge { p1, p2 }
    }
}

fn part1(lines: Vec<String>) -> usize {
    let mut result = 0;
    let tiles = lines.iter().map(Point::from).collect::<Vec<_>>();
    let combos = tiles.iter().combinations(2);

    for combo in combos {
        let a = combo[0];
        let b = combo[1];

        let area = tile_area(a, b);
        if area > result {
            result = area;
        }
    }

    result
}

fn part2(lines: Vec<String>) -> usize {
    let tiles = lines.iter().map(Point::from).collect::<Vec<_>>(); // Ordered Tiles is effectively a Polygon
    let rectangles: Vec<Rectangle> = tiles
        .iter()
        .combinations(2)
        .map(|v| Rectangle::from(v.clone()))
        .sorted_by(|a, b| b.area.cmp(&a.area))
        .collect();

    let mut edges: Vec<Edge> = vec![];
    for i in 0..tiles.len() {
        let p1 = tiles[i].clone();
        let p2 = tiles[(i + 1) % tiles.len()].clone();

        edges.push(Edge::new(p1, p2))
    }

    let result = rectangles.iter().find(|rect| {
        edges.iter().all(|edge| {
            disjoint(edge.p1.y, edge.p2.y, rect.a.y, rect.b.y)
                || disjoint(edge.p1.x, edge.p2.x, rect.a.x, rect.b.x)
        })
    });

    match result {
        Some(r) => r.area,
        None => 0,
    }
}

// Checks if there is overlap between edges.
fn disjoint(a1: usize, a2: usize, b1: usize, b2: usize) -> bool {
    a1.max(a2) <= b1.min(b2) || b1.max(b2) <= a1.min(a2)
}

fn main() {
    println!("Part 1: {}", part1(read_lines_from_file("input.txt")));
    println!("Part 2: {}", part2(read_lines_from_file("input.txt")));
}

// Utilities

#[cfg(test)]
mod test {
    use super::part1;
    use super::part2;

    const EXAMPLE: [&str; 8] = ["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 50);
    }

    #[test]
    fn _solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 24);
    }
}
