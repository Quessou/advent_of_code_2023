use crate::number::{Number, Symbol};
use std::collections::HashMap;

fn find_next_number(s: &str, start_index: usize) -> Option<usize> {
    let next_char = s[start_index as usize..]
        .chars()
        .enumerate()
        .find(|c| c.1.is_digit(10));
    match next_char {
        Some((i, _)) => Some(start_index + i),
        None => None,
    }
}

fn find_end_of_number(s: &str, start_index: usize) -> usize {
    match s[start_index..]
        .chars()
        .enumerate()
        .find(|(_, c)| !c.is_digit(10))
    {
        Some((i, _)) => start_index + i,
        None => s.len(),
    }
}

pub fn get_adjacent_symbols(
    current_line: &str,
    previous_line: &Option<&str>,
    next_line: &Option<&str>,
    start_number_index: usize,
    end_of_number_index: usize,
    current_line_index: usize,
) -> Vec<Symbol> {
    let mut symbols = vec![];
    let current_line_bytes = current_line.as_bytes();
    if start_number_index != 0 && current_line_bytes[start_number_index - 1] != '.' as u8 {
        symbols.push(Symbol {
            character: current_line_bytes[start_number_index - 1] as char,
            line_number: current_line_index,
            column_number: start_number_index - 1,
        })
    }
    if end_of_number_index != current_line.len()
        && current_line_bytes[end_of_number_index] != '.' as u8
    {
        symbols.push(Symbol {
            character: current_line_bytes[end_of_number_index] as char,
            line_number: current_line_index,
            column_number: end_of_number_index,
        })
    }
    let start_index = if start_number_index == 0 {
        0
    } else {
        start_number_index - 1
    };
    let end_index = if end_of_number_index == current_line.len() {
        end_of_number_index
    } else {
        end_of_number_index + 1
    };
    if let Some(prev) = previous_line {
        prev[start_index..end_index]
            .chars()
            .enumerate()
            .filter(|(_, c)| !c.is_ascii_digit() && *c != '.')
            .for_each(|(i, c)| {
                symbols.push(Symbol {
                    character: c,
                    line_number: current_line_index - 1,
                    column_number: start_index + i,
                })
            })
    }
    if let Some(next) = next_line {
        next[start_index..end_index]
            .chars()
            .enumerate()
            .filter(|(_, c)| !c.is_ascii_digit() && *c != '.')
            .for_each(|(i, c)| {
                symbols.push(Symbol {
                    character: c,
                    line_number: current_line_index + 1,
                    column_number: start_index + i,
                })
            })
    }
    symbols
}

pub fn handle_line(
    current_line: &str,
    previous_line: &Option<&str>,
    next_line: &Option<&str>,
    current_line_index: usize,
) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];
    let mut start_index: usize = 0;
    while let Some(next_number_index) = find_next_number(current_line, start_index) {
        let end_of_number_index = find_end_of_number(current_line, next_number_index);
        let number = &current_line[next_number_index..end_of_number_index];

        let number = number.parse::<u32>().unwrap().into();
        let adjacent_symbols: Vec<Symbol> = get_adjacent_symbols(
            current_line,
            previous_line,
            next_line,
            next_number_index,
            end_of_number_index,
            current_line_index,
        );
        let number: Number = Number {
            value: number,
            adjacent_symbols,
        };
        numbers.push(number);

        start_index = end_of_number_index;
    }

    numbers
}

pub fn group_by_gear(numbers: Vec<&Number>) -> HashMap<(usize, usize), Vec<&Number>> {
    let mut gears: HashMap<(usize, usize), Vec<&Number>> = HashMap::default();
    numbers.iter().for_each(|number| {
        number
            .adjacent_symbols
            .iter()
            .filter(|symbol| symbol.character == '*')
            .for_each(|symbol| {
                let arr = gears
                    .entry((symbol.line_number, symbol.column_number))
                    .or_insert(vec![]);
                arr.push(number);
            });
    });

    gears
}
