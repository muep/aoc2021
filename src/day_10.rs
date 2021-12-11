use std::io::Read;

enum Paren {
    Opening(char),
    Closing(char),
}

impl Paren {
    fn from_char(c: char) -> Paren {
        match c {
            '(' => Paren::Opening('('),
            '[' => Paren::Opening('['),
            '{' => Paren::Opening('{'),
            '<' => Paren::Opening('<'),
            ')' => Paren::Closing('('),
            ']' => Paren::Closing('['),
            '}' => Paren::Closing('{'),
            '>' => Paren::Closing('<'),
            _ => panic!("unexpected char {}", c),
        }
    }
}

fn part1_line(line: &str) -> Option<u32> {
    let mut stack = Vec::new();

    for c in line.chars() {
        match Paren::from_char(c) {
            Paren::Opening(c) => {
                stack.push(c);
            }
            Paren::Closing(c) => {
                let expected = stack.pop().unwrap();
                if expected != c {
                    return Some(match c {
                        '(' => 3,
                        '[' => 57,
                        '{' => 1197,
                        '<' => 25137,
                        _ => panic!(),
                    });
                }
            }
        }
    }
    None
}

fn part1(input: &mut dyn Read) -> u32 {
    use std::io::{BufRead, BufReader};

    BufReader::new(input)
        .lines()
        .map(|l| part1_line(&l.unwrap()))
        .filter_map(|a| a)
        .sum()
}

pub fn run_part1(input: &mut dyn Read) {
    println!("{}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_part1_sample() {
        let mut f = File::open("input/day-10-sample.txt").unwrap();
        assert_eq!(part1(&mut f), 26397);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/day-10.txt").unwrap();
        assert_eq!(part1(&mut f), 344193);
    }
}
