use std::collections::{HashMap, HashSet};
use std::fs;

fn get_paths_to_end(
    node: &str,
    paths: &HashMap<&str, Vec<&str>>,
    visits: &HashSet<&str>,
    can_double_visit: bool,
) -> u64 {
    if node == "end" {
        return 1;
    }
    let mut visits = visits.clone();
    let mut can_double_visit = can_double_visit;
    if node.chars().all(|c| c.is_lowercase()) {
        if visits.contains(node) {
            can_double_visit = false;
        }
        visits.insert(node);
    }

    let mut count = 0;
    for n in paths.get(node).unwrap_or(&vec![]) {
        if *n == "start" || (!can_double_visit && visits.contains(n)) {
            continue;
        }
        count += get_paths_to_end(n, paths, &visits, can_double_visit);
    }

    count
}

fn part_one(paths: &HashMap<&str, Vec<&str>>) {
    let path_count = get_paths_to_end("start", paths, &HashSet::new(), false);

    println!("part one - {}", path_count);
}

fn part_two(paths: &HashMap<&str, Vec<&str>>) {
    let path_count = get_paths_to_end("start", paths, &HashSet::new(), true);

    println!("part two - {}", path_count);
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut paths: HashMap<&str, Vec<&str>> = HashMap::new();
    file.lines()
        .map(|l| l.split('-').collect::<Vec<_>>())
        .for_each(|l| {
            if let Some(v) = paths.get_mut(&l[0]) {
                v.push(l[1]);
            } else {
                paths.insert(l[0], vec![l[1]]);
            }
            if let Some(v) = paths.get_mut(&l[1]) {
                v.push(l[0]);
            } else {
                paths.insert(l[1], vec![l[0]]);
            }
        });

    part_one(&paths);
    part_two(&paths);
}
