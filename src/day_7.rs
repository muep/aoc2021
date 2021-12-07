use std::io::Read;

fn load(input: &mut dyn Read) -> Vec<u32> {
    use std::io::{BufRead, BufReader};

    BufReader::new(input)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn absub_u32(a: u32, b: u32) -> u32 {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn expand(old_min: u32, val: u32, old_max: u32) -> (u32, u32) {
    if val < old_min {
        (val, old_max)
    } else if val > old_max {
        (old_min, val)
    } else {
        (old_min, old_max)
    }
}

fn part1(input: &mut dyn Read) -> u32 {
    let crabs = {
        let mut crabs = load(input);
        crabs.sort();
        crabs
    };

    let avg_pos = crabs[crabs.len() / 2];
    crabs
        .into_iter()
        .map(|c| absub_u32(c, avg_pos) as u32)
        .sum()
}

fn part2_cost(pos: u32, crab: u32) -> u32 {
    (0..absub_u32(pos, crab)).map(|a| a + 1).sum()
}

fn part2(input: &mut dyn Read) -> u32 {
    let crabs = load(input);

    let initial = (crabs[0], crabs[0]);
    let (min, max) = crabs
        .iter()
        .skip(1)
        .fold(initial, |(omin, omax), c| expand(omin, *c, omax));

    (min..=max)
        .map(|pos| crabs.iter().map(|c| part2_cost(pos, *c)).sum())
        .min()
        .unwrap()
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
        let mut f = File::open("input/day-7-sample.txt").unwrap();
        assert_eq!(part1(&mut f), 37);
    }

    #[test]
    fn test_part2_sample() {
        let mut f = File::open("input/day-7-sample.txt").unwrap();
        assert_eq!(part2(&mut f), 168);
    }

    #[test]
    fn test_part1_trivial() {
        assert_eq!(part1(&mut "7".as_bytes()), 0);
        assert_eq!(part1(&mut "7,8".as_bytes()), 1);
        assert_eq!(part1(&mut "1,1,5".as_bytes()), 4);
        assert_eq!(part1(&mut "1,1,5,10".as_bytes()), 13);
    }
}
