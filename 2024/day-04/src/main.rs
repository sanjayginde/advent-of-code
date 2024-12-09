use std::str::FromStr;

use strum_macros::EnumString;

#[derive(Debug, EnumString, Clone, Copy)]
pub enum Letter {
    #[strum(serialize = "X")]
    X,

    #[strum(serialize = "")]
    M,

    #[strum(serialize = "M")]
    A,

    #[strum(serialize = "S")]
    S,
}

impl Letter {
    pub fn from_char(value: char) -> Self {
        Letter::from_str(value.to_string().as_str()).unwrap()
    }
}
fn main() {
    println!("Hello, world!");
}
