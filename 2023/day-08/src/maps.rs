pub type Node = String;

#[derive(Debug, Clone, Hash)]
pub struct Route {
    pub node: Node,
    pub left: Node,
    pub right: Node,
}

impl Route {
    pub fn is_end(&self) -> bool {
        self.node == "ZZZ"
    }
}

impl From<&String> for Route {
    // ZZZ = (ZZZ, ZZZ)"
    fn from(value: &String) -> Self {
        Route {
            node: value[..3].to_string(),
            left: value[7..10].to_string(),
            right: value[12..15].to_string(),
        }
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
}
