use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Toggle {
    On,
    Off,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cuboid {
    toggle: Toggle,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

impl Cuboid {
    fn volume(&self) -> i64 {
        let v = (self.max_x + 1 - self.min_x)
            * (self.max_y + 1 - self.min_y)
            * (self.max_z + 1 - self.min_z);
        match self.toggle {
            Toggle::Off => -v,
            Toggle::On => v,
        }
    }

    fn get_intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        if other.min_x > self.max_x
            || other.max_x < self.min_x
            || other.min_y > self.max_y
            || other.max_y < self.min_y
            || other.min_z > self.max_z
            || other.max_z < self.min_z
        {
            // No overlap at all.
            return None;
        }

        Some(Cuboid {
            toggle: if self.toggle == Toggle::On {
                Toggle::Off
            } else {
                Toggle::On
            },
            min_x: max(self.min_x, other.min_x),
            max_x: min(self.max_x, other.max_x),
            min_y: max(self.min_y, other.min_y),
            max_y: min(self.max_y, other.max_y),
            min_z: max(self.min_z, other.min_z),
            max_z: min(self.max_z, other.max_z),
        })
    }
}

fn parse_instruction(input: &str) -> Cuboid {
    let input = input.replace("x=", "").replace("y=", "").replace("z=", "");
    let mut toggle = Toggle::On;
    if input.starts_with("off") {
        toggle = Toggle::Off;
    }

    let things = input
        .replace("on ", "")
        .replace("off ", "")
        .split(',')
        .flat_map(|f| f.split("..").collect::<Vec<&str>>())
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let (min_x, max_x, min_y, max_y, min_z, max_z) = (
        things[0], things[1], things[2], things[3], things[4], things[5],
    );

    Cuboid {
        toggle,
        min_x,
        max_x,
        min_y,
        max_y,
        min_z,
        max_z,
    }
}

fn part_one(instructions: &[Cuboid]) {
    let mut set = HashSet::new();
    for i in instructions {
        for x in max(i.min_x, -50)..=min(i.max_x, 50) {
            for y in max(i.min_y, -50)..=min(i.max_y, 50) {
                for z in max(i.min_z, -50)..=min(i.max_z, 50) {
                    match i.toggle {
                        Toggle::On => set.insert((x, y, z)),
                        Toggle::Off => set.remove(&(x, y, z)),
                    };
                }
            }
        }
    }

    println!("part one - {}", set.len())
}

fn part_two(instructions: &[Cuboid]) {
    let mut cuboids: Vec<Cuboid> = Vec::new();
    for c in instructions.iter() {
        let mut new = cuboids.clone();
        for c2 in cuboids.iter() {
            if let Some(i) = c2.get_intersection(c) {
                new.push(i);
            };
        }
        if c.toggle == Toggle::On {
            new.push(c.to_owned());
        }
        cuboids = new;
    }

    println!(
        "part two - {}",
        cuboids.iter().map(|c| c.volume()).sum::<i64>()
    )
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let instructions = file
        .lines()
        .map(|l| parse_instruction(l))
        .collect::<Vec<_>>();

    part_one(&instructions);
    part_two(&instructions);
}
