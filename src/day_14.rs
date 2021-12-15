use std::collections::HashMap;
use std::io::Read;
use std::iter::once;

type Pair = [char; 2];

fn load(input: &mut dyn Read) -> (Vec<char>, HashMap<Pair, char>) {
    use std::io::{BufRead, BufReader};

    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    let template = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .chain(once('\0'))
        .collect();

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

fn part1_step(rules: &HashMap<Pair, char>, polymer: Vec<char>) -> Vec<char> {
    //println!("{:?}", polymer);
    polymer
        .windows(2)
        .map(|window| match window {
            &[cur, next] => {
                let key: Pair = [cur, next];
                let result: Vec<char> = if let Some(insertion) = rules.get(&key) {
                    vec![cur, *insertion]
                } else {
                    vec![cur]
                };
                result
            }
            _ => panic!(),
        })
        .flatten()
        .chain(once('\0'))
        .collect()
}

fn part1(input: &mut dyn Read) -> u32 {
    let (template, rules) = load(input);
    let polymer = (0..10).fold(template, |polymer, _| part1_step(&rules, polymer));
    let plen = polymer.len();

    let counts = polymer
        .into_iter()
        .take(plen - 1)
        .fold(HashMap::new(), |mut cts, c| {
            let old_count = cts.get(&c).unwrap_or(&0).clone();
            cts.insert(c, old_count + 1);
            cts
        });

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn part2(input: &mut dyn Read) -> u64 {
    let (template, rules) = load(input);
    let initial_pair_counts: HashMap<Pair, u64> = template
        .windows(2)
        .map(|window| match window {
            &[e0, e1] => {
                let pair: Pair = [e0, e1];
                pair
            }
            _ => panic!(),
        })
        .fold(HashMap::new(), |mut cts, pair| {
            let old = *cts.get(&pair).unwrap_or(&0);
            cts.insert(pair, old + 1);
            cts
        });

    let final_pair_counts: HashMap<Pair, u64> =
        (0..40).fold(initial_pair_counts, |mut pair_counts, _| {
            let mut additions: HashMap<Pair, u64> = HashMap::new();
            let mut removals: HashMap<Pair, u64> = HashMap::new();

            for (rule_key, rule_val) in rules.iter() {
                let old_cnt = match pair_counts.get(rule_key) {
                    None => {
                        continue;
                    }
                    Some(count) => *count,
                };

                removals.insert(*rule_key, old_cnt);

                let added_pairs: [Pair; 2] = [[rule_key[0], *rule_val], [*rule_val, rule_key[1]]];

                for p in added_pairs {
                    let prev_adds = *additions.get(&p).unwrap_or(&0);
                    additions.insert(p, prev_adds + old_cnt);
                }
            }

            for (key, cnt) in additions {
                let old = *pair_counts.get(&key).unwrap_or(&0);
                pair_counts.insert(key, old + cnt);
            }

            for (key, cnt) in removals {
                let old = *pair_counts.get(&key).unwrap_or(&0);
                pair_counts.insert(key, old - cnt);
            }

            pair_counts
        });

    let counts: HashMap<char, u64> =
        final_pair_counts
            .iter()
            .fold(HashMap::new(), |mut cts, (pair, cnt)| {
                let old_cnt = *cts.get(&pair[0]).unwrap_or(&0);
                cts.insert(pair[0], old_cnt + cnt);
                cts
            });

    counts.values().max().unwrap() - counts.values().min().unwrap()
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
    fn test_part1_full() {
        let mut f = File::open("input/day-14.txt").unwrap();
        assert_eq!(part1(&mut f), 3408);
    }

    #[test]
    fn test_part2_sample() {
        let mut f = File::open("input/day-14-sample.txt").unwrap();
        assert_eq!(part2(&mut f), 2188189693529);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/day-14.txt").unwrap();
        assert_eq!(part2(&mut f), 3724343376942);
    }
}
