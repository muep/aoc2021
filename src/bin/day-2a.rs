use std::env::args;
use std::fs::File;
use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug, Eq, PartialEq)]
struct Position {
    depth: u32,
    distance: u32,
}

impl Position {
    fn prod(&self) -> u32 {
        self.depth * self.distance
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Cmd {
    Fwd(u32),
    Down(u32),
    Up(u32),
}

impl Cmd {
    fn from_line(line: &str) -> Cmd {
        if let Some(num) = line.strip_prefix("forward ") {
            return Cmd::Fwd(num.parse().unwrap());
        }

        if let Some(num) = line.strip_prefix("up ") {
            return Cmd::Up(num.parse().unwrap());
        }

        if let Some(num) = line.strip_prefix("down ") {
            return Cmd::Down(num.parse().unwrap());
        }

        panic!("bad!")
    }
}

fn find_pos(input: &mut dyn Read) -> Position {
    let (depth, distance) = BufReader::new(input)
        .lines()
        .map(|l| Cmd::from_line(&l.unwrap()))
        .fold((0, 0), |(depth, distance), cmd| match cmd {
            Cmd::Fwd(n) => (depth, distance + n),
            Cmd::Down(n) => (depth + n, distance),
            Cmd::Up(n) => (depth - n, distance),
        });

    Position {
        depth: depth,
        distance: distance,
    }
}

fn main() {
    let position = match args().nth(1) {
        Some(path) => find_pos(&mut File::open(path).unwrap()),
        None => find_pos(&mut stdin()),
    };

    println!("{:?} => answer: {}", position, position.prod());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmd() {
        assert_eq!(Cmd::Fwd(4), Cmd::from_line("forward 4"));
        assert_eq!(Cmd::Up(7), Cmd::from_line("up 7"));
        assert_eq!(Cmd::Down(1), Cmd::from_line("down 1"));
    }

    #[test]
    fn test_example() {
        let mut f = File::open("input/day-2-sample.txt").unwrap();
        let pos = find_pos(&mut f);
        assert_eq!(
            pos,
            Position {
                depth: 10,
                distance: 15
            }
        );
        assert_eq!(pos.prod(), 150);
    }
}
