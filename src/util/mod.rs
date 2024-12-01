pub mod input;

use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Part {
    One,
    Two,
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Part::One => write!(f, "One"),
            Part::Two => write!(f, "Two"),
        }
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "1" | "one" | "first" => Ok(Part::One),
            "2" | "two" | "second" => Ok(Part::Two),
            _ => Err(anyhow::anyhow!("Invalid part {}", s)),
        }
    }
}
