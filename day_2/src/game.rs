use std::{collections::HashMap, iter::Chain, str::FromStr};

use crate::{cube_color::CubeColor, round::Round};

#[derive(Default)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

impl Game {
    pub fn is_game_possible(&self, requirements: &HashMap<CubeColor, u32>) -> bool {
        return self.rounds.iter().all(|r| r.is_possible(&requirements));
    }
    pub fn smallest_acceptable_cube_set(&self) -> HashMap<CubeColor, u32> {
        let mut result = HashMap::new();
        result.insert(CubeColor::Blue, 0);
        result.insert(CubeColor::Red, 0);
        result.insert(CubeColor::Green, 0);
        let flat = self.rounds.iter().flat_map(|r| r.result.iter());
        flat.for_each(|(color, count)| {
            result.insert(
                *color,
                *std::cmp::max::<&u32>(count, result.get(color).unwrap()),
            );
        });

        result
    }

    pub fn compute_power(&self) -> u32 {
        let smallest_possible_set = self.smallest_acceptable_cube_set();
        smallest_possible_set
            .iter()
            .fold(1, |acc, (_, mul)| acc * mul)
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut split = line.trim().split(':');

        let game_id: u32 = split
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let rounds = split
            .next()
            .unwrap()
            .split(";")
            .map(|s| Round::from_str(s).unwrap())
            .collect();
        Ok(Game {
            id: game_id,
            rounds,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_game() {
        let line = "Game 17: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = Game::from_str(line);
        assert!(game.is_ok());
        assert_eq!(game.as_ref().unwrap().id, 17);
        assert_eq!(game.unwrap().rounds.len(), 3);
    }
}
