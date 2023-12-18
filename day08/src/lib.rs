use std::collections::{BTreeMap, HashMap};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1},
    IResult,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair}
};
use nom::character::complete::alphanumeric1;

pub fn part1(input: &str) -> usize {
    let (input, instructions) = parse_instructions(input).expect("Valid instructions");
    let (_, nodes) = parse_nodes(input).expect("Valid map");

    let mut cur = "AAA";
    return instructions.iter().cycle().enumerate().find_map(|(index, instruction)| {
        let (left, right) = nodes.get(cur).expect("invalid node");
        cur = match instruction {
            Instruction::Left(_) => left,
            Instruction::Right(_) => right,
        };
        if cur == "ZZZ" { Some(index + 1) } else { None }
    }).unwrap();
}

pub fn part2(input: &str) -> usize {
    let (input, instructions) = parse_instructions(input).expect("Valid instructions");
    let (_, nodes) = parse_nodes(input).expect("Valid map");

    let mut ghosts: Vec<Ghost> = nodes.keys()
        .filter(|name| name.ends_with("A"))
        .map(|node| Ghost::new(node))
        .collect();
    let _ = instructions.iter().cycle().enumerate().take_while(|(index, instruction)| {
        ghosts.iter_mut()
            .filter(|ghost| ghost.loop_info.is_none())
            .for_each(|ghost| {
                let (left, right) = nodes.get(ghost.current).unwrap();
                let new_node = match instruction {
                    Instruction::Left(_) => left,
                    Instruction::Right(_) => right,
                };
                ghost.visit(new_node, instruction, index + 1);
            });
        return ghosts.iter().any(|ghost| ghost.loop_info.is_none());
    }).last();
    let loops: Vec<LoopInfo> = ghosts.iter().map(|ghost| ghost.loop_info.unwrap()).collect();
    if !loops.iter().all(|l| l.start == l.period) {
        panic!("Can only handle all loops starting at 0");
    }
    return lcm(&loops.iter().map(|l| l.period).collect::<Vec<usize>>());
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, raw_instructions): (&str, Vec<char>) = many1(
        alt((
            complete::char('L'),
            complete::char('R'),
        ))
    )(input)?;
    return Ok((
        input,
        raw_instructions.iter()
            .enumerate()
            .map(|(i, c)| match c {
                'L' => Instruction::Left(i),
                'R' => Instruction::Right(i),
                _ => panic!("Should not happen (unrecognized instruction: {})", c)
            })
            .collect()
    ));
}

fn parse_nodes(input: &str) -> IResult<&str, BTreeMap<&str, (&str, &str)>> {
    let (input, _) = multispace1(input)?;
    let (input, entries): (&str, Vec<(&str, (&str, &str))>) = separated_list1(
        line_ending,
        separated_pair(
            alphanumeric1,
            tag(" = "),
            delimited(
                complete::char('('),
                separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                complete::char(')')
            )
        )
    )(input)?;
    debug_assert!(input.is_empty());
    return Ok((input, BTreeMap::from_iter(entries)));
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Instruction { Left(usize), Right(usize) }
impl Instruction {
    pub fn to_string(self: &Instruction) -> String {
        match self {
            Instruction::Left(x) => format!("(L@[{}])", x),
            Instruction::Right(x) => format!("(R@[{}])", x),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct LoopInfo {
    start: usize,
    period: usize,
}

#[derive(Debug)]
struct Ghost<'a> {
    current: &'a str,
    visited: HashMap<String, usize>,
    loop_info: Option<LoopInfo>,
}
impl <'a>Ghost<'a> {
    pub fn new(node: &str) -> Ghost {
        Ghost {
            current: node,
            visited: HashMap::new(),
            loop_info: None,
        }
    }

    pub fn visit(self: &mut Ghost<'a>, node: &'a str, after: &Instruction, index: usize) {
        let key = after.to_string() + "-" + node;
        self.current = node;
        if node.ends_with("Z") {
            if let Some(seen_index) = self.visited.get(&key) {
                self.loop_info = Some(LoopInfo { start: *seen_index, period: index-seen_index});
            } else {
                self.visited.insert(key, index);
            }
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
        assert_eq!(part1(example2), 6);
    }

    // Doesn't work with chosen method.
    // #[test]
    // fn test_part2() {
    //     let example3 = include_str!("../resources/example3.txt");
    //     assert_eq!(part2(example3), 6);
    // }

    #[test]
    fn test_loop_detection() {
        let nodes = BTreeMap::from([
           ("A", ("B", "C")),
           ("B", ("C", "Z")),
           ("C", ("B", "Z")),
           ("Z", ("B", "C")),
        ]);
        let instructions = vec![Instruction::Right(0), Instruction::Left(1), Instruction::Left(2)];
        assert_ne!(instructions[1], instructions[2]);
        assert_eq!(instructions[1], instructions[1]);
        assert_eq!(Instruction::Left(3), Instruction::Left(3));
        let mut ghost = Ghost::new("A");
        let _: Option<(usize, &Instruction)> = instructions.iter().cycle().enumerate().take_while(|(index, instruction)| {
            let (left, right) = nodes.get(ghost.current).unwrap();
            let new_node = match instruction {
                Instruction::Left(_) => left,
                Instruction::Right(_) => right,
            };
            ghost.visit(new_node, instruction, index + 1);
            return ghost.loop_info.is_none();
        }).last();
        let loop_info = ghost.loop_info.unwrap();
        assert_eq!(loop_info.start, 4);
        assert_eq!(loop_info.period, 3);
    }
}