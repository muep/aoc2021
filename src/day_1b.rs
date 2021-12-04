use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug)]
struct State {
    increases: u32,
    prev: (u32, u32, u32),
}

impl State {
    fn new(x0: u32, x1: u32, x2: u32) -> State {
        State {
            increases: 0,
            prev: (x0, x1, x2),
        }
    }

    fn step(&mut self, x: u32) {
        if x > self.prev.0 {
            self.increases += 1;
        }
        self.prev.0 = self.prev.1;
        self.prev.1 = self.prev.2;
        self.prev.2 = x;
    }
}

fn find_count(input: &mut dyn Read) -> u32 {
    let mut nums = BufReader::new(input)
        .lines()
        .map(|x| x.unwrap().parse().unwrap());

    let mut state = State::new(
        nums.next().unwrap(),
        nums.next().unwrap(),
        nums.next().unwrap(),
    );

    for num in nums {
        state.step(num);
    }

    state.increases
}

pub fn run(input: &mut dyn Read) {
    println!("{}", find_count(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn find_count_test() {
        let mut f = File::open("input/day-1.txt").unwrap();
        assert_eq!(find_count(&mut f), 1567);
    }
}
