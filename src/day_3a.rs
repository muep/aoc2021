use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

fn find_gamma_epsilon(input: &mut dyn Read) -> (u32, u32) {
    let (cnt, bit_stats) =
        BufReader::new(input)
            .lines()
            .fold((0u16, [0u16; 12]), |(cnt, stats), line| {
                let mut stats_out = stats;
                for (c, stat) in line
                    .unwrap()
                    .as_bytes()
                    .iter()
                    .rev()
                    .zip(stats_out.iter_mut())
                {
                    if *c == '1' as u8 {
                        *stat += 1;
                    }
                }
                (cnt + 1, stats_out)
            });

    let threshold = cnt / 2;

    bit_stats
        .iter()
        .enumerate()
        .fold((0, 0), |(gamma, epsilon), (n, stat)| {
            let added_bit = 1 << n;
            if *stat == 0 {
                (gamma, epsilon)
            } else if *stat < threshold {
                (gamma, epsilon | added_bit)
            } else {
                (gamma | added_bit, epsilon)
            }
        })
}

pub fn run(input: &mut dyn Read) {
    let (gamma, epsilon) = find_gamma_epsilon(input);
    println!("{} * {} -> {}", gamma, epsilon, gamma * epsilon);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        use std::fs::File;
        let mut f = File::open("input/day-3-sample.txt").unwrap();
        let (gamma, epsilon) = find_gamma_epsilon(&mut f);

        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
        assert_eq!(gamma * epsilon, 198);
    }
}
