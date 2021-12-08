use std::fs;

fn part_one(positions: &mut [i32]) {
    positions.sort_unstable();
    let median = *positions.get(positions.len() / 2).unwrap();

    let movements = positions.iter().map(|&x| (x - median).abs()).sum::<i32>();
    println!("part one - {}", movements);
}

fn part_two(positions: &[i32]) {
    let mean = positions.iter().sum::<i32>() / positions.len() as i32;

    let movements = positions
        .iter()
        .map(|&x| (1..=(x - mean).abs()).sum::<i32>())
        .sum::<i32>();
    println!("part two - {}", movements);
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut positions = file
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    part_one(&mut positions);
    part_two(&positions);
}
