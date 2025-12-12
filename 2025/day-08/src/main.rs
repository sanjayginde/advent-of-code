use std::rc::Rc;
use std::str::FromStr;

use itertools::Itertools;
use rust_aoc_utils::read_lines_from_file;

#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    circuit: Option<Rc<Circuit>>,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        Ok(Point {
            x,
            y,
            z,
            circuit: None,
        })
    }
}

impl From<&String> for Point {
    fn from(value: &String) -> Self {
        Point::from_str(value).unwrap()
    }
}

#[derive(Debug, Clone)]
struct Edge {
    a_index: usize,
    b_index: usize,
    distance: isize,
}

impl Edge {
    fn new(a_index: usize, b_index: usize, points: &[Point]) -> Self {
        let a = &points[a_index];
        let b = &points[b_index];
        Edge {
            a_index,
            b_index,
            distance: ((b.x - a.x).pow(2) + (b.y - a.y).pow(2) + (b.z - a.z).pow(2)).isqrt(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Circuit {}

/// A wrapper around points and edges that ensures index stability.
/// Once created, the points cannot be reordered, but their data can be modified.
struct PointGraph {
    points: Vec<Point>,
    edges: Vec<Edge>,
}

impl PointGraph {
    fn new(points: Vec<Point>) -> Self {
        let edges = Self::create_edges(&points);
        PointGraph { points, edges }
    }

    fn create_edges(points: &[Point]) -> Vec<Edge> {
        let mut edges = (0..points.len())
            .tuple_combinations()
            .map(|(i, j)| Edge::new(i, j, points))
            .collect::<Vec<_>>();

        edges.sort_by_key(|e| e.distance);
        edges
    }

    fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// Returns a map of circuit pointer to point count
    pub fn circuit_counts(&self) -> std::collections::HashMap<*const Circuit, usize> {
        let mut circuit_counts = std::collections::HashMap::new();
        for point in &self.points {
            if let Some(ref circuit) = point.circuit {
                *circuit_counts.entry(Rc::as_ptr(circuit)).or_insert(0) += 1;
            }
        }
        circuit_counts
    }

    /// Returns the sizes of all circuits, sorted largest first
    pub fn circuit_sizes(&self) -> Vec<usize> {
        let mut counts: Vec<usize> = self.circuit_counts().values().copied().collect();
        counts.sort_by(|a, b| b.cmp(a));
        counts
    }

    /// Returns the total number of points that have circuits
    #[allow(dead_code)]
    pub fn connected_points_count(&self) -> usize {
        self.points.iter().filter(|p| p.circuit.is_some()).count()
    }

    pub fn connect_circuits(&mut self, limit: usize) {
        let edges_to_process: Vec<Edge> = self.edges().iter().take(limit).cloned().collect();

        for edge in edges_to_process {
            self.connect_circuit(&edge);
        }
    }

    /// Connects two points via a circuit using their indices.
    /// This method safely modifies the points while maintaining index validity.
    fn connect_circuit(&mut self, edge: &Edge) {
        // Check conditions first without borrowing mutably
        let a_has_circuit = self.points[edge.a_index].circuit.is_some();
        let b_has_circuit = self.points[edge.b_index].circuit.is_some();

        if !a_has_circuit && !b_has_circuit {
            let circuit = Rc::new(Circuit {});
            self.points[edge.a_index].circuit = Some(circuit.clone());
            self.points[edge.b_index].circuit = Some(circuit);
        } else if !a_has_circuit || !b_has_circuit {
            let circuit = self.points[edge.a_index]
                .circuit
                .clone()
                .or_else(|| self.points[edge.b_index].circuit.clone())
                .unwrap();
            self.points[edge.a_index].circuit = Some(circuit.clone());
            self.points[edge.b_index].circuit = Some(circuit);
        } else {
            // Both have circuits - merge if they're different
            let a_circuit = self.points[edge.a_index].circuit.clone();
            let b_circuit = self.points[edge.b_index].circuit.clone();

            if let (Some(a_circ), Some(b_circ)) = (&a_circuit, &b_circuit)
                && !Rc::ptr_eq(a_circ, b_circ)
            {
                let target_circuit = a_circuit.unwrap();
                let old_circuit = b_circ.clone();

                // Replace all instances of old_circuit with target_circuit
                for point in self.points.iter_mut() {
                    if let Some(ref circuit) = point.circuit
                        && Rc::ptr_eq(circuit, &old_circuit)
                    {
                        point.circuit = Some(target_circuit.clone());
                    }
                }
            }
        }
    }

    fn connect_all(&mut self) -> (Point, Point) {
        let mut result = None;
        let mut edges_iter = self.edges().to_vec().into_iter();
        while !self.all_connected() {
            let edge = edges_iter.next();
            match edge {
                Some(ref e) => {
                    self.connect_circuit(e);
                }
                None => unreachable!("Exhausted all edges!"),
            }

            result = edge;
        }
        match result {
            Some(ref edge) => (
                self.points[edge.a_index].clone(),
                self.points[edge.b_index].clone(),
            ),
            None => unreachable!("Could not connect all points!"),
        }
    }

    fn all_connected(&self) -> bool {
        let circuit = self.points[0].circuit.clone();

        self.points.iter().all(|point| {
            if let (Some(lhs), Some(rhs)) = (&circuit, &point.circuit)
                && Rc::ptr_eq(lhs, rhs)
            {
                return true;
            }
            false
        })
    }
}

fn part1(points: Vec<Point>, limit: usize) -> usize {
    let mut graph = PointGraph::new(points);

    // Process edges safely - indices are guaranteed to remain valid
    graph.connect_circuits(limit);

    let circuit_sizes = graph.circuit_sizes();
    circuit_sizes.iter().take(3).product()
}

fn part2(points: Vec<Point>) -> isize {
    let mut graph = PointGraph::new(points);

    let (a, b) = graph.connect_all();

    a.x * b.x
}

fn main() {
    println!(
        "Solution for part 1 is {}",
        part1(parse_lines(read_lines_from_file("input.txt")), 1_000)
    );

    println!(
        "Solution for part 1 is {}",
        part2(parse_lines(read_lines_from_file("input.txt")))
    );
}

fn parse_lines(lines: Vec<String>) -> Vec<Point> {
    lines.iter().map(Point::from).collect()
}

#[cfg(test)]
mod test {
    use super::parse_lines;
    use super::part1;
    use super::part2;

    const EXAMPLE: [&str; 20] = [
        "162,817,812",
        "57,618,57",
        "906,360,560",
        "592,479,940",
        "352,342,300",
        "466,668,158",
        "542,29,236",
        "431,825,988",
        "739,650,466",
        "52,470,668",
        "216,146,977",
        "819,987,18",
        "117,168,530",
        "805,96,715",
        "346,949,466",
        "970,615,88",
        "941,993,340",
        "862,61,35",
        "984,92,344",
        "425,690,689",
    ];

    #[test]
    fn solve_example_part1() {
        assert_eq!(
            part1(parse_lines(EXAMPLE.map(String::from).to_vec()), 10),
            40
        );
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(
            part2(parse_lines(EXAMPLE.map(String::from).to_vec())),
            25272
        );
    }
}
