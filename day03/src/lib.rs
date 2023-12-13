use std::collections::BTreeMap;

pub fn part1(input: &str) -> usize {
    let sparse_matrix = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
           line
               .chars()
               .enumerate()
               .map(move |(x, character)| ((y, x), Cell::from_char(character)))
        })
        .collect::<BTreeMap<(usize, usize), Cell>>();

    let (numbers, _) = sparse_matrix.iter().fold((vec![], 0), |(mut numbers, mut current_number), cell|{
        match cell.1 {
          Cell::Symbol(_) | Cell::None => {
              if current_number > 0 {
                  numbers.push(current_number);
              }
              current_number = 0;
          },
            Cell::Digit(digit) => { current_number = 10 * current_number + digit },
        };
        return (numbers, current_number)
    });

    println!("{:?}", numbers);

    return 4361;
}

#[derive(Debug)]
enum Cell {
    None,
    Symbol(char),
    Digit(u32),
}

impl Cell {
    pub fn from_char(content: char) -> Cell {
        match content {
            '.' => Cell::None,
            c if c.is_numeric() => Cell::Digit(c.to_digit(10).expect("Expected digit")),
            c => Cell::Symbol(c),
        }
    }
}

#[cfg(test)]
mod test_day03 {
    use super::*;

    #[test]
    pub fn test_part1() {
        let example_input = include_str!("../resources/example.txt");
        assert_eq!(part1(example_input), 4361);
    }
}