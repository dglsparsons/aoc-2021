use std::{collections::HashMap, fs};

fn get_next_value(
    pairs: HashMap<(char, char), u64>,
    insertions: &HashMap<(char, char), char>,
    counts: &mut HashMap<char, u64>,
) -> HashMap<(char, char), u64> {
    let mut new = HashMap::new();
    for (&(c0, c1), &i) in pairs.iter() {
        match insertions.get(&(c0, c1)) {
            None => {
                new.insert((c0, c1), i);
            }
            Some(&n) => {
                *counts.entry(n).or_insert(0) += i;
                *new.entry((c0, n)).or_insert(0) += i;
                *new.entry((n, c1)).or_insert(0) += i;
            }
        };
    }

    new
}

fn part_one(
    mut current_value: HashMap<(char, char), u64>,
    insertions: &HashMap<(char, char), char>,
    mut counts: HashMap<char, u64>,
) {
    for _ in 0..10 {
        current_value = get_next_value(current_value, insertions, &mut counts);
    }

    println!(
        "part one - {}",
        counts.values().max().unwrap() - counts.values().min().unwrap()
    );
}

fn part_two(
    mut current_value: HashMap<(char, char), u64>,
    insertions: &HashMap<(char, char), char>,
    mut counts: HashMap<char, u64>,
) {
    for _ in 0..40 {
        current_value = get_next_value(current_value, insertions, &mut counts);
    }

    println!(
        "part two - {}",
        counts.values().max().unwrap() - counts.values().min().unwrap()
    );
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut lines = file.lines();
    let starting_value = lines.next().unwrap();
    lines.next().unwrap();

    let mut insertions = HashMap::new();
    lines
        .map(|l| l.split(" -> ").collect::<Vec<_>>())
        .for_each(|l| {
            let mut chars = l[0].chars();
            insertions.insert(
                (chars.next().unwrap(), chars.next().unwrap()),
                l[1].chars().next().unwrap(),
            );
        });

    let mut starting_pairs = HashMap::new();
    starting_value
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .for_each(|w| *starting_pairs.entry((w[0], w[1])).or_insert(0) += 1);

    let mut counts = HashMap::new();
    starting_value
        .chars()
        .for_each(|c| *counts.entry(c).or_insert(0) += 1);

    part_one(starting_pairs.to_owned(), &insertions, counts.to_owned());
    part_two(starting_pairs, &insertions, counts);
}
