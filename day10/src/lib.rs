use std::collections::HashMap;

use glam::IVec2;

pub fn part1(input: &str) -> usize {
    let matrix = parse(input);
    let length = matrix.into_iter().enumerate().find_map(|(idx, (_, pipe))| {
        return if pipe == Pipe::Start { Some(idx + 1) } else { None };
    }).expect("A loop");
    return length / 2;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Side { In, Out }
impl Side {
    pub fn opposite(self: &Side) -> Side {
        match self {
            Side::In => Side::Out,
            Side::Out => Side::In,
        }
    }
}

pub fn part2(input: &str) -> usize {
    let matrix = parse(input);
    let mut pipe: Vec<(IVec2, Pipe)> = matrix.clone().into_iter()
        .take_while(|(_, pipe)| pipe != &Pipe::Start)
        .collect();
    let first_neighbour = pipe.first().unwrap().0;
    let last_neighbour = pipe.last().unwrap().0;
    let potential_pipes = vec![Pipe::Horizontal, Pipe::Vertical, Pipe::NE, Pipe::NW, Pipe::SW, Pipe::SE];
    let start_pipe = potential_pipes.iter()
        .find(|potential_pipe| {
            matrix.is_connected_with_supplied_pipe(matrix.start, first_neighbour, **potential_pipe)
                && matrix.is_connected_with_supplied_pipe(matrix.start, last_neighbour, **potential_pipe)
        })
        .unwrap();
    pipe.push((matrix.start, *start_pipe));
    let pipe: HashMap<IVec2, Pipe> = pipe.into_iter().collect();
    let (y_size, x_size) = matrix.dimensions();
    return (0..y_size).into_iter().fold(0, |inside_cells, y| {
        let inside_for_row = (0..x_size).into_iter().fold((0, Side::Out), |(inside_cells_row, side), x| {
            let coord = IVec2::new(x as i32, y as i32);
            if let Some(pipe_cell) = pipe.get(&coord) {
                return match pipe_cell {
                    Pipe::Start | Pipe::Vertical | Pipe::NE | Pipe::NW => (inside_cells_row, side.opposite()),
                    _ => (inside_cells_row, side),
                };
            }
            return match side {
                Side::Out => (inside_cells_row, side),
                Side::In => (inside_cells_row + 1, side),
            };
        }).0;
        return inside_cells + inside_for_row;
    });
}

fn parse(input: &str) -> Matrix {
    let mut start: Option<IVec2> = None;
    let matrix: Vec<Vec<Pipe>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| match c {
            '-' => Pipe::Horizontal,
            '|' => Pipe::Vertical,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            'F' => Pipe::SE,
            '7' => Pipe::SW,
            'S' => {
                start = Some(IVec2 { x: x as i32, y: y as i32 });
                Pipe::Start // TOdo fix
            }
            '.' => Pipe::Ground,
            _ => panic!("HHHH"),
        }).collect()
    }).collect();
    let start = start.unwrap();
    return Matrix { cells: matrix, start };
}

fn neighbours_of(loc: IVec2) -> Vec<IVec2> {
    let offsets = vec![IVec2 { x: -1, y: 0 }, IVec2 { x: 0, y: -1 }, IVec2 { x: 1, y: 0 }, IVec2 { x: 0, y: 1 }];
    offsets.iter()
        .map(|offset| *offset + loc)
        .filter(|neighbour| !(neighbour.x < 0 || neighbour.y < 0))
        .collect()
}

#[derive(Debug, Clone)]
struct Matrix {
    cells: Vec<Vec<Pipe>>,
    start: IVec2,
}

impl Matrix {
    pub fn get(self: &Matrix, loc: IVec2) -> Pipe {
        if let Some(row) = self.cells.get(loc.y as usize) {
            if let Some(cell) = row.get(loc.x as usize) {
                return cell.clone()
            }
        }
        return Pipe::Ground;
    }

    pub fn is_connected(self: &Matrix, cur: IVec2, neighbour: IVec2) -> bool {
        self.is_connected_with_supplied_pipe(cur, neighbour, self.get(cur))
    }

    pub fn is_connected_with_supplied_pipe(self: &Matrix, cur: IVec2, neighbour: IVec2, pipe_for_cur: Pipe) -> bool {
        let dir_vec = neighbour - cur;
        if dir_vec.length_squared() != 1 { return false; }
        let cur = pipe_for_cur;
        let neighbour = self.get(neighbour);
        return match (cur, neighbour) {
            (
                Pipe::Vertical | Pipe::NW | Pipe::NE | Pipe::Start,
                Pipe::Vertical | Pipe::SW | Pipe::SE | Pipe::Start
            ) if dir_vec == IVec2 { x: 0, y: -1 } => true, // North
            (
                Pipe::Vertical | Pipe::SW | Pipe::SE | Pipe::Start,
                Pipe::Vertical | Pipe::NW | Pipe::NE | Pipe::Start
            ) if dir_vec == IVec2 { x: 0, y: 1 } => true, // South
            (
                Pipe::Horizontal | Pipe::NW | Pipe::SW | Pipe::Start,
                Pipe::Horizontal | Pipe::NE | Pipe::SE | Pipe::Start
            ) if dir_vec == IVec2 { x: -1, y: 0 } => true, // West
            (
                Pipe::Horizontal | Pipe::NE | Pipe::SE | Pipe::Start,
                Pipe::Horizontal | Pipe::NW | Pipe::SW | Pipe::Start
            ) if dir_vec == IVec2 { x: 1, y: 0 } => true, // East
            _ => false,
        };


    }

    pub fn dimensions(self: &Matrix) -> (usize, usize) {
        return (self.cells.len(), self.cells[0].len());
    }
}

impl IntoIterator for Matrix {
    type Item = (IVec2, Pipe);
    type IntoIter = MatrixIterator;

    fn into_iter(self) -> Self::IntoIter {
        let start = self.start;
        MatrixIterator {
            matrix: self,
            current: Some(start),
            last: None,
        }
    }
}

#[derive(Debug)]
struct MatrixIterator {
    matrix: Matrix,
    last: Option<IVec2>,
    current: Option<IVec2>,
}

impl Iterator for MatrixIterator {
    type Item = (IVec2, Pipe);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        let new = neighbours_of(current)
            .iter()
            .filter(|neighbour| self.last.is_none() || neighbour != &&self.last.unwrap())
            .map(|neighbour| (neighbour, self.matrix.get(*neighbour)))
            .find_map(|(neighbour, pipe)| {
                if self.matrix.is_connected(current, *neighbour) {
                    Some((neighbour, pipe))
                } else { None }
            })
            .map(|(point, pipe)| (point.clone(), pipe.clone()));
        self.last = Some(current);
        self.current = new.map(|(point, _)| point.clone());
        return new;
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
enum Pipe {
    Horizontal,
    Vertical,
    NE,
    NW,
    SE,
    SW,
    Start,
    Ground,
}


#[cfg(test)]
mod test_day10 {
    use glam::IVec2;

    use super::*;

    #[test]
    fn test_part1() {
        let input1 = include_str!("../resources/example1.txt");
        assert_eq!(part1(input1), 4);
    }

    #[test]
    fn test_part1_other() {
        let input2 = include_str!("../resources/example2.txt");
        assert_eq!(part1(input2), 8);
    }

    #[test]
    fn test_part2_a() { assert_eq!(part2(include_str!("../resources/example3.txt")), 4) }

    #[test]
    fn test_part2_b() { assert_eq!(part2(include_str!("../resources/example4.txt")), 8) }

    #[test]
    fn test_part2_c() { assert_eq!(part2(include_str!("../resources/example5.txt")), 10) }

    #[test]
    fn test_next() {
        let input1 = include_str!("../resources/example1.txt");
        let matrix = parse(input1);
        let mut matrix_iter = matrix.into_iter();
        assert_eq!(matrix_iter.next().unwrap(), (IVec2 { x: 2, y: 1 }, Pipe::Horizontal));
        assert_eq!(matrix_iter.next().unwrap(), (IVec2 { x: 3, y: 1 }, Pipe::SW));
        assert_eq!(matrix_iter.next().unwrap(), (IVec2 { x: 3, y: 2 }, Pipe::Vertical));
        assert_eq!(matrix_iter.next().unwrap(), (IVec2 { x: 3, y: 3 }, Pipe::NW));
        assert_eq!(matrix_iter.next().unwrap(), (IVec2 { x: 2, y: 3 }, Pipe::Horizontal));
        assert_eq!(matrix_iter.next().unwrap(), (IVec2 { x: 1, y: 3 }, Pipe::NE));
        assert_eq!(matrix_iter.next().unwrap(), (IVec2 { x: 1, y: 2 }, Pipe::Vertical));
        assert_eq!(matrix_iter.next().unwrap(), (IVec2 { x: 1, y: 1 }, Pipe::Start));
    }

    #[test]
    fn test_is_connected_north_of() {
        // [p1, p2]   | [7-]
        // [p3, p4]   | [L7]
        let p1 = IVec2 { x: 0, y: 0 };
        let p2 = IVec2 { x: 1, y: 0 };
        let p3 = IVec2 { x: 0, y: 1 };
        let p4 = IVec2 { x: 1, y: 1 };
        let matrix = Matrix {
            cells: vec![
                vec![Pipe::SW, Pipe::Horizontal],
                vec![Pipe::NE, Pipe::SW],
            ],
            start: IVec2 { x: 0, y: 0 },
        };
        assert!(matrix.is_connected(p1, p3));
        assert!(matrix.is_connected(p3, p1));
        assert!(matrix.is_connected(p3, p4));
        assert!(matrix.is_connected(p3, p4));

        assert_eq!(matrix.is_connected(p1, p2), false);
        assert_eq!(matrix.is_connected(p2, p1), false);
        assert_eq!(matrix.is_connected(p2, p4), false);
        assert_eq!(matrix.is_connected(p4, p2), false);

        assert_eq!(matrix.is_connected(p1, p4), false);
        assert_eq!(matrix.is_connected(p4, p1), false);
        assert_eq!(matrix.is_connected(p2, p3), false);
        assert_eq!(matrix.is_connected(p3, p2), false);
    }
}