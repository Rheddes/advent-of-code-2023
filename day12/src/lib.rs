use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    return input.lines().map(parse_line)
        .map(|(pattern, springs)| process_row(pattern, springs))
        .sum();
}

pub fn part2(input: &str) -> usize {
    return input.lines().map(parse_line)
        .map(|(pattern, springs)| {
            let extend_pattern = [pattern, pattern, pattern, pattern, pattern].join("?");
            let extend_springs: Vec<usize> = springs.iter().cycle().take(springs.len() * 5).cloned().collect();
            return (extend_pattern, extend_springs);
        })
        .map(|(pattern, springs)| process_row(pattern.as_str(), springs))
        .sum();
}

fn is_valid(pattern: &Vec<char>, springs: &Vec<usize>) -> bool {
    return pattern.iter().filter(|p| p == &&'#').count() == springs.iter().sum()
        && is_valid_until(pattern, springs, pattern.len(), true)
}

fn is_valid_until(pattern: &Vec<char>, springs: &Vec<usize>, until: usize, strict: bool) -> bool {
    return pattern.iter()
        .take(until)
        .group_by(|c| **c == '#')
        .into_iter()
        .filter_map(|(sat, group)| if sat { Some(group.count()) } else { None })
        .zip(springs.iter())
        .all(|(p, s)| if strict { p == *s } else { p <= *s }); // Last group could also be less :\
}

fn f(pattern: Vec<char>, springs: &Vec<usize>, skip_first: usize) -> usize {
    let next_idx = pattern.iter().enumerate().skip(skip_first).find(|(_, p)| p == &&'?');
    if next_idx.is_none() {
        return if is_valid(&pattern, springs) { 1 }  else { 0 };
    }
    let (idx, _) = next_idx.unwrap();
    if !is_valid_until(&pattern, springs, idx, false) { return 0; }
    let head = &pattern[0..idx];
    let tail = &pattern[idx+1..];
    return f([head, &['#'], tail].concat(), springs, idx) + f([head, &['.'], tail].concat(), springs, idx)
}

fn process_row(pattern: &str, springs: Vec<usize>) -> usize {
    let pattern: Vec<char> = pattern.chars().collect();
    return f(pattern, &springs, 0);
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

    #[test]
    fn test_part1() {
        let input = include_str!("../resources/example.txt");
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn test_part2_oneline() {
        assert_eq!(part2("???.### 1,1,3"), 1);
        assert_eq!(part2(".??..??...?##. 1,1,3"), 16384);
        // assert_eq!(part2("?###???????? 3,2,1"), 506250);
    }

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(&vec!['#', '.', '#', '.', '#', '#', '#'], &vec![1,1,3]), true);
    }

    fn build_vec(input: &str) -> Vec<char> { input.chars().collect() }

    #[test]
    fn test_f() {
        assert_eq!(f("???.###".chars().collect(), &vec![1,1,3], 0), 1);
        assert_eq!(f("????.###".chars().collect(), &vec![1,1,3], 0), 3);
        assert_eq!(f("?###????????".chars().collect(), &vec![3,2,1], 0), 10);
    }

    #[test]
    fn test_is_valid_until() {
        assert_eq!(is_valid(&build_vec(".###.##.#..#"), &vec![3,2,1]), false);
        assert_eq!(is_valid(&build_vec(".###.##.#..."), &vec![3,2,1]), true);

    }

    #[test]
    fn test_valid_until() {
        assert_eq!(is_valid_until(&build_vec("####????????"), &vec![3,2,1], 4, false), false);
        assert_eq!(is_valid_until(&build_vec(".###????????"), &vec![3,2,1], 4, false), true);
        assert_eq!(is_valid_until(&build_vec(".###.#??????"), &vec![3,2,1], 6, false), true);
    }
}