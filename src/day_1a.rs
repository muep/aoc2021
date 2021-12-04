use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

fn find_count(input: &mut dyn Read) -> u32 {
    let mut lines = BufReader::new(input).lines().map(|x| x.unwrap());
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
        assert_eq!(find_count(&mut f), 1529);
    }
}
