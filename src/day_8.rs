use std::io::Read;

fn part1(input: &mut dyn Read) -> u32 {
    use std::io::{BufRead, BufReader};
    let target_sizes = [2, 3, 4, 7];

    BufReader::new(input)
        .lines()
        .map(|l| {
            l.unwrap()
                .split(" | ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .filter(|p| target_sizes.contains(&p.len()))
                .count() as u32
        })
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
        let mut f = File::open("input/day-8-sample.txt").unwrap();
        assert_eq!(part1(&mut f), 26);
    }
}
