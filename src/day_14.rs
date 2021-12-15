use std::collections::HashMap;
use std::io::Read;
use std::iter::once;

fn load(input: &mut dyn Read) -> (Vec<char>, HashMap<[char; 2], char>) {
    use std::io::{BufRead, BufReader};

    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    let template = lines.next().unwrap().unwrap().chars().collect();

    assert!(lines.next().unwrap().unwrap().is_empty());

    let rules = lines
        .map(|l| {
            let line = l.unwrap();
            let pieces: Vec<&str> = line.split(" -> ").collect();
            let mut from = pieces[0].chars();
            let from_0 = from.next().unwrap();
            let from_1 = from.next().unwrap();
            let to = pieces[1].chars().next().unwrap();
            ([from_0, from_1], to)
        })
        .collect();

    (template, rules)
}

fn part1_step(rules: &HashMap<[char; 2], char>, polymer: Vec<char>) -> Vec<char> {
    once(polymer[0])
        .chain(
            polymer
                .windows(2)
                .map(|window| match window {
                    &[prev, cur] => {
                        let key: [char; 2] = [prev, cur];
                        let result: Vec<char> = if let Some(insertion) = rules.get(&key) {
                            vec![*insertion, cur]
                        } else {
                            vec![cur]
                        };
                        result
                    }
                    _ => panic!(),
                })
                .flatten(),
        )
        .collect()
}

fn part1(input: &mut dyn Read) -> u32 {
    let (template, rules) = load(input);
    let polymer = (0..10).fold(template, |polymer, _| part1_step(&rules, polymer));
    let counts = polymer.into_iter().fold(HashMap::new(), |mut cts, c| {
        let old_count = cts.get(&c).unwrap_or(&0).clone();
        cts.insert(c, old_count + 1);
        cts
    });

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn part2(_: &mut dyn Read) -> u32 {
    0
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
        let mut f = File::open("input/day-14-sample.txt").unwrap();
        assert_eq!(part1(&mut f), 1588);
    }

    #[test]
    fn test_part2_sample() {
        let mut f = File::open("input/day-14-sample.txt").unwrap();
        assert_eq!(part2(&mut f), 0);
    }
}
