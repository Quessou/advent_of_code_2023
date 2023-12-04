#[derive(Debug)]
pub struct CardNumbers(pub Vec<u32>);

impl From<Vec<u32>> for CardNumbers {
    fn from(value: Vec<u32>) -> Self {
        CardNumbers { 0: value }
    }
}
