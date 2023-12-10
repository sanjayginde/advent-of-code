// pub type Node = [char; 3];

pub trait Node {
    fn new(value: [char; 3]) -> Self;

    fn value(&self) -> [char; 3];

    fn is_start(&self) -> bool;

    fn is_end(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Part1Node {
    value: [char; 3],
}

impl Node for Part1Node {
    fn new(value: [char; 3]) -> Self {
        Self { value }
    }

    fn value(&self) -> [char; 3] {
        self.value
    }

    fn is_start(&self) -> bool {
        self.value == START_NODE_PART_1.value()
    }

    fn is_end(&self) -> bool {
        self.value == ['Z', 'Z', 'Z']
    }
}

pub const START_NODE_PART_1: Part1Node = Part1Node {
    value: ['A', 'A', 'A'],
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Part2Node {
    value: [char; 3],
}

impl Node for Part2Node {
    fn new(value: [char; 3]) -> Self {
        Self { value }
    }

    fn value(&self) -> [char; 3] {
        self.value
    }

    fn is_start(&self) -> bool {
        self.value[2] == 'A'
    }

    fn is_end(&self) -> bool {
        self.value[2] == 'Z'
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Route<T: Node> {
    pub node: T,
    pub left: T,
    pub right: T,
}

impl<T: Node> Route<T> {
    pub fn new(node: T, left: T, right: T) -> Self {
        Route { node, left, right }
    }
}

impl<T: Node> From<&String> for Route<T> {
    // ZZZ = (ZZZ, ZZZ)"
    fn from(value: &String) -> Self {
        let mut node = value[..3].chars().into_iter();
        let mut left = value[7..10].chars();
        let mut right = value[12..15].chars();
        Route::new(
            T::new([
                node.next().unwrap(),
                node.next().unwrap(),
                node.next().unwrap(),
            ]),
            T::new([
                left.next().unwrap(),
                left.next().unwrap(),
                left.next().unwrap(),
            ]),
            T::new([
                right.next().unwrap(),
                right.next().unwrap(),
                right.next().unwrap(),
            ]),
        )
    }
}

impl<T: Node> From<String> for Route<T> {
    // ZZZ = (ZZZ, ZZZ)"
    fn from(value: String) -> Self {
        Route::from(&value)
    }
}

#[cfg(test)]
mod test {
    use super::{Node, Part1Node, Part2Node, Route};

    #[test]
    fn route_from_string() {
        let rows = ["BHK = (GRP, RXF)"].map(String::from).to_vec();

        let route: Route<Part1Node> = Route::from(rows[0].clone());

        println!("{:?}", route);
        assert_eq!(route.node.value(), ['B', 'H', 'K']);
        assert_eq!(route.left.value(), ['G', 'R', 'P']);
        assert_eq!(route.right.value(), ['R', 'X', 'F']);
    }

    #[test]
    fn part_2_node() {
        let rows = ["HFA = (GRP, RXF)", "HBZ = (HDI, AIN)"]
            .map(String::from)
            .to_vec();

        let start: Route<Part2Node> = Route::from(rows[0].clone());

        assert_eq!(start.node.is_start(), true);
        assert_eq!(start.node.is_end(), false);

        let end: Route<Part2Node> = Route::from(rows[1].clone());
        assert_eq!(end.node.is_start(), false);
        assert_eq!(end.node.is_end(), true);
    }
}
