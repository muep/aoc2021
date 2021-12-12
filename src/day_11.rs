use std::io::Read;

struct World {
    nums: Vec<u8>,
    cols: usize,
    flashes: u32,
}

impl World {
    fn from_input(input: &mut dyn Read) -> World {
        use std::io::{BufRead, BufReader};

        let mut cols = 0;
        let mut nums = Vec::new();

        for line in BufReader::new(input).lines().map(|l| l.unwrap()) {
            cols = line.len();

            nums.extend(line.chars().map(|c| c.to_digit(10).unwrap() as u8))
        }

        World {
            nums,
            cols,
            flashes: 0,
        }
    }

    fn rows(&self) -> usize {
        self.nums.len() / self.cols
    }

    fn step(mut self) -> World {
        self.nums = self.nums.into_iter().map(|n| n + 1).collect();
        let mut to_flash: Vec<usize> = Vec::with_capacity(self.nums.len());

        loop {
            to_flash.clear();
            to_flash.extend(self.nums.iter().enumerate().filter_map(|(pos, energy)| {
                if *energy > 9 {
                    Some(pos)
                } else {
                    None
                }
            }));
            if to_flash.is_empty() {
                break;
            }

            for pos in to_flash.iter() {
                self.flashes += 1;
                self.nums[*pos] = 0;
                for neighbor in neighbors(self.cols, self.rows(), *pos) {
                    if self.nums[neighbor] > 0 {
                        self.nums[neighbor] += 1;
                    }
                }
            }
        }

        self
    }
}

fn neighbors(cols: usize, rows: usize, pos: usize) -> Vec<usize> {
    let col = pos % cols;
    let row = pos / cols;

    let with_left = col > 0;
    let with_right = col < cols - 1;
    let with_top = row > 0;
    let with_bottom = row < rows - 1;

    let mut res = Vec::with_capacity(8);
    if with_top {
        if with_left {
            res.push(offset(cols, row - 1, col - 1));
        }
        res.push(offset(cols, row - 1, col));
        if with_right {
            res.push(offset(cols, row - 1, col + 1));
        }
    }
    if with_left {
        res.push(offset(cols, row, col - 1));
    }
    if with_right {
        res.push(offset(cols, row, col + 1));
    }
    if with_bottom {
        if with_left {
            res.push(offset(cols, row + 1, col - 1));
        }
        res.push(offset(cols, row + 1, col));
        if with_right {
            res.push(offset(cols, row + 1, col + 1));
        }
    }
    res
}

fn offset(cols: usize, row: usize, col: usize) -> usize {
    col + cols * row
}

fn part1(input: &mut dyn Read) -> u32 {
    let world = (0..100).fold(World::from_input(input), |w, _| w.step());

    world.flashes
}

fn part2(_: &mut dyn Read) -> u32 {
    0
}

pub fn run_part1(input: &mut dyn Read) {
    println!("{}", part1(input));
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
        let mut f = File::open("input/day-11-sample.txt").unwrap();
        assert_eq!(part1(&mut f), 1656);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/day-11.txt").unwrap();
        assert_eq!(part1(&mut f), 1683);
    }

    #[test]
    fn test_part2_sample() {
        let mut f = File::open("input/day-11-sample.txt").unwrap();
        assert_eq!(part2(&mut f), 0);
    }
}
