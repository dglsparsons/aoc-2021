use std::{collections::HashSet, fs};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

fn step(enhancement: &[char], image: HashSet<Coord>, out_of_bounds: bool) -> HashSet<Coord> {
    let y_min = image.iter().map(|c| c.y).min().unwrap() - 1;
    let y_max = image.iter().map(|c| c.y).max().unwrap() + 1;
    let x_min = image.iter().map(|c| c.x).min().unwrap() - 1;
    let x_max = image.iter().map(|c| c.x).max().unwrap() + 1;
    let mut out = HashSet::new();
    let get = |y, x| {
        if out_of_bounds && enhancement[0] == '#' {
            if y <= y_min || y >= y_max {
                return '1';
            }
            if x <= x_min || x >= x_max {
                return '1';
            }
        }
        if image.contains(&Coord { y, x }) {
            '1'
        } else {
            '0'
        }
    };
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let binary = vec![
                get(y - 1, x - 1),
                get(y - 1, x),
                get(y - 1, x + 1),
                get(y, x - 1),
                get(y, x),
                get(y, x + 1),
                get(y + 1, x - 1),
                get(y + 1, x),
                get(y + 1, x + 1),
            ]
            .iter()
            .collect::<String>();
            let binary = usize::from_str_radix(&binary, 2).unwrap();
            if enhancement.get(binary) == Some(&'#') {
                out.insert(Coord { y, x });
            }
        }
    }

    out
}

fn part_one(enhancement: &[char], image: &HashSet<Coord>) {
    let mut new_image = image.to_owned();
    for i in 0..2 {
        new_image = step(enhancement, new_image, i % 2 == 1);
    }

    println!("part one - {}", new_image.len());
}

fn part_two(enhancement: &[char], image: &HashSet<Coord>) {
    let mut new_image = image.to_owned();
    for i in 0..50 {
        new_image = step(enhancement, new_image, i % 2 == 1);
    }

    println!("part two - {}", new_image.len());
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let enhancement = file.lines().next().unwrap().chars().collect::<Vec<_>>();

    let image = file
        .lines()
        .skip(2)
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '#' {
                        Some(Coord {
                            y: y as i64,
                            x: x as i64,
                        })
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>();

    part_one(&enhancement, &image);
    part_two(&enhancement, &image);
}
