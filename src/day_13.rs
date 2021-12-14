use std::collections::HashSet;
use std::io::Read;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    row: u16,
    col: u16,
}

#[derive(Clone, Copy, Debug)]
enum Fold {
    X(u16),
    Y(u16),
}

fn load(input: &mut dyn Read) -> (Vec<Point>, Vec<Fold>) {
    use std::io::{BufRead, BufReader};

    let (points, folds): (Vec<Option<Point>>, Vec<Option<Fold>>) = BufReader::new(input)
        .lines()
        .map(|l| {
            let ln = l.unwrap();
            let (point, fold) = if ln.is_empty() {
                (None, None)
            } else if let Some(num) = ln.strip_prefix("fold along x=") {
                (None, Some(Fold::X(num.parse().unwrap())))
            } else if let Some(num) = ln.strip_prefix("fold along y=") {
                (None, Some(Fold::Y(num.parse().unwrap())))
            } else {
                let mut pieces = ln.split(',');
                let col_text = pieces.next().unwrap();
                let row_text = pieces.next().unwrap();
                (
                    Some(Point {
                        row: row_text.parse().unwrap(),
                        col: col_text.parse().unwrap(),
                    }),
                    None,
                )
            };
            (point, fold)
        })
        .unzip();
    (
        points.into_iter().flatten().collect(),
        folds.into_iter().flatten().collect(),
    )
}

fn fold_scalar(a: u16, fold_at: u16) -> u16 {
    if a <= fold_at {
        a
    } else {
        2 * fold_at - a
    }
}

fn fold(points: Vec<Point>, f: Fold) -> Vec<Point> {
    let res: HashSet<Point> = points
        .into_iter()
        .map(|Point { col, row }| match f {
            Fold::Y(y) => Point {
                col,
                row: fold_scalar(row, y),
            },
            Fold::X(x) => Point {
                col: fold_scalar(col, x),
                row,
            },
        })
        .collect();
    res.into_iter().collect()
}

fn part1(input: &mut dyn Read) -> u32 {
    let (points, folds) = load(input);

    fold(points, folds[0]).len() as u32
}

fn part2(input: &mut dyn Read) -> HashSet<Point> {
    let (points, folds) = load(input);
    folds
        .into_iter()
        .fold(points, |points, f| fold(points, f))
        .into_iter()
        .collect()
}

pub fn run_part1(input: &mut dyn Read) {
    println!("{}", part1(input));
}

pub fn run_part2(input: &mut dyn Read) {
    let points = part2(input);
    for row in 0..6 {
        for col in 0..39 {
            let c = if points.contains(&Point { col, row }) {
                '#'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_part1_sample() {
        let mut f = File::open("input/day-13-sample.txt").unwrap();
        assert_eq!(part1(&mut f), 17);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/day-13.txt").unwrap();
        assert_eq!(part1(&mut f), 695);
    }

    #[test]
    fn test_part2_sample() {
        let mut f = File::open("input/day-13-sample.txt").unwrap();
        assert_eq!(part2(&mut f).len(), 16);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/day-13.txt").unwrap();
        assert_eq!(part2(&mut f).len(), 89);
    }
}
