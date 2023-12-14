use nom::sequence::{pair, preceded};
use nom::bytes::complete::tag;
use nom::{IResult, Parser};
use nom::branch::Alt;
use nom::character::complete::{self, space1};
use nom::multi::separated_list1;

fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    return preceded(
        tag("seeds: "),
        separated_list1(space1, complete::u32)
    )(input);
}

fn parse_map(input: &str) -> IResult<&str, &str> {
    todo!();
}

pub fn part1(input: &str) -> usize {
    let (input, seeds) = parse_seeds(input).expect("Valid seeds");
    let (input, seed_map) = parse_map(input).expect("Valid map");

    dbg!(seeds);
    return 3;
}

#[cfg(test)]
mod test_day05 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../resources/example.txt");
        assert_eq!(part1(input), 35);
    }
}