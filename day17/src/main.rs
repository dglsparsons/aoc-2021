use std::fs;

fn part_one(min_y: i32) {
    let mut dy = min_y.abs() - 1;
    let mut pos = 0;
    while dy > 0 {
        pos += dy;
        dy -= 1;
    }
    println!("part one: {}", pos);
}

fn part_two(min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
    let mut x_pos = 0;
    let mut min_dx = 1;
    while x_pos < min_x {
        min_dx += 1;
        x_pos += min_dx;
    }

    let hits_target = |mut dx: i32, mut dy: i32| {
        let mut x = 0;
        let mut y = 0;
        while x <= max_x && y >= min_y {
            x += dx;
            y += dy;
            if dx > 0 {
                dx -= 1;
            }
            dy -= 1;
            if (x >= min_x && x <= max_x) && (y >= min_y && y <= max_y) {
                return true;
            }
        }
        false
    };

    let mut count = 0;
    for dx in min_dx..=max_x {
        for dy in min_y..=(min_y.abs() - 1) {
            if hits_target(dx, dy) {
                count += 1;
            }
        }
    }

    println!("part two: {}", count);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input
        .trim()
        .replace("target area: ", "")
        .replace("x=", "")
        .replace("y=", "")
        .replace(" ", "")
        .replace("..", ",")
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let (min_x, max_x, min_y, max_y) = (input[0], input[1], input[2], input[3]);

    part_one(min_y);
    part_two(min_x, max_x, min_y, max_y);
}
