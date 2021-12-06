use std::cmp;
use std::collections::HashMap;
use std::{fs, str::FromStr};

#[derive(Hash, Eq, PartialEq, Debug)]
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
    let mut visited = HashMap::new();

    for (start, end) in lines.iter() {
        if start.x == end.x {
            (cmp::min(start.y, end.y)..=cmp::max(start.y, end.y)).for_each(|y| {
                *visited.entry(Coordinate { x: start.x, y }).or_insert(0) += 1;
            });
        }
        if start.y == end.y {
            (cmp::min(start.x, end.x)..=cmp::max(start.x, end.x)).for_each(|x| {
                *visited.entry(Coordinate { x, y: start.y }).or_insert(0) += 1;
            });
        }
    }

    let count = visited.values().filter(|&x| *x >= 2).count();
    println!("part one - {}", count);
}

fn part_two(lines: &[(Coordinate, Coordinate)]) {
    let mut visited = HashMap::new();

    for (start, end) in lines.iter() {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let dx = if dx == 0 { dx } else { dx / dx.abs() };
        let dy = if dy == 0 { dy } else { dy / dy.abs() };

        let mut x = start.x;
        let mut y = start.y;
        loop {
            *visited.entry(Coordinate { x, y }).or_insert(0) += 1;
            if x == end.x && y == end.y {
                break;
            }
            x += dx;
            y += dy;
        }
    }
    let count = visited.values().filter(|&x| *x >= 2).count();
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
