use std::{collections::HashMap, fs, iter::FromIterator};

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut initial_state = HashMap::new();
    file.trim_end()
        .split(',')
        .map(|n| {
            let blah = format!("Not a number - '{}'", n);
            n.parse::<i64>().expect(&blah)
        })
        .for_each(|n| {
            *initial_state.entry(n).or_insert(0) += 1;
        });

    part_one(&initial_state);
    part_two(&initial_state);
}

fn part_two(initial_state: &HashMap<i64, i64>) {
    let end_state = get_state_after_n_steps(initial_state, 256);
    println!("part two - {}", end_state.values().sum::<i64>());
}

fn part_one(initial_state: &HashMap<i64, i64>) {
    let end_state = get_state_after_n_steps(initial_state, 80);
    println!("part one - {}", end_state.values().sum::<i64>());
}

fn get_state_after_n_steps(initial_state: &HashMap<i64, i64>, n: usize) -> HashMap<i64, i64> {
    let mut state = initial_state.clone();
    for _ in 0..n {
        state = HashMap::from_iter([
            (0, *state.get(&1).unwrap_or(&0)),
            (1, *state.get(&2).unwrap_or(&0)),
            (2, *state.get(&3).unwrap_or(&0)),
            (3, *state.get(&4).unwrap_or(&0)),
            (4, *state.get(&5).unwrap_or(&0)),
            (5, *state.get(&6).unwrap_or(&0)),
            (
                6,
                *state.get(&7).unwrap_or(&0) + state.get(&0).unwrap_or(&0),
            ),
            (7, *state.get(&8).unwrap_or(&0)),
            (8, *state.get(&0).unwrap_or(&0)),
        ]);
    }

    state
}
