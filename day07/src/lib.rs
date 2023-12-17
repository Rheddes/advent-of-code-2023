use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let hands: Vec<HandPart1> = input.lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(HandPart1::from)
        .collect();
    return hands.iter()
        .sorted_by_key(|h| h.score)
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum();
}

pub fn part2(input: &str) -> usize {
    let hands: Vec<HandPart2> = input.lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(HandPart2::from)
        .collect();
    return hands.iter()
        .sorted_by_key(|h| h.score)
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum();
}

#[derive(Debug)]
struct HandPart1 {
    bid: usize,
    score: (HandType, u8, u8, u8, u8, u8)
}
impl HandPart1 {
    fn cards_to_type(cards: &str) -> HandType {
        let card_counts = cards.chars().counts();
        let card_counts = card_counts.values().cloned().sorted().rev().collect::<Vec<usize>>();
        if card_counts.contains(&5) { return HandType::FiveOfAKind; }
        if card_counts.contains(&4) { return HandType::FourOfAKind; }
        if card_counts.contains(&3) && card_counts.contains(&2) { return HandType::FullHouse; }
        if card_counts.contains(&3) { return HandType::ThreeOfAKind; }
        if card_counts.contains(&2) {
            if card_counts.iter().filter(|count| **count == 2).count() == 2 {
                return HandType::TwoPair;
            }
            return HandType::OnePair;
        }
        return HandType::HighCard;
    }

    fn card_to_int(card: char) -> u8 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            value => value.to_digit(10).unwrap() as u8,
        }
    }

    pub fn from((hand, bid): (&str, &str)) -> HandPart1 {
        let hand_type = HandPart1::cards_to_type(hand);
        return HandPart1 {
            bid: bid.parse::<usize>().unwrap(),
            score: HandPart1::score(hand, hand_type),
        };
    }

    fn score(cards: &str, hand_type: HandType) -> (HandType, u8, u8, u8, u8, u8) {
        let cards: Vec<u8> = cards.chars().into_iter().map(HandPart1::card_to_int).collect();
        return (hand_type, cards[0], cards[1], cards[2], cards[3], cards[4]);
    }
}

#[derive(Debug)]
struct HandPart2 {
    bid: usize,
    score: (HandType, u8, u8, u8, u8, u8)
}
impl HandPart2 {
    fn cards_to_type(cards: &str) -> HandType {
        let card_counts = cards.chars().counts();
        let jokers = card_counts.get(&'J').unwrap_or(&0).clone();
        let card_counts = card_counts.into_iter()
            .filter(|(c, _)| *c != 'J')
            .map(|(_, v)| v)
            .sorted()
            .rev()
            .collect::<Vec<usize>>();
        let (biggest, second_biggest) = (*card_counts.get(0).unwrap_or(&0) + jokers, *card_counts.get(1).unwrap_or(&0));
        if biggest == 5 { return HandType::FiveOfAKind; }
        if biggest == 4{ return HandType::FourOfAKind; }
        if biggest == 3 && second_biggest == 2 { return HandType::FullHouse; }
        if biggest == 3 { return HandType::ThreeOfAKind; }
        if biggest == 2 && second_biggest == 2 { return HandType::TwoPair; }
        if biggest == 2 { return HandType::OnePair; }
        return HandType::HighCard;
    }

    fn card_to_int(card: char) -> u8 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            value => value.to_digit(10).unwrap() as u8,
        }
    }

    pub fn from((hand, bid): (&str, &str)) -> HandPart2 {
        let hand_type = HandPart2::cards_to_type(hand);
        return HandPart2 {
            bid: bid.parse::<usize>().unwrap(),
            score: HandPart2::score(hand, hand_type),
        };
    }

    fn score(cards: &str, hand_type: HandType) -> (HandType, u8, u8, u8, u8, u8) {
        let cards: Vec<u8> = cards.chars().into_iter().map(HandPart2::card_to_int).collect();
        return (hand_type, cards[0], cards[1], cards[2], cards[3], cards[4]);
    }
}

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod test_day07 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../resources/example.txt");
        assert_eq!(part1(input), 6440)
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../resources/example.txt");
        assert_eq!(part2(input), 5905)
    }

    #[test]
    fn test_hand_types() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::OnePair);
    }
}