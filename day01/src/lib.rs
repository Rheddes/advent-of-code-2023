pub fn process_part1(input: &str) -> usize {
    input.lines().map(|line| part1(line)).sum()
}

pub fn process_part2(input: &str) -> usize {
    input.lines().map(|line| part2(line)).sum()
}

fn part1(input: &str) -> usize {
    let a = input.chars().find(|c| c.is_numeric()).unwrap();
    let b = input.chars().rev().find(|c| c.is_numeric()).unwrap();
    return format!("{a}{b}").parse::<usize>().unwrap();
}

fn part2(input: &str) -> usize {
    return part1(replace_digits(input).as_str());
}


fn replace_digits(input: &str) -> String {
    input.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3ree")
        .replace("four", "f4ur")
        .replace("five", "f5ve")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8ght")
        .replace("nine", "n9ne")
}

mod test {
    use super::*;

    #[test]
    pub fn test_part1() {
        assert_eq!(part1("treb7uchet"), 77);
        assert_eq!(part1("1abc2"), 12);
        assert_eq!(part1("pqr3stu8vwx"), 38);
        assert_eq!(part1("a1b2c3d4e5f"), 15);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2("two1nine"), 29);
        assert_eq!(part2("eightwothree"), 83);
        assert_eq!(part2("abcone2threexyz"), 13);
        assert_eq!(part2("xtwone3four"), 24);
        assert_eq!(part2("4nineeightseven2"), 42);
        assert_eq!(part2("zoneight234"), 14);
        assert_eq!(part2("7pqrstsixteen"), 76);
    }
}