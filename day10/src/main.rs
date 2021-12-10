use std::fs;

fn part_one(input: &str) {
    let total = input
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            let mut score = 0;
            line.chars().for_each(|c| match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                c => {
                    if stack.pop() != Some(c) {
                        score = match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0,
                        }
                    }
                }
            });
            score
        })
        .sum::<usize>();

    println!("part one {}", total);
}

struct StackItem {
    c: char,
    points: u64,
}

fn get_autocomplete_score(line: &str) -> Option<u64> {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' => stack.push(StackItem { c: ')', points: 1 }),
            '[' => stack.push(StackItem { c: ']', points: 2 }),
            '{' => stack.push(StackItem { c: '}', points: 3 }),
            '<' => stack.push(StackItem { c: '>', points: 4 }),
            c => {
                if stack.pop().map(|i| i.c) != Some(c) {
                    return None;
                }
            }
        }
    }

    Some(stack.iter().rev().fold(0, |acc, c| acc * 5 + c.points))
}

fn part_two(input: &str) {
    let mut scores = input
        .lines()
        .map(|line| get_autocomplete_score(line))
        .flatten()
        .collect::<Vec<u64>>();

    scores.sort_unstable();
    println!("part two {}", scores[scores.len() / 2]);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    part_one(&input);
    part_two(&input);
}
