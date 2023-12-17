use std::collections::HashMap;
use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::multispace1;
use nom::IResult;
use nom::multi::many1;

pub fn part1(input: &str) -> usize {
    let (input, instructions) = parse_instructions(input).expect("Valid instructions");
    let (input, _) = multispace1(input)?;

    dbg!(instructions);
    return 3;
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions): (&str, Vec<char>) = many1(alt((complete::char('L'), complete::char('R'))))(input)?;
    return Ok((input, instructions.iter().map(Instruction::from_char).collect()));
}

fn parse_map(input: &str) -> IResult<&str, HashMap<&str, Node>>

#[derive(Debug)]
enum Instruction { Left, Right }
impl Instruction {
    pub fn from_char(c: &char) -> Instruction {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Unknown instruction: {}", c),
        }
    }
}

#[cfg(test)]
mod test_day08 {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let example1 = include_str!("../resources/example1.txt");
        assert_eq!(part1(example1), 2);
    }
    #[test]
    fn test_part1_example2() {
        let example2 = include_str!("../resources/example2.txt");
        assert_eq!(part1(example2), 2);
    }
}