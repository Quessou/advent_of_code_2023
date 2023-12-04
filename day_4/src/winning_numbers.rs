#[derive(Debug)]
pub struct WinningNumbers(pub Vec<u32>);

impl From<Vec<u32>> for WinningNumbers {
    fn from(value: Vec<u32>) -> Self {
        WinningNumbers { 0: value }
    }
}
