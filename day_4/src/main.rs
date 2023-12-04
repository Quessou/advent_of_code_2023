mod card;
mod card_numbers;
mod winning_numbers;

mod parse_utils;
mod read_file;

use std::path::Path;
use std::str::FromStr;

use card::Card;
use read_file::read_to_lines;

fn main() {
    let lines = read_to_lines(Path::new("./test_input.txt"));
    let mut cards_and_copy_count = lines
        .iter()
        .map(|l| (Card::from_str(l).unwrap(), 1 as u32))
        .collect::<Vec<(Card, u32)>>();

    let sum = cards_and_copy_count
        .iter()
        .fold(0, |acc, c| acc + c.0.point_value());

    let mut iter = cards_and_copy_count.iter();
    while let Some(data) = iter.next() {
        let winning_numbers_count = data.0.get_winning_numbers_count();
        let mut next_elements_iter = iter.clone();
        // Note : have to use data.1 to clone the cards the appropriate amount of time here
        for _ in 0..winning_numbers_count {
            let mut after: &(&mut Card, &mut u32) = next_elements_iter
                .next()
                .map(|(card, count)| (&mut card, &mut count))
                .as_mut()
                .unwrap();
            *after.1 = *after.1 + 1;
        }
    }
    /*
    .for_each(|(card, count)| {
        let winning_numbers_count = card.get_winning_numbers_count();
        for _ in 0..winning_numbers_count {
            let next =
        }
    });
    */

    println!("{}", sum);
}
