use std::collections::HashMap;
use std::str::FromStr;

use crate::cube_color::CubeColor;

#[derive(Default)]
pub struct Round {
    pub result: HashMap<CubeColor, u32>,
}

impl Round {
    pub fn is_possible(&self, requirements: &HashMap<CubeColor, u32>) -> bool {
        return self
            .result
            .iter()
            .all(|(color, count)| requirements[color] >= *count);
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut round = Round::default();
        line.split(',')
            .map(|s| {
                let mut it = s.trim().split(' ');
                let count = it.next().unwrap();
                let count = count.trim().parse::<u32>().unwrap();
                let color = CubeColor::from_str(it.next().unwrap()).unwrap();
                (color, count)
            })
            .for_each(|(color, count)| {
                round.result.insert(color, count);
            });
        Ok(round)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserialize_from_str() {
        let line = "3 blue, 4 red, 2 green";
        let round = Round::from_str(line);
        assert!(round.is_ok());
        let round = round.unwrap();
        assert_eq!(round.result[&CubeColor::Red], 4);
        assert_eq!(round.result[&CubeColor::Blue], 3);
        assert_eq!(round.result[&CubeColor::Green], 2);
    }

    #[test]
    fn test_round_possible_nominal_case() {
        let mut result: HashMap<CubeColor, u32> = HashMap::new();
        result.insert(CubeColor::Green, 2);
        result.insert(CubeColor::Red, 3);
        result.insert(CubeColor::Blue, 4);
        let round = Round { result };

        let mut feasible: HashMap<CubeColor, u32> = HashMap::new();
        feasible.insert(CubeColor::Green, 5);
        feasible.insert(CubeColor::Red, 5);
        feasible.insert(CubeColor::Blue, 5);

        assert!(round.is_possible(&feasible));

        let mut unfeasible: HashMap<CubeColor, u32> = HashMap::new();
        unfeasible.insert(CubeColor::Green, 1);
        unfeasible.insert(CubeColor::Red, 5);
        unfeasible.insert(CubeColor::Blue, 5);
        assert!(!round.is_possible(&unfeasible));
    }

    #[test]
    fn test_round_possible_missing_color_case() {
        let mut result: HashMap<CubeColor, u32> = HashMap::new();
        result.insert(CubeColor::Red, 3);
        result.insert(CubeColor::Blue, 4);
        let round = Round { result };

        let mut feasible: HashMap<CubeColor, u32> = HashMap::new();
        feasible.insert(CubeColor::Green, 5);
        feasible.insert(CubeColor::Red, 5);
        feasible.insert(CubeColor::Blue, 5);

        assert!(round.is_possible(&feasible));

        let mut result: HashMap<CubeColor, u32> = HashMap::new();
        result.insert(CubeColor::Red, 3);
        let round = Round { result };

        assert!(round.is_possible(&feasible));

        let mut unfeasible: HashMap<CubeColor, u32> = HashMap::new();
        unfeasible.insert(CubeColor::Green, 5);
        unfeasible.insert(CubeColor::Red, 1);
        unfeasible.insert(CubeColor::Blue, 5);
        assert!(!round.is_possible(&unfeasible));
    }
}
