use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

enum Param {
    Ogr,
    Csr,
}

fn bits_in_use(v: &u16) -> u8 {
    for n in 0..16 {
        if *v < (1 << n) {
            return n;
        }
    }
    return 16;
}

fn bit_at_pos(bit_pos: u8, num: u16) -> bool {
    (num >> bit_pos) & 1 != 0
}

fn most_common_at_pos(bit_pos: u8, nums: &[u16]) -> bool {
    let one_cnt = nums
        .iter()
        .filter(|n| bit_at_pos(bit_pos, **n) == true)
        .count();
    let zero_cnt = nums.len() - one_cnt;

    if zero_cnt <= one_cnt {
        true
    } else {
        false
    }
}

fn find(param: Param, input: Vec<u16>, bits: u8) -> u16 {
    let mut remaining = input;

    for b in (0..bits).rev() {
        let most_common_at_b = most_common_at_pos(b, &remaining);
        let least_common_at_b = !most_common_at_b;

        let target = match param {
            Param::Ogr => most_common_at_b,
            Param::Csr => least_common_at_b,
        };

        remaining = remaining
            .into_iter()
            .filter(|n| bit_at_pos(b, *n) == target)
            .collect();

        if remaining.len() <= 1 {
            break;
        }
    }
    return *remaining.first().unwrap();
}

fn find_ogr_csr(input: &mut dyn Read) -> (u32, u32) {
    let numbers: Vec<u16> = BufReader::new(input)
        .lines()
        .map(|l| u16::from_str_radix(&l.unwrap(), 2).unwrap())
        .collect();

    let bits = numbers.iter().map(bits_in_use).max().unwrap();
    let ogr = find(Param::Ogr, numbers.clone(), bits);
    let csr = find(Param::Csr, numbers, bits);

    (ogr as u32, csr as u32)
}

pub fn run(input: &mut dyn Read) {
    let (ogr, csr) = find_ogr_csr(input);
    println!("{} * {} -> {}", ogr, csr, ogr * csr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_at_pos() {
        assert_eq!(bit_at_pos(0, 1), true);
        assert_eq!(bit_at_pos(1, 1), false);

        assert_eq!(bit_at_pos(0, 5), true);
        assert_eq!(bit_at_pos(1, 5), false);
        assert_eq!(bit_at_pos(2, 5), true);
    }

    #[test]
    fn test_bits_in_use() {
        assert_eq!(bits_in_use(&1), 1);
        assert_eq!(bits_in_use(&3), 2);
        assert_eq!(bits_in_use(&4), 3);
        assert_eq!(bits_in_use(&31), 5);
        assert_eq!(bits_in_use(&32), 6);
        assert_eq!(bits_in_use(&4095), 12);
        assert_eq!(bits_in_use(&4096), 13);
    }

    #[test]
    fn test_example() {
        use std::fs::File;
        let mut f = File::open("input/day-3-sample.txt").unwrap();
        let (ogr, csr) = find_ogr_csr(&mut f);

        assert_eq!(ogr, 23);
        assert_eq!(csr, 10);
        assert_eq!(ogr * csr, 230);
    }

    #[test]
    fn test_full() {
        use std::fs::File;
        let mut f = File::open("input/day-3.txt").unwrap();
        let (ogr, csr) = find_ogr_csr(&mut f);

        assert_eq!(ogr, 781);
        assert_eq!(csr, 2734);
        assert_eq!(ogr * csr, 2135254);
    }
}
