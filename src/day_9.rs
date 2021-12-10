use std::io::Read;

#[derive(Clone, Copy)]
struct Size {
    cols: usize,
    rows: usize,
}

#[derive(Clone, Copy, Debug)]
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

fn neigh4(size: Size, point: Position) -> [Option<Position>; 4] {
    let mut out = [None; 4];
    let mut next = 0;

    if point.row > 0 {
        out[next] = Some(Position {
            col: point.col,
            row: point.row - 1,
        });
        next += 1;
    }
    if point.col > 0 {
        out[next] = Some(Position {
            col: point.col - 1,
            row: point.row,
        });
        next += 1;
    }

    if point.row < size.rows - 1 {
        out[next] = Some(Position {
            col: point.col,
            row: point.row + 1,
        });
        next += 1;
    }
    if point.col < size.cols - 1 {
        out[next] = Some(Position {
            col: point.col + 1,
            row: point.row,
        });
    }

    out
}

fn part1(input: &mut dyn Read) -> u32 {
    let (cols, nums) = load(input);

    let size = Size {
        cols: cols,
        rows: nums.len() / cols,
    };

    nums.iter()
        .enumerate()
        .map(|(pos, height)| {
            let position = Position {
                row: pos / size.cols,
                col: pos % size.cols,
            };

            let neighbors = neigh4(size, position);
            for neighbor in neighbors {
                let Position { col, row } = match neighbor {
                    Some(a) => a,
                    None => {
                        break;
                    }
                };

                let height_at_neighbor = nums[col + size.cols * row];
                if height_at_neighbor <= *height {
                    return 0;
                }
            }

            (*height + 1) as u32
        })
        .sum()
}

pub fn run_part1(input: &mut dyn Read) {
    println!("{}", part1(input));
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
}
