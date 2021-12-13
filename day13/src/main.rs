use std::{collections::HashSet, fs};

enum Direction {
    X,
    Y,
}

struct Instruction {
    direction: Direction,
    at: usize,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

fn fold(coords: &HashSet<Coord>, i: &Instruction) -> HashSet<Coord> {
    let mut new_coords = HashSet::new();
    for c in coords.iter() {
        match i.direction {
            Direction::X => {
                if c.x > i.at {
                    new_coords.insert(Coord {
                        y: c.y,
                        x: c.x - 2 * (c.x - i.at),
                    });
                } else {
                    new_coords.insert(c.clone());
                }
            }
            Direction::Y => {
                if c.y > i.at {
                    new_coords.insert(Coord {
                        y: c.y - 2 * (c.y - i.at),
                        x: c.x,
                    });
                } else {
                    new_coords.insert(c.clone());
                }
            }
        }
    }
    new_coords
}

fn part_one(coords: &HashSet<Coord>, instructions: &[Instruction]) {
    let coords = fold(coords, &instructions[0]);
    println!("part one: {}", coords.len());
}

fn part_two(coords: &HashSet<Coord>, instructions: &[Instruction]) {
    let mut coords = coords.to_owned();
    for i in instructions.iter() {
        coords = fold(&coords, i)
    }
    for y in 0..6 {
        let mut out = "".to_owned();
        for x in 0..40 {
            if coords.contains(&Coord { x, y }) {
                out.push('#');
            } else {
                out.push(' ');
            }
        }
        println!("{}", out);
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut split = file.split("\n\n");
    let coords = split
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|l| Coord { x: l[0], y: l[1] })
        .collect::<HashSet<_>>();

    let instructions = split
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let l = l.strip_prefix("fold along ").unwrap();
            let mut l = l.split('=');
            let d = l.next().unwrap();
            let n = l.next().unwrap();

            Instruction {
                direction: if d == "y" { Direction::Y } else { Direction::X },
                at: n.parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    part_one(&coords, &instructions);
    part_two(&coords, &instructions);
}
