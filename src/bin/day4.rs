
use std::{ io::{self, BufRead}, fs::File };


#[derive(Debug, Default)]
struct PuzzleInput {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

#[derive(Debug, Default)]
struct Board {
    numbers: Vec<Vec<usize>>,
    marked: Vec<Vec<bool>>
}

impl Board {

    fn push_row(&mut self, numbers: Vec<usize>) {
        self.marked.push(
            numbers.iter().map(|_| false).collect()
        );
        self.numbers.push(numbers);
    }

    fn mark_number(&mut self, number: usize) {
        for (x, row) in self.numbers.iter().enumerate() {
            for (y, n) in row.iter().enumerate() {
                if *n == number {
                    self.marked[x][y] = true;
                }
            }
        }
    }

    fn get_score(&self, last_number: usize) -> usize {
        let mut sum = 0;
        for (x, row) in self.marked.iter().enumerate() {
            for (y, mark) in row.iter().enumerate() {
                if !mark {
                    sum += self.numbers[x][y];
                }
            }
        }

        sum * last_number
    }

    fn has_won(&self) -> bool {
        let mut col_wins = (0..self.marked[0].len())
            .map(|_| true)
            .collect::<Vec<_>>();

        for row in self.marked.iter() {
            let mut row_win = true;
            for index in 0..row.len() {
                col_wins[index] = col_wins[index] && row[index];
                row_win = row_win && row[index];
            }
            if row_win { return true; }
        }

        for col in col_wins {
            if col { return true; }
        }

        false
    }
}


fn parse_input() -> PuzzleInput {
    let f = File::open("input/day4.txt").unwrap();
    let mut lines = io::BufReader::new(f).lines();

    let numbers = lines.next()
        .unwrap().unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut boards : Vec<Board> = vec![];

    for line in lines {
        let line = line.unwrap();
        if line == "" {
            boards.push(Board::default());
            continue;
        }
        let numbers = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();

        let index = boards.len() - 1;
        boards[index].push_row(numbers);
    }


    PuzzleInput {
        numbers,
        boards
    }
}

fn part1() {
    let mut input = parse_input();

    // part 1
    for n in input.numbers.iter() {
        for (index, board) in input.boards.iter_mut().enumerate() {
            board.mark_number(*n);
            if board.has_won() {
                println!("Board {} has won with score {}!", index, board.get_score(*n));
                return
            }
        }
    }
}


fn main() {
    part1();

    // part 2
    let mut input = parse_input();
    for n in input.numbers.iter() {
        let mut index_to_remove = vec![];
        let boards_len = input.boards.len();
        for (index, board) in input.boards.iter_mut().enumerate() {
            board.mark_number(*n);
            if board.has_won() {
                index_to_remove.push(index);
                println!("Board {} has won with score {}!", index, board.get_score(*n));

                if boards_len == 1 {
                    println!("This was the last board!")
                }
            }
        }
        for i in index_to_remove.iter().rev() {
            input.boards.remove(*i);
        }
    }
}
