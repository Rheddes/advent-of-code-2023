use glam::IVec2;

pub fn part1(input: &str) -> usize {
    let (start, matrix) = parse(input);
    dbg!(matrix.get(start));
    todo!()
}

fn parse(input: &str) -> (IVec2, Matrix) {
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
    return (start, Matrix { cells: matrix, last: None, current: start });
}

fn neighbours_of(loc: IVec2) -> Vec<IVec2> {
    let offsets = vec![IVec2 { x: -1, y: 0 }, IVec2 { x: 0, y: -1 }, IVec2 { x: 1, y: 0 }, IVec2 { x: 0, y: 1 }];
    offsets.iter()
        .map(|offset| *offset + loc)
        .filter(|neighbour| !(neighbour.x < 0 || neighbour.y < 0))
        .collect()
}

struct Matrix {
    cells: Vec<Vec<Pipe>>,
    last: Option<IVec2>,
    current: IVec2,
}
impl Matrix {
    pub fn get(self: &Matrix, loc: IVec2) -> &Pipe {
        return &self.cells[loc.x as usize][loc.y as usize];
    }

    pub fn next(self: &mut Matrix) -> IVec2 {
        let north = IVec2 { x: -1, y: 0 };
        let west = IVec2 { x: 0, y: -1 };
        let south = IVec2 { x: 1, y: 0 };
        let east = IVec2 { x: 0, y: 1 };
        let new = neighbours_of(self.current)
            .iter()
            .filter(|neighbour| self.last.is_none() || neighbour != &&self.last.unwrap())
            .map(|neighbour| (neighbour, self.get(*neighbour)))
            .find_map(|(neighbour, pipe)|
                match pipe {
                    Pipe::Vertical | Pipe::SE | Pipe::SW if *neighbour - self.current == north => Some(neighbour),
                    Pipe::Horizontal | Pipe::SE | Pipe::NE if *neighbour - self.current == west => Some(neighbour),
                    Pipe::Vertical | Pipe::NE | Pipe::SW if *neighbour - self.current == south => Some(neighbour),
                    Pipe::Horizontal | Pipe::SW | Pipe::NW if *neighbour - self.current == east => Some(neighbour),
                _ => None,
                })
            .cloned()
            .unwrap();
        self.last = Some(self.current);
        self.current = new.clone();
        return new.clone();

    }
}

#[derive(Debug, Copy, Clone)]
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
    use super::*;

    #[test]
    fn test_part1() {
        let input1 = include_str!("../resources/example1.txt");
        assert_eq!(part1(input1), 4);
        let input2 = include_str!("../resources/example1.txt");
        assert_eq!(part1(input2), 8);
    }

    #[test]
    fn test_next() {
        let input1 = include_str!("../resources/example1.txt");
        let (start, mut matrix) = parse(input1);
        println!("{:?}", matrix.next());
        println!("{:?}", matrix.next());
        println!("{:?}", matrix.next());
        println!("{:?}", matrix.next());
        println!("{:?}", matrix.next());
    }
}