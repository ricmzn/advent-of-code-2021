use std::{error::Error, fs::read_to_string};

pub fn run() -> Result<(), Box<dyn Error>> {
    let data = read_to_string("day4.txt")?;
    println!(
        "day 4, part 1: first bingo winner score: {}",
        get_first_winner_score(&data)
    );
    println!(
        "day 4, part 2: last bingo winner score: {}",
        get_last_winner_score(&data)
    );
    Ok(())
}

#[derive(Debug)]
struct BingoCell {
    number: i32,
    marked: bool,
}

#[derive(Debug, Default)]
struct BingoBoard {
    numbers: Vec<BingoCell>,
    score: Option<i32>,
}

impl BingoBoard {
    fn get_cell(&self, line: usize, column: usize) -> &BingoCell {
        &self.numbers[line * 5 + column]
    }

    fn mark(&mut self, number: i32) {
        for cell in &mut self.numbers {
            if cell.number == number {
                cell.marked = true;
            }
        }
        if self.is_winner() && self.score.is_none() {
            self.score = Some(self.unmarked_sum() * number);
        }
    }

    fn is_winner(&self) -> bool {
        let mut lines = [0; 5];
        let mut columns = [0; 5];
        for line in 0..5 {
            for column in 0..5 {
                if self.get_cell(line, column).marked {
                    lines[line] += 1;
                    columns[column] += 1;
                }
            }
            if lines.contains(&5) || columns.contains(&5) {
                return true;
            }
        }
        false
    }

    fn unmarked_sum(&self) -> i32 {
        self.numbers
            .iter()
            .filter(|cell| cell.marked == false)
            .map(|cell| cell.number)
            .sum()
    }

    fn validate(&self) {
        assert_eq!(self.numbers.len(), 25);
    }
}

fn get_bingo_game(game: &str) -> (Vec<i32>, Vec<BingoBoard>) {
    let mut lines = game.trim().lines();
    let calls: Vec<i32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    let mut current_board = BingoBoard::default();
    for line in lines {
        if line.trim().is_empty() && current_board.numbers.len() > 0 {
            current_board.validate();
            boards.push(current_board);
            current_board = BingoBoard::default();
        } else {
            current_board
                .numbers
                .extend(line.split_whitespace().map(|num| BingoCell {
                    number: num.parse().unwrap(),
                    marked: false,
                }));
        }
    }
    current_board.validate();
    boards.push(current_board);

    (calls, boards)
}

fn get_first_winner_score(game: &str) -> i32 {
    let (calls, mut boards) = get_bingo_game(game);
    for call in calls {
        for board in boards.iter_mut() {
            board.mark(call);
            if let Some(score) = board.score {
                return score;
            }
        }
    }
    panic!("No winner found")
}

fn get_last_winner_score(game: &str) -> i32 {
    let (calls, mut boards) = get_bingo_game(game);
    let mut winners = Vec::new();
    for call in calls {
        for (id, board) in boards.iter_mut().enumerate() {
            board.mark(call);
            if board.score.is_some() && !winners.contains(&id) {
                winners.push(id);
            }
        }
    }
    let last_winner = *winners.last().expect("No winner found");
    boards[last_winner].score.unwrap()
}

#[cfg(test)]
const INPUT: &str = r"
    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
    8  2 23  4 24
    21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19

    3 15  0  2 22
    9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
    2  0 12  3  7
";

#[test]
fn part1() {
    assert_eq!(get_first_winner_score(INPUT), 4512);
}

#[test]
fn part2() {
    assert_eq!(get_last_winner_score(INPUT), 1924);
}
