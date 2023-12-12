use std::collections::VecDeque;
use regex::Regex;

pub fn part1(input: &str) -> usize {
    return input.lines().map(|line| Card::from_str(line)).map(|card| card.score()).sum();
}

pub fn part2(input: &str) -> usize {
    let mut cards: VecDeque<(usize, Card)> = input.lines().map(|line| (1, Card::from_str(line))).collect();
    let mut sum: usize = 0;
    while let Some((copies, card)) = cards.pop_front() {
        cards.iter_mut()
            .take(card.matching_numbers_count())
            .for_each(|entry| entry.0 += copies);
        sum += copies;
    }
    return sum;
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    actual_numbers: Vec<usize>,
}

impl Card {
    fn from_str(input: &str) -> Card {
        let card_regex = Regex::new(r"^Card\W+(?<card_id>\d+): (?<winning_numbers>.*) \| (?<actual_numbers>.*)").expect("Invalid game match regex");
        let captures = card_regex.captures(input).expect("Invalid input");

        let card_id = captures["card_id"].parse::<usize>().unwrap();

        let winning_numbers = captures["winning_numbers"].split(' ').filter_map(|winning_number| winning_number.parse::<usize>().ok()).collect();
        let actual_numbers = captures["actual_numbers"].split(' ').filter_map(|number| number.parse::<usize>().ok()).collect();
        return Card { id: card_id, winning_numbers, actual_numbers };
    }
    fn matching_numbers_count(self: &Card) -> usize {
        self.actual_numbers.iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }

    fn score(self: &Card) -> usize {
        let matching_count = self.matching_numbers_count();
        if matching_count == 0 { return 0; }
        let base: usize = 2;
        return base.pow((matching_count - 1) as u32);
    }
}

#[cfg(test)]
mod test_day04 {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../resources/example.txt")), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../resources/example.txt")), 30);
    }

    #[test]
    fn test_card_parser() {
        let card = Card::from_str("Card  3: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(card.id, 3);
        assert_eq!(card.winning_numbers, [41, 48, 83, 86, 17]);
        assert_eq!(card.actual_numbers, [83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(card.matching_numbers_count(), 4);
        assert_eq!(card.score(), 8);
    }
}