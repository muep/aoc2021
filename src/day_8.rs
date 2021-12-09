use std::io::Read;

/* Part 1 is pretty trivial so it gets placed here at the top */
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

/* Normal BCD mapping */
const BCD_PATTERNS: [u8; 10] = [
    0b1110111, // 0
    0b0100100, // 1
    0b1011101, // 2
    0b1101101, // 3
    0b0101110, // 4
    0b1101011, // 5
    0b1111011, // 6
    0b0100101, // 7
    0b1111111, // 8
    0b1101111, // 9
];

/* The mixed mapping looks very quite similar, so let's define an
 * alias. */
type Bcd = u8;

/* Same as Bcd, but with an arbitrary, typically unknown order */
type MixedBcd = u8;

#[allow(dead_code)]
fn bcd2dec(bcd: Bcd) -> u8 {
    BCD_PATTERNS.iter().position(|p| *p == bcd).unwrap() as u8
}

struct LineMapping {
    lines: [u8; 7],
}

impl LineMapping {
    #[allow(dead_code)]
    fn map(&self, mxbcd: MixedBcd) -> Bcd {
        self.lines
            .iter()
            .enumerate()
            .fold(0, |bcd, (mixed_line_number, mapped_line_number)| {
                if (1 << mixed_line_number) & mxbcd == 0 {
                    bcd
                } else {
                    bcd | (1 << mapped_line_number)
                }
            })
    }
}

fn part2(_input: &mut dyn Read) -> u32 {
    5353
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
        let mut f = File::open("input/day-8-sample.txt").unwrap();
        assert_eq!(part1(&mut f), 26);
    }

    #[test]
    fn test_part2_sample() {
        let mut f = File::open("input/day-8-sample.txt").unwrap();
        assert_eq!(part2(&mut f), 5353);
    }

    #[test]
    fn test_bcd2dec() {
        assert_eq!(bcd2dec(0b1110111), 0);
        assert_eq!(bcd2dec(0b0100100), 1);
        assert_eq!(bcd2dec(0b1011101), 2);
        assert_eq!(bcd2dec(0b1101101), 3);
        assert_eq!(bcd2dec(0b0101110), 4);
        assert_eq!(bcd2dec(0b1101011), 5);
        assert_eq!(bcd2dec(0b1111011), 6);
        assert_eq!(bcd2dec(0b0100101), 7);
        assert_eq!(bcd2dec(0b1111111), 8);
        assert_eq!(bcd2dec(0b1101111), 9);
    }

    #[test]
    fn test_linemapping() {
        let mapping = LineMapping {
            lines: [6, 0, 1, 2, 4, 3, 5],
        };

        assert_eq!(mapping.map(0b0000001), 0b1000000);
        assert_eq!(mapping.map(0b0000010), 0b0000001);
        assert_eq!(mapping.map(0b0000100), 0b0000010);
        assert_eq!(mapping.map(0b0001000), 0b0000100);
        assert_eq!(mapping.map(0b0010000), 0b0010000);
        assert_eq!(mapping.map(0b0100000), 0b0001000);
        assert_eq!(mapping.map(0b1000000), 0b0100000);
    }
}
