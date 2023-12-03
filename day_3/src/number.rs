#[derive(Debug)]
pub struct Symbol {
    pub character: char,
    pub line_number: usize,
    pub column_number: usize,
}

#[derive(Debug)]
pub struct Number {
    pub value: u32,
    pub adjacent_symbols: Vec<Symbol>,
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number {
            value,
            adjacent_symbols: vec![],
        }
    }
}
