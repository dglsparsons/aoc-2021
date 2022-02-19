use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

enum AmphipodType {
    A,
    B,
    C,
    D,
}

struct Amphipod {
    x: usize,
    y: usize,
    species: AmphipodType,
}

impl Amphipod {
    fn step_cost(c: char) -> usize {
        match c {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            _ => 1000,
        }
    }
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut initial_state = HashMap::new();
    file.lines().skip(1).enumerate().for_each(|(y, l)| {
        l.chars().skip(1).enumerate().for_each(|(x, c)| {
            if c == '#' || c == ' ' {
                return;
            }

            initial_state.insert((y, x), c);
        })
    });
    part_one(&initial_state);
}

#[derive(Eq, PartialEq, Debug)]
struct Candidate {
    cost: usize,
    state: HashMap<(usize, usize), char>,
}

fn is_in_front_of_room();

impl Candidate {
    fn is_end_state(&self) -> bool {
        false
    }

    fn get_moves(&self) -> Vec<Candidate> {
        let mut moves = Vec::new();
        moves
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

fn part_one(initial_state: &HashMap<(usize, usize), char>) {
    let mut candidates = BinaryHeap::new();
    let mut visited = HashSet::new();

    candidates.push(Reverse(Candidate {
        cost: 0,
        state: initial_state.to_owned(),
    }));

    let cost = loop {
        let current = candidates.pop().unwrap().0;
        if current.is_end_state() {
            break current.cost;
        }

        visited.insert(current.state.to_owned());
        for c in current.get_moves() {
            if visited.contains(&c.state) {
                continue;
            }
            visited.insert(c.state.to_owned());
            candidates.push(Reverse(c));
        }
    };
    // take next step
    println!("part one - {}", cost);
}
