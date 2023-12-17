pub fn part1(input: &str) -> usize {
    let raw_input: Vec<usize> = input.split_whitespace().into_iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    let (times, distances) = raw_input.split_at(raw_input.len() / 2);
    return times.iter()
        .zip(distances)
        .map(|(time, distance)| Race { time: *time, distance: *distance })
        .map(|race| race.n_ways_to_win())
        .product();
}

fn numeric(input: &&str) -> bool {
    match input.parse::<usize>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn part2(input: &str) -> usize {
    let raw_input: Vec<&str> = input.split_whitespace().into_iter().filter(numeric).collect();
    let (times, distances) = raw_input.split_at(raw_input.len() / 2);
    let time = times.iter().fold("".to_owned(), |res, cur| res + cur).parse::<usize>().unwrap();
    let distance = distances.iter().fold("".to_owned(), |res, cur| res + cur).parse::<usize>().unwrap();
    return (Race { time, distance, }).n_ways_to_win();
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    pub fn n_ways_to_win(self: &Race) -> usize {
        let first_win = (1..self.time).into_iter().find(|n| (self.time - n) * n > self.distance).unwrap();
        return if self.time % 2 == 0 {
            self.time - (2 * (first_win - 1)) - 1
        } else {
            self.time - (2 * first_win) + 1
        };
    }
}


mod test {
    use super::*;

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(include_str!("../resources/example.txt")), 288);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(include_str!("../resources/example.txt")), 71503);

    }
}