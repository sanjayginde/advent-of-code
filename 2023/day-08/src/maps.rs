pub type Node = String;

#[derive(Debug, Clone, Hash)]
pub struct Route {
    pub node: Node,
    pub left: Node,
    pub right: Node,
    is_part1_end: bool,
    is_part2_start: bool,
    is_part2_end: bool,
}

impl Route {
    pub fn new(node: Node, left: Node, right: Node) -> Self {
        // let ch = node.clone().chars().nth(2).unwrap();
        let is_part1_end = node.clone() == "ZZZ";
        let is_part2_start = node.clone().ends_with("A");
        let is_part2_end = node.clone().ends_with("Z");

        Route {
            node,
            left,
            right,
            is_part1_end,
            is_part2_start,
            is_part2_end,
        }
    }

    pub fn is_part1_end(&self) -> bool {
        self.node == "ZZZ"
    }

    pub fn is_part2_start(&self) -> bool {
        self.is_part2_start
    }

    pub fn is_part2_end(&self) -> bool {
        self.is_part2_end
    }
}

impl From<&String> for Route {
    // ZZZ = (ZZZ, ZZZ)"
    fn from(value: &String) -> Self {
        Route::new(
            value[..3].to_string(),
            value[7..10].to_string(),
            value[12..15].to_string(),
        )
    }
}

impl From<String> for Route {
    // ZZZ = (ZZZ, ZZZ)"
    fn from(value: String) -> Self {
        Route::from(&value)
    }
}

#[cfg(test)]
mod test {
    use super::Route;

    #[test]
    fn route_from_string() {
        let rows = ["BHK = (GRP, RXF)"].map(String::from).to_vec();

        let route = Route::from(rows[0].clone());

        println!("{:?}", route);
        assert_eq!(route.node, "BHK");
        assert_eq!(route.left, "GRP");
        assert_eq!(route.right, "RXF");
    }

    #[test]
    fn part2_start_end() {
        let rows = ["HFA = (GRP, RXF)", "HBZ = (HDI, AIN)"]
            .map(String::from)
            .to_vec();

        let start = Route::from(rows[0].clone());

        assert_eq!(start.is_part2_start(), true);
        assert_eq!(start.is_part2_end(), false);

        let end = Route::from(rows[1].clone());
        assert_eq!(end.is_part2_start(), false);
        assert_eq!(end.is_part2_end(), true);
    }
}
