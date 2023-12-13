use std::collections::{BTreeMap, HashSet};
use std::io::Read;

pub fn part1(input: &str) -> u32 {
    let sparse_matrix = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
           line
               .chars()
               .enumerate()
               .filter(|(_, character)| *character != '.' )
               .map(move |(x, character)| (Point(y as i32, x as i32), Cell::from_char(character)))
        })
        .collect::<BTreeMap<Point, Cell>>();

    let numbers = construct_numbers(&sparse_matrix);
    let symbol_set: HashSet<Point> = sparse_matrix.iter().filter_map(|(point, cell)| match cell {
        Cell::Symbol(_) => Some(point.clone()),
        _ => None,
    }).collect();

    return numbers.iter().filter(|number| {
        number.neighbouring_points().iter().any(|neighbour| symbol_set.contains(neighbour))
    }).map(|number| number.value).sum();
}

// The not so pretty number constructing code.
// Preferably would require less in for loop state mutability.
// This is the best for now. At least it works™️.
fn construct_numbers(sparse_matrix: &BTreeMap<Point, Cell>) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];
    let mut current_number:u32 = 0;
    let mut current_points: Vec<Point> = vec![];
    for (point, cell) in sparse_matrix.iter() {
        if let Cell::Digit(digit) = cell {
            if current_points.last().is_some_and(|last_point| { last_point.1 + 1 == point.1}) {
                current_number = current_number * 10 + digit;
                current_points.push(point.clone());
            } else {
                if current_number > 0 {
                    numbers.push(Number { value: current_number, locations: current_points.clone() });
                }
                current_number = *digit;
                current_points = vec![point.clone()];
            }
        }
    }
    numbers.push(Number { value: current_number, locations: current_points.clone() });
    return numbers;
}

#[derive(Debug)]
struct Number<> {
    value: u32,
    locations: Vec<Point>,
}

impl Number {
    pub fn neighbouring_points(self: &Number) -> Vec<Point> {
        let Point(base_y, start_x) = *self.locations.first().unwrap();
        let end_x = self.locations.last().unwrap().1;
        return [Point(base_y, start_x - 1), Point(base_y, end_x + 1)]
            .into_iter()
            .chain(
                (start_x-1..=end_x+1).into_iter().map(|x| Point(base_y - 1, x))
            )
            .chain(
                (start_x-1..=end_x+1).into_iter().map(|x| Point(base_y + 1, x))
            ).collect();
    }
}
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
struct Point(i32, i32);

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

    pub fn is_number(self: &Cell) -> bool {
        match self {
            Cell::Digit(_) => true,
            _ => false,
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

    #[test]
    fn test_construct_numbers() {
        let sparse_matrix = BTreeMap::from([
            (Point(0, 0), Cell::Digit(4)),
            (Point(0, 1), Cell::Digit(6)),
            (Point(0, 2), Cell::Digit(7)),
            (Point(0, 3), Cell::Symbol('&')),
            (Point(0, 4), Cell::Digit(1)),
            (Point(0, 5), Cell::Digit(5)),
            (Point(1, 0), Cell::Digit(5)),
        ]);

        let numbers = construct_numbers(&sparse_matrix);
        assert_eq!(numbers[0].value, 467);
        assert_eq!(numbers[1].value, 15);
        assert_eq!(numbers[2].value, 5);
    }

    #[test]
    fn test_neighbours_of_number() {
        let number = Number {
            value: 467,
            locations: vec![Point(0, 0), Point(0, 1), Point(0, 2)],
        };
        assert_eq!(number.neighbouring_points().len(), 12);
    }
}