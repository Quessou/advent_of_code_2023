use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum CubeColor {
    Red,
    Green,
    Blue,
}

impl FromStr for CubeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(CubeColor::Blue),
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            _ => unreachable!(),
        }
    }
}
