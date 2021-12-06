use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

struct State([u64; 9]);

fn load(input: &mut dyn Read) -> State {
    BufReader::new(input)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|t| t.parse().unwrap())
        .fold(State([0; 9]), |State(mut counts), fish: usize| {
            counts[fish] += 1;
            State(counts)
        })
}

fn step(State(nums): State) -> State {
    State([
        nums[1],
        nums[2],
        nums[3],
        nums[4],
        nums[5],
        nums[6],
        nums[0] + nums[7],
        nums[8],
        nums[0],
    ])
}

fn fishies_after_days(input: &mut dyn Read, days: u16) -> u64 {
    let State(final_nums) = (0..days).fold(load(input), |s, _| step(s));
    final_nums.iter().sum()
}

fn part1(input: &mut dyn Read) -> u64 {
    fishies_after_days(input, 80)
}

fn part2(input: &mut dyn Read) -> u64 {
    fishies_after_days(input, 256)
}

pub fn run_part1(input: &mut dyn Read) {
    println!("{}", part1(input))
}

pub fn run_part2(input: &mut dyn Read) {
    println!("{}", part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_load() {
        let mut f = File::open("input/day-6-sample.txt").unwrap();
        let State(nums) = load(&mut f);
        assert_eq!(nums, [0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_sample_part1() {
        let mut f = File::open("input/day-6-sample.txt").unwrap();
        assert_eq!(part1(&mut f), 5934);
    }

    #[test]
    fn test_sample_part2() {
        let mut f = File::open("input/day-6-sample.txt").unwrap();
        assert_eq!(part2(&mut f), 26984457539);
    }
}
