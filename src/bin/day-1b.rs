use std::env::args;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

fn find_count(file_path: &str) -> u32 {
    let file = File::open(file_path).unwrap();

    let mut nums = BufReader::new(file)
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

fn main() {
    for arg in args().skip(1) {
        println!("{}", find_count(&arg));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_count_test() {
        assert_eq!(find_count("input/day-1.txt"), 1567);
    }
}
