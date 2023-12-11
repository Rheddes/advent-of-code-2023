use std::collections::BTreeMap;
use regex::Regex;

pub fn part1(input: &str) -> usize {
    let cubes_in_bag = BTreeMap::from([
        (Color::Red, 12),
        (Color::Green, 13),
        (Color::Blue, 14),
    ]);

    return input
        .lines()
        .map(Game::from_str)
        .filter(|game| game.is_valid(&cubes_in_bag))
        .map(|game| game.id)
        .sum();
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Vec<CubeObservation>>,
}

impl Game {
    pub fn is_valid(&self, cubes: &BTreeMap<Color, usize> ) -> bool {
        self.rounds.iter().all(|round| {
            round.iter().all(|observation| {
                cubes[&observation.color] >= observation.amount
            })
        })
    }

    pub fn from_str(input: &str) -> Game {
        let game_id_regex = Regex::new(r"^Game (?<game_id>\d+):(?<rounds>.*)").expect("Invalid game match regex");

        let captures = game_id_regex.captures(input).expect("Invalid input");
        let game_id = captures["game_id"].parse::<usize>().unwrap();
        let rounds = &captures["rounds"];
        let rounds = rounds
            .split(';')
            .map(|round| round
                .split(',')
                .map(|observation| CubeObservation::from_str(observation))
                .collect())
            .collect();

        return Game {
            id: game_id,
            rounds: rounds,
        };
    }
}

#[derive(Debug)]
struct CubeObservation {
    color: Color,
    amount: usize,
}

impl CubeObservation {
    pub fn from_str(input: &str) -> CubeObservation {
        let [amount, color] = input.trim().split(' ').collect::<Vec<&str>>()[..] else { panic!() };
        return CubeObservation {
            color: Color::from_str(color),
            amount: amount.parse::<usize>().unwrap(),
        };
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn from_str(input: &str) -> Color {
        return match input {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Invalid color"),
        };
    }
}

#[cfg(test)]
mod test_day02 {
    use super::*;

    #[test]
    fn test_part_1() {
        let example_input = include_str!("../resources/example.txt");
        assert_eq!(part1(example_input), 8);
    }
}