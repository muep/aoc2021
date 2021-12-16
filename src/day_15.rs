use std::io::Read;

const UNKNOWN: u32 = u32::MAX;

struct Cell {
    local_risk: u8,
    cumulative_risk: u32,
    previous: usize,
    visited: bool,
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
            visited: false,
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

fn get_route(map: &Map, start: usize, dest: usize) -> Vec<usize> {
    let mut res = vec![dest];
    let mut here = dest;

    while here != start {
        here = map.cells[here].previous;
        res.push(here);
    }

    res.reverse();
    res
}

fn total_risk(mut map: Map) -> (u32, Vec<usize>) {
    let cols = map.cols;
    let rows = map.cells.len() / cols;

    let start = 0;
    let target = map.cells.len() - 1;

    let mut to_check: Vec<usize> = vec![start];
    map.cells[start].cumulative_risk = 0;

    while let Some(here) = to_check.first().cloned() {
        let local_cumulative = map.cells[here].cumulative_risk;

        map.cells[here].visited = true;
        let neighbors: Vec<usize> = neighbors4(cols, rows, here)
            .into_iter()
            .filter(|p| !map.cells[*p].visited)
            .collect();

        for n in neighbors.iter().cloned() {
            let n_cumulative = local_cumulative + map.cells[n].local_risk as u32;

            map.cells[n].cumulative_risk = u32::min(map.cells[n].cumulative_risk, n_cumulative);
            map.cells[n].previous = here;
            if n == target {
                return (n_cumulative, get_route(&map, start, target));
            }
        }

        to_check = to_check
            .into_iter()
            .filter(|p| *p != here)
            .chain(neighbors.into_iter())
            .collect();
        to_check.sort_by_key(|p| map.cells[*p].cumulative_risk);
    }

    (
        map.cells[target].cumulative_risk,
        get_route(&map, start, target),
    )
}

fn part1(input: &mut dyn Read) -> (u32, Vec<usize>) {
    let map = load(input);
    total_risk(map)
}

fn part2(_: &mut dyn Read) -> u32 {
    0
}

pub fn run_part1(input: &mut dyn Read) {
    let (risk, route) = part1(input);
    println!("total risk {}, going through {:?}", risk, route);
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
        assert_eq!(part1(&mut f).0, 40);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/day-15.txt").unwrap();
        assert_eq!(part1(&mut f).0, 707);
    }

    #[test]
    fn test_part2_sample() {
        let mut f = File::open("input/day-15-sample.txt").unwrap();
        assert_eq!(part2(&mut f), 0);
    }
}
