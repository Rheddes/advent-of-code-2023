use std::collections::HashMap;

type Cache = HashMap<(Vec<char>, Vec<usize>), usize>;

pub fn part1(input: &str) -> usize {
    return input.lines().map(parse_line)
        .map(|(pattern, springs)| process_row_string_pattern(pattern, springs))
        .sum();
}

pub fn part2(input: &str) -> usize {
    return input.lines().map(parse_line)
        .map(|(pattern, springs)| ([pattern].repeat(5).join("?"), springs.repeat(5)))
        .map(|(pattern, springs)| process_row_string_pattern(pattern.as_str(), springs))
        .sum();
}

fn process_row_string_pattern(pattern: &str, springs: Vec<usize>) -> usize {
    return process_row(pattern.chars().collect::<Vec<_>>().as_slice(), springs.as_slice(), &mut HashMap::new());
}

fn process_row(pattern: &[char], springs: &[usize], cache: &mut Cache) -> usize {
    let cache_key = (pattern.to_vec(), springs.to_vec());
    if let Some(&result) = cache.get(&cache_key) { return result; }
    if springs.is_empty() {
        return !pattern.contains(&'#') as usize;
    }
    if pattern.is_empty() {
        return 0;
    }
    let result = match pattern[0] {
        '.' => process_row(&pattern[1..], springs, cache),
        '#' => process_spring(pattern, springs, cache),
        '?' => process_spring(pattern, springs, cache) + process_row(&pattern[1..], springs, cache),
        _ => panic!("Illegal character"),
    };
    cache.insert(cache_key, result);
    return result;
}

fn process_spring(pattern: &[char], springs: &[usize], cache: &mut Cache) -> usize {
    let spring_length = springs[0];
    if pattern.len() < spring_length || pattern[0..spring_length].contains(&'.') {
        return 0;
    }
    if pattern.len() == spring_length {
        return (springs.len() == 1) as usize;
    }
    if pattern[spring_length] == '#' {
        return 0;
    }
    return process_row(&pattern[spring_length+1..], &springs[1..], cache);
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
    fn test_part1_oneline() {
        assert_eq!(part1("???.### 1,1,3"), 1);
        assert_eq!(part1(".??..??...?##. 1,1,3"), 4);
        assert_eq!(part1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(part1("????.#...#... 4,1,1"), 1);
        assert_eq!(part1("????.######..#####. 1,6,5"), 4);
        assert_eq!(part1("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn test_part2_oneline() {
        assert_eq!(part2("???.### 1,1,3"), 1);
        assert_eq!(part2(".??..??...?##. 1,1,3"), 16384);
        assert_eq!(part2("?###???????? 3,2,1"), 506250);
    }
}