pub fn part1(input: &str) -> usize {
    parse(input);
    todo!("part1")
}

fn parse(input: &str) {
    let thing: Vec<()> = input.lines().map(parse_line).collect();
    dbg!(thing);
}

fn parse_line(line: &str) {
    let mut iter = line.splitn(2, ' ');
    let pattern = iter.next().unwrap();
    let batches = iter.next().unwrap();
    println!("{:?}", batches);
}

#[cfg(test)]
mod test_day12 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../resources/example.txt");
        assert_eq!(part1(input), 10);
    }
}