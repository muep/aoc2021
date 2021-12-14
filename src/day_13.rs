use std::io::Read;

fn part1(_: &mut dyn Read) -> u32 {
    0
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
        let mut f = File::open("input/day-13-sample.txt").unwrap();
        assert_eq!(part1(&mut f), 0);
    }

    #[test]
    fn test_part2_sample() {
        let mut f = File::open("input/day-13-sample.txt").unwrap();
        assert_eq!(part2(&mut f), 0);
    }
}
