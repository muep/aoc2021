use std::io::Read;

fn part1(_: &mut dyn Read) -> u32 {
    26397
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
}
