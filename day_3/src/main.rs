mod read_file;

use std::path::Path;

use read_file::read_to_lines;

mod part_1;
use part_1::*;

use crate::part_2::group_by_gear;
mod gear;
mod number;
mod part_2;

use number::Number;

fn main() {
    let lines = read_to_lines(Path::new("./input.txt"));
    let mut previous_line: Option<&str> = None;
    let mut it = lines.iter().enumerate().peekable();
    let mut numbers: Vec<Number> = vec![];
    while let Some(line) = it.next() {
        numbers.append(&mut part_2::handle_line(
            line.1,
            &previous_line,
            &it.peek().map(|x| x.1.as_str()),
            line.0,
        ));
        previous_line = Some(line.1);
    }
    let numbers_ref: Vec<&Number> = numbers.iter().map(|n| n).collect();
    let sum = numbers_ref.iter().fold(0, |sum, n| sum + n.value);
    println!("{}", sum);
    let gears = group_by_gear(numbers_ref);
    let valid_gears: Vec<Vec<&Number>> = gears
        .into_iter()
        .filter(|((_, _), arr)| arr.len() == 2)
        .map(|(_, arr)| arr)
        .collect();
    let result = valid_gears.iter().fold(0, |acc, numbers| {
        acc + numbers.iter().fold(1, |sum, n| sum * n.value)
    });

    println!("{}", result);
}
