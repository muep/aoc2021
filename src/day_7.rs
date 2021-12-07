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

fn part1(input: &mut dyn Read) -> u32 {
    let crabs = {
        let mut crabs = load(input);
        crabs.sort();
        crabs
    };

    let avg_pos = crabs[crabs.len() / 2];
    crabs
        .into_iter()
        .map(|c| i32::abs(c as i32 - avg_pos as i32) as u32)
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
    fn test_sample() {
        let mut f = File::open("input/day-7-sample.txt").unwrap();
        assert_eq!(part1(&mut f), 37);
    }

    #[test]
    fn test_part1_trivial() {
        assert_eq!(part1(&mut "7".as_bytes()), 0);
        assert_eq!(part1(&mut "7,8".as_bytes()), 1);
        assert_eq!(part1(&mut "1,1,5".as_bytes()), 4);
        assert_eq!(part1(&mut "1,1,5,10".as_bytes()), 13);
    }
}
