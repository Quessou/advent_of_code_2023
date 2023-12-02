mod cube_color;
mod game;
mod read_file;
mod round;

use read_file::read_to_lines;

use game::Game;
use std::collections::HashMap;
use std::str::FromStr;

use std::path::Path;

use crate::cube_color::CubeColor;

fn main() {
    let lines = read_to_lines(Path::new("./input_part1.txt"));
    let games = lines
        .iter()
        .map(|s| Game::from_str(s).unwrap())
        .collect::<Vec<_>>();
    let mut requirements: HashMap<CubeColor, u32> = HashMap::new();
    requirements.insert(CubeColor::Green, 13);
    requirements.insert(CubeColor::Red, 12);
    requirements.insert(CubeColor::Blue, 14);
    let possible_games_sum_id = games.iter().fold(0, |sum, g| {
        if g.is_game_possible(&requirements) {
            sum + g.id
        } else {
            sum
        }
    });
    println!("{}", possible_games_sum_id);

    let powers = games.iter().fold(0, |sum, g| sum + g.compute_power());
    println!("{}", powers);
}
