use std::convert::TryFrom;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("unable to read input file");

    let measurements: Vec<_> = file
        .lines()
        .map(|s| s.parse::<usize>().expect("couldn't parse number"))
        .collect();

    part_one(&measurements);

    part_two(&measurements);
}

fn part_one(measurements: &[usize]) {
    let count = measurements
        .windows(2)
        .flat_map(<&[usize; 2]>::try_from)
        .filter(|[a, b]| b > a)
        .count();

    println!("part one - count: {}", count);
}

fn part_two(measurements: &[usize]) {
    let count = measurements
        .windows(4)
        .flat_map(<&[usize; 4]>::try_from)
        .filter(|[a, b, c, d]| b + c + d > a + b + c)
        .count();

    println!("part two - count: {}", count);
}
