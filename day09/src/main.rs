use std::collections::HashSet;
use std::fs;

struct Neighbour {
    x: usize,
    y: usize,
    v: u32,
}

fn get_neighbours(grid: &[Vec<u32>], y: usize, x: usize) -> Vec<Neighbour> {
    vec![
        if y > 0 {
            grid.get(y - 1)
                .and_then(|y| y.get(x))
                .map(|&v| Neighbour { x, y: y - 1, v })
        } else {
            None
        },
        if x > 0 {
            grid.get(y)
                .and_then(|y| y.get(x - 1))
                .map(|&v| Neighbour { x: x - 1, y, v })
        } else {
            None
        },
        grid.get(y + 1)
            .and_then(|y| y.get(x))
            .map(|&v| Neighbour { x, y: y + 1, v }),
        grid.get(y)
            .and_then(|y| y.get(x + 1))
            .map(|&v| Neighbour { x: x + 1, y, v }),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn part_one(grid: &[Vec<u32>]) {
    let mut risk = 0;
    for (i, y) in grid.iter().enumerate() {
        for (j, x) in y.iter().enumerate() {
            if get_neighbours(grid, i, j).iter().all(|n| n.v > *x) {
                risk += x + 1
            }
        }
    }
    println!("part one - {}", risk);
}

fn build_basin(
    grid: &[Vec<u32>],
    visited: &mut HashSet<(usize, usize)>,
    i: usize,
    j: usize,
) -> usize {
    match grid.get(i).and_then(|row| row.get(j)) {
        None => 0,
        Some(_) if visited.contains(&(i, j)) => 0,
        Some(v) if *v == 9 => 0,
        _ => {
            visited.insert((i, j));
            let neighbours = get_neighbours(grid, i, j);

            neighbours
                .iter()
                .map(|n| build_basin(grid, visited, n.y, n.x))
                .sum::<usize>()
                + 1
        }
    }
}

fn part_two(grid: &[Vec<u32>]) {
    let mut visited = HashSet::new();
    let mut basins = Vec::new();

    for (i, y) in grid.iter().enumerate() {
        for j in 0..y.len() {
            basins.push(build_basin(grid, &mut visited, i, j));
        }
    }
    basins.sort_unstable();
    basins.reverse();

    println!("part two - {}", basins[0] * basins[1] * basins[2]);
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let grid = file
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part_one(&grid);
    part_two(&grid);
}
