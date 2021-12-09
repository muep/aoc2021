use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

type Board = [u8; 25];

fn nums_and_boards(input: &mut dyn Read) -> (Vec<u8>, Vec<Board>) {
    let mut lines = BufReader::new(input).lines();

    let nums: Vec<u8> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    loop {
        if lines.next().is_none() {
            break;
        }

        let mut board = [0; 25];
        for row in 0..5 {
            let line_text = match lines.next() {
                Some(Ok(l)) => l,
                _ => {
                    break;
                }
            };

            for (col, num_text) in line_text.split_whitespace().enumerate() {
                board[5 * row + col] = num_text.parse().unwrap();
            }
        }
        boards.push(board);
    }

    (nums, boards)
}

#[derive(Clone, Copy)]
struct Victory {
    sum: u16,
    last_num: u8,
}

#[derive(Clone, Copy)]
struct Bingo {
    board: Board,
    last_num: u8,
    marks: u32,
}

impl Bingo {
    fn with_board(board: Board) -> Bingo {
        Bingo {
            board: board,
            last_num: 0,
            marks: 0,
        }
    }

    fn draw(mut self, num: u8) -> Bingo {
        self.last_num = num;
        let pos = match self.board.iter().position(|n| *n == num) {
            Some(p) => p,
            None => {
                return self;
            }
        };

        self.marks |= 1 << pos;
        self
    }

    fn sum_of_unmarked(&self) -> u16 {
        self.board
            .into_iter()
            .enumerate()
            .filter(|(pos, _)| self.marks & 1 << pos == 0)
            .map(|(_, num)| num as u16)
            .sum()
    }

    fn victory(&self) -> Option<Victory> {
        if victory(self.marks) {
            Some(Victory {
                sum: self.sum_of_unmarked(),
                last_num: self.last_num,
            })
        } else {
            None
        }
    }
}

fn find_worst_victory(input: &mut dyn Read) -> Victory {
    let (nums, boards) = nums_and_boards(input);

    let mut bingos: Vec<Bingo> = boards
        .into_iter()
        .map(|board| Bingo::with_board(board))
        .collect();

    for num in nums.into_iter() {
        let (winners, in_progress): (Vec<Bingo>, Vec<Bingo>) = bingos
            .into_iter()
            .map(|b| b.draw(num))
            .partition(|b| b.victory().is_some());
        bingos = in_progress;

        if bingos.is_empty() {
            let mut victories: Vec<Victory> =
                winners.iter().map(|b| b.victory().unwrap()).collect();
            victories.sort_by_key(|Victory { sum, last_num }| *sum as u32 * *last_num as u32);
            return *victories.first().unwrap();
        }
    }

    panic!("No victories!")
}

fn victory_on_col(col: u8, marks: u32) -> bool {
    (0..5).all(|row| {
        let pos = 5 * row + col;
        0 != marks & (1 << pos)
    })
}

fn victory_on_row(row: u8, marks: u32) -> bool {
    let start = row * 5;
    let end = start + 5;
    (start..end).all(|pos| 0 != marks & (1 << pos))
}

fn victory(marks: u32) -> bool {
    (0..5).any(|col| victory_on_col(col, marks)) || (0..5).any(|row| victory_on_row(row, marks))
}

pub fn run(input: &mut dyn Read) {
    let Victory { sum, last_num } = find_worst_victory(input);
    println!("{} * {} -> {}", sum, last_num, sum as u32 * last_num as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_victorycond() {
        assert!(victory_on_row(1, 0b0000_00000_00000_11111_00000));
        assert!(!victory_on_row(1, 0b0000_00000_00000_10111_00000));
        assert!(victory_on_col(3, 0b01000_01000_01000_01000_01000));
        assert!(!victory_on_col(2, 0b01000_01000_01000_01000_01000));
    }

    #[test]
    fn test_example() {
        use std::fs::File;

        let mut f = File::open("input/day-4-sample.txt").unwrap();
        let Victory { sum, last_num } = find_worst_victory(&mut f);
        assert_eq!(sum, 148);
        assert_eq!(last_num, 13);
    }

    #[test]
    fn test_full() {
        use std::fs::File;

        let mut f = File::open("input/day-4.txt").unwrap();
        let Victory { sum, last_num } = find_worst_victory(&mut f);
        assert_eq!(sum, 258);
        assert_eq!(last_num, 88);
    }
}
