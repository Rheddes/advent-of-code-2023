use day11::{part1, part2};

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input, 1_000_000));
}
