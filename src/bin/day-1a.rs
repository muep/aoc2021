use std::env::args;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn find_count(file_path: &str) -> u32 {
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

    cnt
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
        assert_eq!(find_count("input/day-1.txt"), 1529);
    }
}
