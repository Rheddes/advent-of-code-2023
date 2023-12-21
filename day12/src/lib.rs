use std::iter;
use std::path::Iter;
use std::process::id;
use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    return input.lines().map(parse_line)
        .map(|(pattern, springs)| process_row(pattern, springs))
        .sum();
}

fn is_valid(pattern: Vec<char>, springs: Vec<usize>) -> bool {
    return pattern.iter()
        .group_by(|c| **c == '#')
        .into_iter()
        .filter_map(|(sat, group)| if sat { Some(group.count()) } else { None })
        .zip(springs.iter())
        .all(|(p, s)| p == *s);
}

fn f(pattern: Vec<char>, springs: Vec<usize>, idx: usize) -> usize {
    if idx == pattern.len() {
        return if is_valid(pattern, springs) { 1 }  else { 0 };
    }
    let next_idx = pattern;
    let (head, tail) = pattern.split_at(idx);
    let current: char = v.first();
    let tail: &[i32] = &v[1..];
    return f()
    return 0;
}

fn process_row(pattern: &str, springs: Vec<usize>) -> usize {
    let pattern: Vec<char> = pattern.chars().collect();
    return f(pattern, springs, 0);
}

fn parse_line(line: &str) -> (&str, Vec<usize>) {
    let mut iter = line.splitn(2, ' ');
    let pattern= iter.next().expect("A pattern of hotsprings");
    let springs: Vec<usize>  = iter.next().expect("A description of broken springs")
        .split(',')
        .map(|x| x.parse::<usize>().expect("Valid springs"))
        .collect();
    return (pattern, springs);
}

#[cfg(test)]
mod test_day12 {
    use super::*;

    // #[test]
    // fn test_part1() {
    //     let input = include_str!("../resources/example.txt");
    //     assert_eq!(part1(input), 10);
    // }

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(vec!['#', '.', '#', '.', '#', '#', '#'], vec![1,1,3]), true);
    }
}