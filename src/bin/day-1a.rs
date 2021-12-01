use std::env::args;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn run(file_path: &str) {
    let file = File::open(file_path).unwrap();

    let mut lines = BufReader::new(file).lines().map(|x| x.unwrap());
    let mut prev = lines.next().unwrap().parse::<u32>().unwrap();
    let mut cnt = 0;

    for line in lines {
        let num = line.parse::<u32>().unwrap();
        if num > prev {
            cnt += 1;
        }
        prev = num;
    }

    println!("{}", cnt);
}

fn main() {
    for arg in args().skip(1) {
        run(&arg);
    }
}
