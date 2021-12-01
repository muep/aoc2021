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

fn run(file_path: &str) {
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

    println!("{}", state.increases);
}

fn main() {
    for arg in args().skip(1) {
        run(&arg);
    }
}
