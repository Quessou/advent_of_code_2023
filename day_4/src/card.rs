use std::collections::HashSet;
use std::str::FromStr;

use crate::card_numbers::CardNumbers;
use crate::parse_utils::parse_list_of_ints;
use crate::winning_numbers::WinningNumbers;

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning_numbers: WinningNumbers,
    card_numbers: CardNumbers,
}

impl Card {
    pub fn get_winning_numbers_count(&self) -> u32 {
        let mut intersection: HashSet<&u32> = HashSet::from_iter(self.winning_numbers.0.iter());
        intersection = intersection
            .intersection(&HashSet::from_iter(self.card_numbers.0.iter()))
            .map(|m| *m)
            .collect();
        intersection.len() as u32
    }
    pub fn point_value(&self) -> u32 {
        let winning_numbers_count = self.get_winning_numbers_count();
        match winning_numbers_count {
            0 => 0,
            _ => (2 as u32).pow(winning_numbers_count),
        }
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut halves = s.split('|');
        let first_half = halves.next().unwrap();
        let mut first_half = first_half.split(':');
        let card_id: u32 = first_half
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let winning_numbers: Vec<u32> = parse_list_of_ints(first_half.next().unwrap(), ' ');
        let card_numbers: Vec<u32> = parse_list_of_ints(halves.next().unwrap().trim(), ' ');

        Ok(Card {
            id: card_id,
            winning_numbers: winning_numbers.into(),
            card_numbers: card_numbers.into(),
        })
    }
}
