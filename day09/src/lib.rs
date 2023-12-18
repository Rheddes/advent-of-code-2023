use itertools::Itertools;

pub fn part1(input: &str) -> isize {
    return parse_sequences(input).iter()
        .map(|sequence| get_next(sequence))
        .sum();
}

pub fn part2(input: &str) -> isize {
    return parse_sequences(input).iter()
        .map(|sequence| get_prev(sequence))
        .sum();
}

fn parse_sequences(input: &str) -> Vec<Vec<isize>> {
    let sequences: Vec<Vec<isize>> = input.lines().map(|line| {
        line.split_whitespace().map(|number| number.parse::<isize>().unwrap()).collect()
    }).collect();
    sequences
}

fn get_next(sequence: &Vec<isize>) -> isize {
    if sequence.iter().all(|x| *x == 0) { return 0; }
    let derivative = get_next(&sequence.iter().tuple_windows().map(|(x1, x2)| x2 - x1).collect());
    return sequence.last().unwrap() + derivative;
}

fn get_prev(sequence: &Vec<isize>) -> isize {
    if sequence.iter().all(|x| *x == 0) { return 0; }
    let derivative = get_prev(&sequence.iter().tuple_windows().map(|(x1, x2)| x2 - x1).collect());
    return sequence.first().unwrap() - derivative;
}

#[cfg(test)]
mod test_day09 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../resources/example.txt");
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../resources/example.txt");
        assert_eq!(part2(input), 2);
    }

    #[test]
    fn test_derivative() {
        assert_eq!(get_next(&vec![0]), 0);
    }
}