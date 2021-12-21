use std::{
    cmp::{max, min},
    collections::HashMap,
    ops::AddAssign,
};

fn take_turn(position: usize, roll: usize) -> usize {
    (position + roll + 9) % 10 + 1
}

fn part_one() {
    let (mut p1_pos, mut p2_pos) = (7, 8);
    let (mut p1_points, mut p2_points) = (0, 0);

    let mut roll = 1;
    let mut num_rolls = 0;
    loop {
        p1_pos = take_turn(p1_pos, roll * 3 + 3);
        p1_points += p1_pos;
        num_rolls += 3;
        roll += 3;
        if p1_points >= 1000 {
            break;
        }

        p2_pos = take_turn(p2_pos, roll * 3 + 3);
        p2_points += p2_pos;
        num_rolls += 3;
        roll += 3;
        if p2_points >= 1000 {
            break;
        }
    }

    println!("part one - {}", min(p1_points, p2_points) * num_rolls);
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum Player {
    P1,
    P2,
}

#[derive(Copy, Clone)]
struct Wins((usize, usize));

impl Wins {
    const P1: Self = Wins((1, 0));
    const P2: Self = Wins((0, 1));
    fn new() -> Self {
        Wins((0, 0))
    }
    fn most(&self) -> usize {
        max(self.0 .0, self.0 .1)
    }
}

impl AddAssign for Wins {
    fn add_assign(&mut self, rhs: Self) {
        self.0 .0 += rhs.0 .0;
        self.0 .1 += rhs.0 .1;
    }
}

fn calc_wins(
    p1_pos: usize,
    p2_pos: usize,
    p1_points: usize,
    p2_points: usize,
    turn: Player,
    cache: &mut HashMap<(usize, usize, usize, usize, Player), Wins>,
) -> Wins {
    if p1_points >= 21 {
        return Wins::P1;
    }
    if p2_points >= 21 {
        return Wins::P2;
    }
    if let Some(w) = cache.get(&(p1_pos, p2_pos, p1_points, p2_points, turn)) {
        return *w;
    }
    let mut wins = Wins::new();
    for r1 in 1..=3 {
        for r2 in 1..=3 {
            for r3 in 1..=3 {
                match turn {
                    Player::P1 => {
                        let pos = take_turn(p1_pos, r1 + r2 + r3);
                        let pts = p1_points + pos;
                        let w = calc_wins(pos, p2_pos, pts, p2_points, Player::P2, cache);
                        cache.insert((pos, p2_pos, pts, p2_points, Player::P2), w);
                        wins += w
                    }
                    Player::P2 => {
                        let pos = take_turn(p2_pos, r1 + r2 + r3);
                        let pts = p2_points + pos;
                        let w = calc_wins(p1_pos, pos, p1_points, pts, Player::P1, cache);
                        cache.insert((p1_pos, pos, p1_points, pts, Player::P1), w);
                        wins += w
                    }
                }
            }
        }
    }
    wins
}

fn part_two() {
    let mut cache = HashMap::new();
    let wins = calc_wins(7, 8, 0, 0, Player::P1, &mut cache);

    println!("part two - {}", wins.most());
}

fn main() {
    part_one();
    part_two();
}
