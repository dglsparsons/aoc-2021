use std::cmp;
use std::{fs, str::FromStr};

struct Coordinate {
    x: i64,
    y: i64,
}

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split.next().unwrap().parse::<i64>().unwrap();
        let y = split.next().unwrap().parse::<i64>().unwrap();
        Ok(Coordinate { x, y })
    }
}

fn part_one(lines: &[(Coordinate, Coordinate)]) {
    let mut grid = vec![vec![0; 1000]; 1000];

    for (start, end) in lines.iter() {
        if start.x == end.x {
            (cmp::min(start.y, end.y)..=cmp::max(start.y, end.y)).for_each(|y| {
                grid[start.x as usize][y as usize] += 1;
            });
        }
        if start.y == end.y {
            (cmp::min(start.x, end.x)..=cmp::max(start.x, end.x)).for_each(|x| {
                grid[x as usize][start.y as usize] += 1;
            });
        }
    }

    let count = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&x| x >= &2)
        .count();

    println!("part one - {}", count);
}

fn part_two(lines: &[(Coordinate, Coordinate)]) {
    let mut grid = vec![vec![0; 1000]; 1000];

    for (start, end) in lines.iter() {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let dx = if dx == 0 { dx } else { dx / dx.abs() };
        let dy = if dy == 0 { dy } else { dy / dy.abs() };

        let mut x = start.x;
        let mut y = start.y;
        loop {
            grid[y as usize][x as usize] += 1;
            if x == end.x && y == end.y {
                break;
            }
            x += dx;
            y += dy;
        }
    }
    let count = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&x| x >= &2)
        .count();

    println!("part two - {}", count);
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let lines = file
        .lines()
        .map(|l| l.split(" -> ").collect::<Vec<&str>>())
        .map(|l| {
            (
                l[0].parse::<Coordinate>().unwrap(),
                l[1].parse::<Coordinate>().unwrap(),
            )
        })
        .collect::<Vec<(Coordinate, Coordinate)>>();

    part_one(&lines);
    part_two(&lines);
}
