use std::collections::HashSet;
use std::io::Read;

#[derive(Clone, Copy)]
struct Size {
    cols: usize,
    rows: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    col: usize,
    row: usize,
}

fn load(input: &mut dyn Read) -> (usize, Vec<u8>) {
    use std::io::{BufRead, BufReader};
    use std::iter::once;

    let mut lines = BufReader::new(input).lines();
    let line1 = lines.next().unwrap().unwrap();
    let pitch = line1.len();
    let nums = once(line1)
        .chain(lines.map(|l| l.unwrap()))
        .map(|l| {
            let buf: Vec<u8> = l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
            buf
        })
        .flatten()
        .collect();
    (pitch, nums)
}

fn neighbors(size: Size, point: Position) -> Vec<Position> {
    let mut out = Vec::with_capacity(4);

    if point.row > 0 {
        out.push(Position {
            col: point.col,
            row: point.row - 1,
        });
    }
    if point.col > 0 {
        out.push(Position {
            col: point.col - 1,
            row: point.row,
        });
    }

    if point.row < size.rows - 1 {
        out.push(Position {
            col: point.col,
            row: point.row + 1,
        });
    }
    if point.col < size.cols - 1 {
        out.push(Position {
            col: point.col + 1,
            row: point.row,
        });
    }

    out
}

fn low_points(cols: usize, nums: &[u8]) -> Vec<Position> {
    let size = Size {
        cols: cols,
        rows: nums.len() / cols,
    };

    nums.iter()
        .enumerate()
        .map(|(pos, height)| {
            (
                Position {
                    row: pos / size.cols,
                    col: pos % size.cols,
                },
                *height,
            )
        })
        .filter(|(position, height)| {
            neighbors(size, *position)
                .iter()
                .all(|n| height_at(cols, nums, *n) > *height)
        })
        .map(|(p, _)| p)
        .collect()
}

fn part1(input: &mut dyn Read) -> u32 {
    let (cols, nums) = load(input);

    low_points(cols, &nums)
        .iter()
        .map(|p| (height_at(cols, &nums, *p) + 1) as u32)
        .sum()
}

fn higher_neighbors(size: Size, nums: &[u8], point: Position) -> HashSet<Position> {
    let all_neighbors = neighbors(size, point);
    let local_height = height_at(size.cols, nums, point);

    all_neighbors
        .into_iter()
        .filter(|p| {
            let h = height_at(size.cols, nums, *p);
            local_height < h && h < 9
        })
        .collect()
}

fn height_at(cols: usize, nums: &[u8], p: Position) -> u8 {
    nums[p.col + cols * p.row]
}

fn basin(cols: usize, nums: &[u8], low: Position) -> Vec<Position> {
    let size = Size {
        cols: cols,
        rows: nums.len() / cols,
    };

    let mut points = HashSet::new();
    points.insert(low);
    let mut to_check = higher_neighbors(size, nums, low);

    while !to_check.is_empty() {
        /* Take one unchecked point */
        let pos = *to_check.iter().next().unwrap();

        /* Add to the set of output points */
        points.insert(pos);

        /* Remove from to be checked */
        to_check.remove(&pos);

        /* Add neighbors, except for ones that were already selected */
        to_check.extend(
            higher_neighbors(size, nums, pos)
                .iter()
                .filter(|p| !points.contains(p)),
        );
    }

    points.into_iter().collect()
}

fn part2(input: &mut dyn Read) -> u32 {
    let (cols, nums) = load(input);

    let mut basins: Vec<Vec<Position>> = low_points(cols, &nums)
        .into_iter()
        .map(|p| basin(cols, &nums, p))
        .collect();
    basins.sort_by_key(|b| b.len() as isize * -1);
    basins
        .iter()
        .take(3)
        .map(|b| b.len() as u32)
        .fold(1, |acc, l| acc * l)
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
        let mut f = File::open("input/day-9-sample.txt").unwrap();
        assert_eq!(part1(&mut f), 15);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/day-9.txt").unwrap();
        assert_eq!(part1(&mut f), 522);
    }

    #[test]
    fn test_part2_sample() {
        let mut f = File::open("input/day-9-sample.txt").unwrap();
        assert_eq!(part2(&mut f), 1134);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/day-9.txt").unwrap();
        assert_eq!(part2(&mut f), 916688);
    }
}
