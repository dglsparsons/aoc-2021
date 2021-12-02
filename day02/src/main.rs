use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("unable to read input file");

    let commands: Vec<_> = file
        .lines()
        .map(|l| l.split(' ').collect::<Vec<_>>())
        .map(|l| {
            let command = l[0];
            let value = l[1].parse::<i64>().unwrap();
            (command, value)
        })
        .collect();

    part_one(&commands);
    part_two(&commands);
}

fn part_one(commands: &[(&str, i64)]) {
    let res = commands
        .iter()
        .fold((0, 0), |(pos, depth), (c, v)| match *c {
            "forward" => (pos + v, depth),
            "down" => (pos, depth + v),
            "up" => (pos, depth - v),
            _ => panic!("unknown command"),
        });

    let (y, z) = res;
    println!("part one - {}", y * z);
}

fn part_two(commands: &[(&str, i64)]) {
    let res = commands
        .iter()
        .fold((0, 0, 0), |(pos, depth, aim), (c, v)| match *c {
            "forward" => (pos + v, depth + (aim * v), aim),
            "down" => (pos, depth, aim + v),
            "up" => (pos, depth, aim - v),
            _ => panic!("unknown command"),
        });

    let (y, z, _) = res;
    println!("part two - {}", y * z);
}
