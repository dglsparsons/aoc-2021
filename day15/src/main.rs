use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
};

#[derive(Eq, PartialEq, Debug)]
struct Candidate {
    cost: u32,
    coord: (usize, usize),
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

fn get_shortest_path_cost(grid: &[Vec<u32>]) -> u32 {
    let mut candidates = BinaryHeap::new();
    let mut visited = HashSet::new();

    candidates.push(Reverse(Candidate {
        cost: 0,
        coord: (0, 0),
    }));

    // take the lowest value, expanding that direction
    // until we are at the end.
    loop {
        let current = candidates.pop().unwrap().0;
        if current.coord == (grid.len() - 1, grid[0].len() - 1) {
            break current.cost;
        }

        visited.insert(current.coord);
        for c in get_neighbours(grid, &current.coord) {
            if visited.contains(&c) {
                continue;
            }
            visited.insert(c);
            candidates.push(Reverse(Candidate {
                cost: current.cost + grid[c.0][c.1],
                coord: c,
            }));
        }
    }
}

fn get_neighbours(grid: &[Vec<u32>], &(y, x): &(usize, usize)) -> Vec<(usize, usize)> {
    vec![
        if y > 0 { Some((y - 1, x)) } else { None },
        if y < grid.len() - 1 {
            Some((y + 1, x))
        } else {
            None
        },
        if x > 0 { Some((y, x - 1)) } else { None },
        if x < grid[0].len() - 1 {
            Some((y, x + 1))
        } else {
            None
        },
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<(usize, usize)>>()
}

fn part_one(grid: &[Vec<u32>]) {
    let cost = get_shortest_path_cost(grid);
    println!("{}", cost);
}

fn part_two(grid: &[Vec<u32>]) {
    let one_by_five = grid
        .iter()
        .map(|row| {
            (0..5)
                .flat_map(move |n| {
                    row.to_owned()
                        .iter()
                        .map(|c| if c + n < 10 { c + n } else { c + n - 9 })
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut extended = one_by_five.clone();
    for n in 1..5 {
        extended.extend(one_by_five.iter().map(|row| {
            row.iter()
                .map(|c| if c + n < 10 { c + n } else { c + n - 9 })
                .collect::<Vec<_>>()
        }))
    }
    let cost = get_shortest_path_cost(&extended);
    println!("{}", cost);
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let grid = file
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part_one(&grid);
    part_two(&grid);
}
