use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

#[derive(Clone)]
struct Board {
    squares: [[u32; 5]; 5],
}

impl Board {
    fn has_won(&self, called: &HashSet<u32>) -> bool {
        for i in 0..5 {
            if (0..5).all(|j| called.contains(&self.squares[i][j])) {
                return true;
            }
            if (0..5).all(|j| called.contains(&self.squares[j][i])) {
                return true;
            }
        }
        false
    }

    fn sum_uncalled(&self, called: HashSet<u32>) -> u32 {
        self.squares
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&x| !called.contains(x))
            .sum()
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut squares = [[0; 5]; 5];
        for (y, line) in s.lines().enumerate() {
            for (x, n) in line.split_whitespace().enumerate() {
                squares[y][x] = n.parse().unwrap();
            }
        }

        Ok(Board { squares })
    }
}

fn part_one(instructions: &[u32], boards: &[Board]) {
    let mut called = HashSet::new();
    for i in instructions {
        called.insert(*i);
        for board in boards {
            if board.has_won(&called) {
                println!("{}", board.sum_uncalled(called) * i);
                return;
            }
        }
    }
}

fn part_two(instructions: &[u32], boards: &[Board]) {
    let mut boards = boards.to_vec();
    let mut called = HashSet::new();
    for i in instructions {
        called.insert(*i);
        if boards.len() == 1 && boards[0].has_won(&called) {
            println!("{}", boards[0].sum_uncalled(called) * i);
            return;
        }
        boards.retain(|b| !b.has_won(&called));
    }
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let contents = file.split("\n\n").collect::<Vec<_>>();
    let instructions = contents[0]
        .split(',')
        .map(|i| i.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let boards = contents[1..]
        .iter()
        .map(|s| s.parse::<Board>().unwrap())
        .collect::<Vec<_>>();

    part_one(&instructions, &boards);
    part_two(&instructions, &boards);
}
