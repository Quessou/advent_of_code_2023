use std::io::{BufReader, Read};
use std::path::Path;

pub fn read_to_lines(path: &Path) -> Vec<String> {
    let file = std::fs::OpenOptions::new().read(true).open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_to_string(&mut data).unwrap();

    let mut lines: Vec<String> = data.split("\n").map(|s| s.to_owned()).collect();
    lines.pop();
    lines
}
