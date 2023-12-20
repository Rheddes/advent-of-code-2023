use std::cmp::{max, min};
use std::collections::HashSet;
use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let galaxy_map = parse(input);
    let mapped_galaxies: Vec<(usize, usize)> = galaxy_map.galaxies.iter().map(|(x, y)| {
        let step_y = galaxy_map.empty_rows.iter().filter(|row_y| row_y < &y).count();
        let step_x = galaxy_map.empty_columns.iter().filter(|col_x| col_x < &x).count();
        (x + step_x, y + step_y)
    }).collect();
    return mapped_galaxies.iter().combinations(2)
        .map(|pair| manhattan_distance(*pair[0], *pair[1]))
        .sum();
}

pub fn part2(input: &str, n_times: usize) -> usize {
    let galaxy_map = parse(input);
    let mapped_galaxies: Vec<(usize, usize)> = galaxy_map.galaxies.iter().map(|(x, y)| {
        // We subtract one to account for _replacing_ the row.
        let step_y = galaxy_map.empty_rows.iter().filter(|row_y| row_y < &y).count() * (n_times-1);
        let step_x = galaxy_map.empty_columns.iter().filter(|col_x| col_x < &x).count() * (n_times-1);
        (x + step_x, y + step_y)
    }).collect();
    return mapped_galaxies.iter().combinations(2)
        .map(|pair| manhattan_distance(*pair[0], *pair[1]))
        .sum::<usize>();
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    return (max(a.0, b.0) - min(a.0, b.0)) +  (max(a.1, b.1) - min(a.1, b.1));
}

fn parse(input: &str) -> GalaxyMap {
    let mut empty_rows: HashSet<usize> = (0..input.lines().count()).collect();
    let mut empty_columns: HashSet<usize> = (0..input.lines().next().unwrap().chars().count()).collect();
    let mut galaxies: Vec<(usize, usize)> = vec![];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                galaxies.push((x, y));
                if empty_rows.contains(&y) { empty_rows.remove(&y); }
                if empty_columns.contains(&x) { empty_columns.remove(&x); }
            }
        });
    });
    let mut empty_columns: Vec<usize> = empty_columns.iter().cloned().collect();
    empty_columns.sort();
    let mut empty_rows: Vec<usize> = empty_rows.iter().cloned().collect();
    empty_rows.sort();
    return GalaxyMap { galaxies, empty_columns, empty_rows };
}

#[derive(Debug)]
struct GalaxyMap {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
}

#[cfg(test)]
mod test_day11 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../resources/example.txt");
        assert_eq!(part1(input), 374);
    }

    #[test]
    fn test_part2_10() {
        let input = include_str!("../resources/example.txt");
        assert_eq!(part2(input, 10), 1030);
    }

    #[test]
    fn test_part2_100() {
        let input = include_str!("../resources/example.txt");
        assert_eq!(part2(input, 100), 8410);
    }

    #[test]
    fn test_parse() {
        let parsed = parse(include_str!("../resources/example.txt"));
        assert_eq!(parsed.galaxies.len(), 9);
        assert_eq!(parsed.empty_rows.len(), 2);
        assert_eq!(parsed.empty_columns.len(), 3);
    }
}