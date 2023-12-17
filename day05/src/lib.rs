use std::ops::Range;
use nom::sequence::{pair, preceded, separated_pair, tuple};
use nom::bytes::complete::{tag, take_until};
use nom::{IResult};
use nom::character::complete::{self, line_ending, newline, space1};
use nom::multi::{many1, separated_list1};

fn parse_seeds_p1(input: &str) -> IResult<&str, Vec<u64>> {
    return preceded(
        tag("seeds: "),
        separated_list1(space1, complete::u64)
    )(input);
}

fn parse_seeds_p2(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    return preceded(
        tag("seeds: "),
        separated_list1(space1, separated_pair(complete::u64, space1, complete::u64))
    )(input);
}

fn parse_map(input: &str) -> IResult<&str, Vec<(u64, u64, u64)>> {
    return preceded(
        take_until("map:"),
        preceded(
            pair(tag("map:"), newline),
            separated_list1(
                newline,
                tuple((complete::u64, preceded(space1, complete::u64), preceded(space1, complete::u64)))
            )
        )
    )(input);
}

fn parse_and_process_map(input: &str) -> IResult<&str, SeedMap> {
    let (input, seed_map) = parse_map(input).expect("Valid seed map");
    let seed_map = SeedMap {
        mappings: seed_map.iter().map(|mapping| {
            SeedMapping {
                source: mapping.1..(mapping.1 + mapping.2),
                destination: mapping.0..(mapping.0 + mapping.2),
            }
        }).collect(),
    };

    return Ok((input, seed_map));
}

fn parse_maps(input: &str) -> IResult<&str, Vec<SeedMap>> {
    return preceded(
        many1(line_ending),
        separated_list1(line_ending, parse_and_process_map)
    )(input);
}

pub fn part1(input: &str) -> u64 {
    let (input, seeds) = parse_seeds_p1(input).expect("Valid seeds");
    let (_, seed_maps) = parse_maps(input).expect("Valid map");
    let mapped = seed_maps.iter().fold(seeds, |cur, seed_map| {
        cur.iter().map(|seed| seed_map.map_seed(seed)).collect()
    });
    return *mapped.iter().min().unwrap();
}

pub fn part2(input: &str) -> u64 {
    let (input, seeds) = parse_seeds_p2(input).expect("Valid seeds");
    let seeds: Vec<Range<u64>> = seeds.iter()
        .map(|(start, length)| { *start..(start + length) })
        .collect();
    let (_, seed_maps) = parse_maps(input).expect("Valid map");

    let mapped_ranges = seed_maps.iter().fold(seeds, |cur, seed_map| {
        cur.iter().flat_map(|seed| seed_map.map_seed_range(seed)).collect()
    });
    return mapped_ranges.iter().map(|range| range.start).min().unwrap();
}

#[derive(Debug)]

struct SeedMapping {
    source: Range<u64>,
    destination: Range<u64>
}

impl SeedMapping {
    fn identity(start: u64, end: u64) -> SeedMapping {
        SeedMapping {
            source: start..end,
            destination: start..end,
        }
    }

    pub fn translate(self: &SeedMapping, seed: u64) -> u64 {
        seed - self.source.start + self.destination.start
    }

    pub fn clone(self: &SeedMapping) -> SeedMapping {
        SeedMapping {
            source: self.source.start..self.source.end,
            destination: self.destination.start..self.destination.end,
        }
    }
}

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<SeedMapping>
}

impl SeedMap {
    fn lowest_after(self: &SeedMap, seed: &u64) -> u64 {
        self.mappings.iter()
            .map(|mapping| mapping.source.start)
            .filter(|start| start > seed)
            .min().unwrap_or(u64::MAX)
    }
    fn find_map(self: &SeedMap, seed: &u64) -> SeedMapping {
        return match self.mappings.iter().find(|map| map.source.contains(seed)) {
            Some(seed_map) => seed_map.clone(),
            None =>  SeedMapping::identity(*seed, self.lowest_after(seed)),
        }
    }
    pub fn map_seed(self: &SeedMap, seed: &u64) -> u64 {
        self.find_map(seed).translate(*seed)
    }

    pub fn map_seed_range(self: &SeedMap, seed_range: &Range<u64>) -> Vec<Range<u64>> {
        let mut result: Vec<Range<u64>> = vec![];
        let mut seed_range_left = Some(seed_range.clone());
        while let Some(seed_range) = seed_range_left {
            let mapping = self.find_map(&seed_range.start);
            let mapped_end = mapping.source.end;
            if mapped_end < seed_range.end {
                result.push(mapping.translate(seed_range.start)..mapping.translate(mapped_end));
                seed_range_left = Some(mapped_end..seed_range.end);

            } else {
                result.push(mapping.translate(seed_range.start)..mapping.translate(seed_range.end));
                seed_range_left = None;
            }
        }
        return result;
    }
}

#[cfg(test)]
mod test_day05 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../resources/example.txt");
        assert_eq!(part1(input), 35);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../resources/example.txt");
        assert_eq!(part2(input), 46);
    }
}