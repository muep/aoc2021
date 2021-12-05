use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn from_text(txt: &str) -> Point {
        let mut pieces = txt.split(',');
        Point {
            x: pieces.next().unwrap().parse().unwrap(),
            y: pieces.next().unwrap().parse().unwrap(),
        }
    }
}

struct Line(Point, Point);

impl Line {
    fn from_text(ln: &str) -> Line {
        let mut point_parts = ln.split(" -> ");
        Line(
            Point::from_text(point_parts.next().unwrap()),
            Point::from_text(point_parts.next().unwrap()),
        )
    }
}

fn ascending<T>(a: T, b: T) -> (T, T)
where
    T: Ord,
{
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

fn part1(input: &mut dyn Read) -> u32 {
    let pitch = 1024;
    let buf_sz = 1048576;
    let mut buf: Vec<u16> = Vec::new();
    buf.resize(buf_sz, 0);

    for Line(p0, p1) in BufReader::new(input)
        .lines()
        .map(|l| Line::from_text(&l.unwrap()))
    {
        if p0.x == p1.x {
            /* Vertical line */
            let x = p0.x;
            let (y_min, y_max) = ascending(p0.y, p1.y);
            for y in y_min..=y_max {
                buf[x as usize + pitch * (y as usize)] += 1;
            }
        } else if p0.y == p1.y {
            /* Horizontal line */
            let y = p0.y;
            let (x_min, x_max) = ascending(p0.x, p1.x);
            for x in x_min..=x_max {
                buf[x as usize + pitch * (y as usize)] += 1;
            }
        }
    }

    buf.into_iter().filter(|n| *n > 1).count() as u32
}

pub fn run_part1(input: &mut dyn Read) {
    println!("{}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_point() {
        let pt = Point::from_text("121,396");
        assert_eq!(pt.x, 121);
        assert_eq!(pt.y, 396);
    }

    #[test]
    fn test_line() {
        let Line(p0, p1) = Line::from_text("700,793 -> 700,892");
        assert_eq!(p0.x, 700);
        assert_eq!(p1.y, 892);
    }

    #[test]
    fn test_part1_sample() {
        let mut f = File::open("input/day-5-sample.txt").unwrap();
        let res = part1(&mut f);
        assert_eq!(res, 5);
    }
}
