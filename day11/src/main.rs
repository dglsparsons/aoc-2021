use std::fs;

fn flash(grid: &mut [[u32; 10]; 10], y: usize, x: usize, flashes: &mut usize) {
    let o = grid.get_mut(y).and_then(|r| r.get_mut(x));
    match o {
        None => {}
        Some(octopus) => {
            *octopus += 1;
            if *octopus != 10 {
                return;
            }
            *flashes += 1;
            if y > 0 && x > 0 {
                flash(grid, y - 1, x - 1, flashes);
            }
            if y > 0 {
                flash(grid, y - 1, x, flashes);
                flash(grid, y - 1, x + 1, flashes);
            }
            if x > 0 {
                flash(grid, y, x - 1, flashes);
                flash(grid, y + 1, x - 1, flashes);
            }
            flash(grid, y + 1, x, flashes);
            flash(grid, y + 1, x + 1, flashes);
            flash(grid, y, x + 1, flashes);
        }
    }
}

fn next_step(grid: &mut [[u32; 10]; 10], flashes: &mut usize) {
    for line in grid.iter_mut() {
        for o in line.iter_mut() {
            if *o >= 10 {
                *o = 0;
            }
        }
    }
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            flash(grid, y, x, flashes)
        }
    }
}

fn part_one(grid: &mut [[u32; 10]; 10]) {
    let mut flashes = 0;

    for _ in 0..100 {
        next_step(grid, &mut flashes);
    }

    println!("part one - {}", flashes);
}

fn part_two(grid: &mut [[u32; 10]; 10]) {
    let mut flashes = 0;

    let mut step = 0;
    while !grid.iter().flat_map(|l| l.map(|o| o >= 10)).all(|x| x) {
        next_step(grid, &mut flashes);
        step += 1
    }

    println!("part two - {}", step);
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut grid = [[0; 10]; 10];

    for (y, line) in file.lines().enumerate() {
        for (x, n) in line.chars().enumerate() {
            grid[y][x] = n.to_digit(10).unwrap();
        }
    }

    part_one(&mut grid.to_owned());
    part_two(&mut grid.to_owned());
}
