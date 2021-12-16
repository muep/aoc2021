use std::collections::BTreeSet;
use std::io::Read;

const UNKNOWN: u32 = u32::MAX;

struct Cell {
    local_risk: u8,
    cumulative_risk: u32,
    previous: usize,
}

struct Map {
    cells: Vec<Cell>,
    cols: usize,
}

fn load_grid(input: &mut dyn Read) -> (usize, Vec<u8>) {
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

fn load(input: &mut dyn Read) -> Map {
    let (cols, risks) = load_grid(input);
    let cells = risks
        .into_iter()
        .map(|local_risk| Cell {
            local_risk,
            cumulative_risk: UNKNOWN,
            previous: 0,
        })
        .collect();

    Map { cells, cols }
}

fn neighbors4(cols: usize, rows: usize, pos: usize) -> Vec<usize> {
    let col = pos % cols;
    let row = pos / cols;

    let with_left = col > 0;
    let with_right = col < cols - 1;
    let with_top = row > 0;
    let with_bottom = row < rows - 1;

    let mut res = Vec::with_capacity(4);
    if with_top {
        res.push(offset(cols, row - 1, col));
    }
    if with_left {
        res.push(offset(cols, row, col - 1));
    }
    if with_right {
        res.push(offset(cols, row, col + 1));
    }
    if with_bottom {
        res.push(offset(cols, row + 1, col));
    }
    res
}

fn offset(cols: usize, row: usize, col: usize) -> usize {
    col + cols * row
}

fn part1(input: &mut dyn Read) -> u32 {
    let mut map = load(input);
    let cols = map.cols;
    let rows = map.cells.len() / cols;

    let start = 0;
    let target = map.cells.len() - 1;

    let mut visited: BTreeSet<usize> = BTreeSet::new();

    let mut to_check: BTreeSet<(u32, usize)> = BTreeSet::new();
    to_check.insert((0, start));

    while let Some(cell) = to_check.iter().map(|c| *c).next() {
        let (local_cumulative, here) = cell;
        //println!("Checking cell {} with risk {}", here, local_cumulative);

        // No need to check this cell again
        to_check.remove(&cell);
        visited.insert(here);

        let neighbors: BTreeSet<(u32, usize)> = neighbors4(cols, rows, here)
            .into_iter()
            .filter(|p| !visited.contains(p))
            .map(|p| (local_cumulative + map.cells[p].local_risk as u32, p))
            .collect();

        for (cumulative, pos) in neighbors.iter().cloned() {
            //println!("Recording risk({}) = {}", pos, cumulative);
            map.cells[pos].cumulative_risk = u32::min(cumulative, map.cells[pos].cumulative_risk);
            map.cells[pos].previous = here;
        }

        //println!("Adding {:?} to be checked", neighbors);

        to_check = to_check.union(&neighbors).cloned().collect();
    }

    map.cells[target].cumulative_risk
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
        let mut f = File::open("input/day-15-sample.txt").unwrap();
        assert_eq!(part1(&mut f), 40);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/day-15.txt").unwrap();
        assert_eq!(part1(&mut f), 707);
    }

    #[test]
    fn test_part2_sample() {
        let mut f = File::open("input/day-15-sample.txt").unwrap();
        assert_eq!(part2(&mut f), 0);
    }
}
