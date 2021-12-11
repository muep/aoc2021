use std::io::Read;

struct ParenType {
    opening: char,
    closing: char,
    score_invalid: u32,
    score_autocomplete: u64,
}

const PARENS: [ParenType; 4] = [
    ParenType {
        opening: '(',
        closing: ')',
        score_invalid: 3,
        score_autocomplete: 1,
    },
    ParenType {
        opening: '[',
        closing: ']',
        score_invalid: 57,
        score_autocomplete: 2,
    },
    ParenType {
        opening: '{',
        closing: '}',
        score_invalid: 1197,
        score_autocomplete: 3,
    },
    ParenType {
        opening: '<',
        closing: '>',
        score_invalid: 25137,
        score_autocomplete: 4,
    },
];

enum Paren {
    Opening { closing_pair: char },
    Closing(char),
}

impl Paren {
    fn from_char(c: char) -> Paren {
        for ParenType {
            opening, closing, ..
        } in PARENS
        {
            if c == opening {
                return Paren::Opening {
                    closing_pair: closing,
                };
            } else if c == closing {
                return Paren::Closing(c);
            }
        }
        panic!("unexpected char {}", c)
    }
}

enum LineStatus {
    IllegalCharacter(char),
    AutoComplete(String),
}

fn score_invalid(c: char) -> u32 {
    for ParenType {
        closing,
        score_invalid,
        ..
    } in PARENS
    {
        if closing == c {
            return score_invalid;
        }
    }
    0
}

fn score_autocomplete(c: char) -> u64 {
    for ParenType {
        closing,
        score_autocomplete,
        ..
    } in PARENS
    {
        if closing == c {
            return score_autocomplete;
        }
    }
    0
}

fn parse(line: &str) -> LineStatus {
    let mut stack = Vec::new();

    for c in line.chars() {
        match Paren::from_char(c) {
            Paren::Opening { closing_pair: pair } => {
                stack.push(pair);
            }
            Paren::Closing(c) => {
                let expected = stack.pop().unwrap();
                if expected != c {
                    return LineStatus::IllegalCharacter(c);
                }
            }
        }
    }
    stack.reverse();
    LineStatus::AutoComplete(stack.iter().collect())
}

fn part1(input: &mut dyn Read) -> u32 {
    use std::io::{BufRead, BufReader};

    BufReader::new(input)
        .lines()
        .map(|l| parse(&l.unwrap()))
        .filter_map(|a| match a {
            LineStatus::IllegalCharacter(c) => Some(score_invalid(c)),
            _ => None,
        })
        .sum()
}

fn part2(input: &mut dyn Read) -> u64 {
    use std::io::{BufRead, BufReader};

    let mut scores: Vec<u64> = BufReader::new(input)
        .lines()
        .map(|l| parse(&l.unwrap()))
        .filter_map(|a| match a {
            LineStatus::AutoComplete(s) => {
                Some(s.chars().fold(0, |acc, c| acc * 5 + score_autocomplete(c)))
            }
            _ => None,
        })
        .collect();

    scores.sort();
    scores[scores.len() / 2]
}

pub fn run_part1(input: &mut dyn Read) {
    println!("{}", part1(input));
}

pub fn run_part2(input: &mut dyn Read) {
    println!("{}", part2(input));
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

    #[test]
    fn test_part2_sample() {
        let mut f = File::open("input/day-10-sample.txt").unwrap();
        assert_eq!(part2(&mut f), 288957);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/day-10.txt").unwrap();
        assert_eq!(part2(&mut f), 3241238967);
    }
}
