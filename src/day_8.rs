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

fn bcd2dec(bcd: Bcd) -> u8 {
    BCD_PATTERNS.iter().position(|p| *p == bcd).unwrap() as u8
}

fn the_only_enabled_line(lines: u8) -> Option<u8> {
    let mut found = None;

    for line in 0..7 {
        if (1 << line) & lines != 0 {
            if found.is_some() {
                return None;
            }
            found = Some(line);
        }
    }
    found
}

struct LineMapping {
    lines: [u8; 7],
}

impl LineMapping {
    fn map(&self, input: u8) -> u8 {
        self.lines
            .iter()
            .enumerate()
            .fold(0, |bcd, (input_line_number, mapped_line_mask)| {
                if (1 << input_line_number) & input == 0 {
                    bcd
                } else {
                    bcd | mapped_line_mask
                }
            })
    }

    fn with_line_order(line_order: [u8; 7]) -> LineMapping {
        LineMapping {
            lines: [
                1 << line_order[0],
                1 << line_order[1],
                1 << line_order[2],
                1 << line_order[3],
                1 << line_order[4],
                1 << line_order[5],
                1 << line_order[6],
            ],
        }
    }
}

enum Known {
    Single(u8),
    OneOf([u8; 3]),
}

impl Known {
    fn from_bits_enabled(bits_enabled: usize) -> Known {
        match bits_enabled {
            2 => Known::Single(1),
            3 => Known::Single(7),
            4 => Known::Single(4),
            5 => Known::OneOf([2, 3, 5]),
            6 => Known::OneOf([0, 6, 9]),
            7 => Known::Single(8),
            _ => panic!("Unexpected number of bits enabled: {}", bits_enabled),
        }
    }
}

struct PossibleMappings {
    /* Mapping from input lines to ones that may be their corresponding outputs */
    input_lines: [u8; 7],
}

impl PossibleMappings {
    fn anything() -> PossibleMappings {
        PossibleMappings {
            input_lines: [0x7f; 7],
        }
    }

    fn insert_bcd(&mut self, input_signal: Bcd) {
        let bits_enabled = (0..7).filter(|n| (1 << n) & input_signal != 0).count();
        match Known::from_bits_enabled(bits_enabled) {
            Known::Single(n) => {
                /* Only one corresponding number */
                let output_signals = BCD_PATTERNS[n as usize];

                for input_line in 0..7 {
                    let input_enabled = (1 << input_line) & input_signal != 0;
                    if input_enabled {
                        self.input_lines[input_line] &= output_signals;
                    } else {
                        self.input_lines[input_line] &= !output_signals;
                    }
                }
            }
            Known::OneOf(nums) => {
                let (known_enabled, known_disabled): (u8, u8) =
                    nums.iter().fold((0x7f, 0x7f), |(enabled, disabled), n| {
                        (
                            enabled & BCD_PATTERNS[*n as usize],
                            disabled & !BCD_PATTERNS[*n as usize],
                        )
                    });

                assert!(known_enabled & known_disabled == 0);
                for input_line in 0..7 {
                    let input_enabled = (1 << input_line) & input_signal != 0;
                    if input_enabled {
                        /* This input was enabled, so can't be mapped
                         * to the ones that are sure to be off */
                        self.input_lines[input_line] &= !known_disabled;
                    } else {
                        self.input_lines[input_line] &= !known_enabled;
                    }
                }
            }
        }

        let mut should_recheck = true;
        while should_recheck {
            should_recheck = false;

            for input_line in 0..7 {
                let possible_outputs = self.input_lines[input_line];
                if the_only_enabled_line(possible_outputs).is_none() {
                    continue;
                }

                for other_input_line in (0..7).filter(|n| *n != input_line) {
                    if self.input_lines[other_input_line] & possible_outputs == 0 {
                        continue;
                    }

                    self.input_lines[other_input_line] &= !possible_outputs;

                    /* Something got disabled, so let's recheck */
                    should_recheck = true;
                }
            }
        }
    }

    fn mapping(&self) -> LineMapping {
        LineMapping::with_line_order([
            the_only_enabled_line(self.input_lines[0]).unwrap(),
            the_only_enabled_line(self.input_lines[1]).unwrap(),
            the_only_enabled_line(self.input_lines[2]).unwrap(),
            the_only_enabled_line(self.input_lines[3]).unwrap(),
            the_only_enabled_line(self.input_lines[4]).unwrap(),
            the_only_enabled_line(self.input_lines[5]).unwrap(),
            the_only_enabled_line(self.input_lines[6]).unwrap(),
        ])
    }
}

fn pattern2bcd(pattern: &str) -> Bcd {
    /* This gives a scrambled result */
    pattern.chars().fold(0, |bits, c| match c {
        'a' => bits | 1,
        'b' => bits | 2,
        'c' => bits | 4,
        'd' => bits | 8,
        'e' => bits | 16,
        'f' => bits | 32,
        'g' => bits | 64,
        a => panic!("Unexpected char {} in pattern {}", a, pattern),
    })
}

fn part2_line(line: &str) -> u32 {
    let (patterns, nums): (Vec<&str>, &str) = {
        let mut pieces = line.split(" | ");
        let mut patterns: Vec<&str> = pieces.next().unwrap().split_whitespace().collect();
        patterns.sort_by_key(|p| p.len());
        let nums = pieces.next().unwrap();
        (patterns, nums)
    };

    let mut possibles = PossibleMappings::anything();

    for num in patterns.into_iter().map(pattern2bcd) {
        possibles.insert_bcd(num);
    }

    let mapping = possibles.mapping();

    nums.split_whitespace()
        .map(pattern2bcd)
        .map(|n| bcd2dec(mapping.map(n)))
        .fold(0, |old, digit| old * 10 + digit as u32)
}

fn part2(input: &mut dyn Read) -> u32 {
    use std::io::{BufRead, BufReader};

    BufReader::new(input)
        .lines()
        .map(|l| part2_line(l.unwrap().as_str()))
        .sum()
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
        assert_eq!(part2(&mut f), 61229);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/day-8.txt").unwrap();
        assert_eq!(part1(&mut f), 383);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/day-8.txt").unwrap();
        assert_eq!(part2(&mut f), 998900);
    }

    #[test]
    fn test_part2_line() {
        assert_eq!(part2(&mut "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".as_bytes()), 5353);
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
        let mapping = LineMapping::with_line_order([6, 0, 1, 2, 4, 3, 5]);

        assert_eq!(mapping.map(0b0000001), 0b1000000);
        assert_eq!(mapping.map(0b0000010), 0b0000001);
        assert_eq!(mapping.map(0b0000100), 0b0000010);
        assert_eq!(mapping.map(0b0001000), 0b0000100);
        assert_eq!(mapping.map(0b0010000), 0b0010000);
        assert_eq!(mapping.map(0b0100000), 0b0001000);
        assert_eq!(mapping.map(0b1000000), 0b0100000);

        /* Some combinations as well */
        assert_eq!(mapping.map(0b0011000), 0b0010100);
        assert_eq!(mapping.map(0b0011010), 0b0010101);
        assert_eq!(mapping.map(0b1111111), 0b1111111);
    }

    #[test]
    fn test_possible_mappings() {
        /* Mapping where we especially map lines 2,5 - i.e. c and f -
         * to the first two lines. And the 0/a line is in the 2/c slot */
        let mapping = LineMapping::with_line_order([2, 3, 1, 6, 4, 0, 5]);

        let mut possibles = PossibleMappings::anything();

        possibles.insert_bcd(mapping.map(BCD_PATTERNS[4]));
        possibles.insert_bcd(mapping.map(BCD_PATTERNS[1]));
        possibles.insert_bcd(mapping.map(BCD_PATTERNS[7]));

        /* Should have pretty trivial behavior so far. Especially
         * should know that the third line is 0/a */
        assert_eq!(possibles.input_lines[0], 0b0100100);
        assert_eq!(possibles.input_lines[1], 0b0100100);
        assert_eq!(possibles.input_lines[2], 0b0000001);
        assert_eq!(possibles.input_lines[3], 0b0001010);
        assert_eq!(possibles.input_lines[4], 0b1010000);
        assert_eq!(possibles.input_lines[5], 0b1010000);
        assert_eq!(possibles.input_lines[6], 0b0001010);

        possibles.insert_bcd(mapping.map(BCD_PATTERNS[2]));
        possibles.insert_bcd(mapping.map(BCD_PATTERNS[3]));
        possibles.insert_bcd(mapping.map(BCD_PATTERNS[5]));

        assert_eq!(possibles.input_lines[0], 0b0100100);
        assert_eq!(possibles.input_lines[1], 0b0100100);
        assert_eq!(possibles.input_lines[2], 0b0000001);
        assert_eq!(possibles.input_lines[3], 0b0000010);
        assert_eq!(possibles.input_lines[4], 0b0010000);
        assert_eq!(possibles.input_lines[5], 0b1000000);
        assert_eq!(possibles.input_lines[6], 0b0001000);

        possibles.insert_bcd(mapping.map(BCD_PATTERNS[0]));
        possibles.insert_bcd(mapping.map(BCD_PATTERNS[6]));
        possibles.insert_bcd(mapping.map(BCD_PATTERNS[9]));

        assert_eq!(possibles.input_lines[0], 0b0100000);
        assert_eq!(possibles.input_lines[1], 0b0000100);
        assert_eq!(possibles.input_lines[2], 0b0000001);
        assert_eq!(possibles.input_lines[3], 0b0000010);
        assert_eq!(possibles.input_lines[4], 0b0010000);
        assert_eq!(possibles.input_lines[5], 0b1000000);
        assert_eq!(possibles.input_lines[6], 0b0001000);

        let rev_mapping = possibles.mapping();
        for n in 0..128 {
            assert_eq!(n, rev_mapping.map(mapping.map(n)));
        }
    }
}
