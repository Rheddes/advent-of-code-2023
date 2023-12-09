use day01::{process_part1, process_part2};

fn main() {
    let result_part1: usize = process_part1(include_str!("../resources/input.txt"));
    println!("part 1: {result_part1}");
    let result_part2: usize = process_part2(include_str!("../resources/input.txt"));
    println!("part 2: {result_part2}");
}
