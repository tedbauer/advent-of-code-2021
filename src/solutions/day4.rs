use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Board {
    number_to_location: HashMap<u32, (u8, u8)>,

    /// `row_spots.get(i)` = number of spots marked in row `i`.
    row_spots: Vec<u8>,

    /// `col_spots.get(i)` = number of spots marked in col `i`.
    col_spots: Vec<u8>,

    unmarked: HashSet<u32>,
}

impl Board {
    fn new() -> Self {
        Self {
            number_to_location: HashMap::new(),
            row_spots: vec![0; 5],
            col_spots: vec![0; 5],
            unmarked: HashSet::new(),
        }
    }

    fn submit_move(&mut self, number: u32) {
        if let Some((row, col)) = self.number_to_location.get(&number) {
            *self.row_spots.get_mut(*row as usize).unwrap() += 1;
            *self.col_spots.get_mut(*col as usize).unwrap() += 1;
            self.unmarked.remove(&number);
        }
    }

    fn winning(&self) -> bool {
        self.row_spots.iter().any(|n| *n == 5) || self.col_spots.iter().any(|n| *n == 5)
    }
}

pub fn part1() {
    let stdin = io::stdin();

    let inp = stdin
        .lock()
        .lines()
        .map(|r| r.unwrap())
        .collect::<Vec<String>>();

    let mut input = inp.iter();

    let drawn = input
        .next()
        .expect("no drawn numbers line")
        .split(",")
        .map(|n| n.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut boards: Vec<Board> = Vec::new();

    {
        loop {
            let mut board = Board::new();

            if let None = input.next() {
                break;
            }

            for row in 0..5 {
                for (col, v) in input
                    .next()
                    .expect("missing board row")
                    .split_whitespace()
                    .map(|n| n.trim().parse::<u32>().unwrap())
                    .enumerate()
                {
                    board.number_to_location.insert(v, (row as u8, col as u8));
                    board.unmarked.insert(v);
                }
            }
            boards.push(board);
        }
    }

    for number in drawn {
        let mut won = false;
        for b in &mut boards {
            b.submit_move(number);
            if b.winning() {
                let unmarked_sum: u32 = b.unmarked.iter().sum();
                println!("unmarked sum: {}", unmarked_sum);
                println!("winning no: {}", number);
                println!("product: {}", unmarked_sum * number);
                won = true;
                break;
            }
        }
        if won {
            break;
        }
    }
}
