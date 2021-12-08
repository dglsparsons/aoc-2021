use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::FromIterator,
};

fn part_one(input: &[Vec<Vec<&str>>]) {
    let unique_counts: HashSet<usize> = HashSet::from_iter([2, 3, 4, 7]);
    let count = input
        .iter()
        .map(|row| {
            row[1]
                .iter()
                .filter(|w| unique_counts.contains(&w.len()))
                .count()
        })
        .sum::<usize>();
    println!("part one - {}", count);
}

fn part_two(input: &[Vec<Vec<&str>>]) {
    let count = input
        .iter()
        .map(|row| {
            let mut segment_counts = HashMap::new();
            row[0].iter().for_each(|digit| {
                digit.chars().for_each(|c| {
                    *segment_counts.entry(c).or_insert(0) += 1;
                })
            });
            row[1]
                .iter()
                .map(|digit| {
                    let sum = digit
                        .chars()
                        .map(|c| segment_counts.get(&c).unwrap())
                        .sum::<usize>();
                    match sum {
                        42 => '0',
                        17 => '1',
                        34 => '2',
                        39 => '3',
                        30 => '4',
                        37 => '5',
                        41 => '6',
                        25 => '7',
                        49 => '8',
                        45 => '9',
                        _ => panic!("unexpected sum {}", sum),
                    }
                })
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum::<usize>();

    println!("part two - {}", count);
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let input = file
        .lines()
        .map(|l| {
            l.split(" | ")
                .map(|l| l.split_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part_one(&input);
    part_two(&input);
}
