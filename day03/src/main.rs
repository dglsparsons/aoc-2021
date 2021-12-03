use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("unable to read input file");

    let readings: Vec<_> = file.lines().collect();

    part_one(&readings);
    part_two(&readings);
}

fn part_one(readings: &[&str]) {
    let mut one_counts = vec![0; readings[0].len()];
    for reading in readings.iter() {
        for (i, c) in reading.chars().enumerate() {
            if c == '1' {
                one_counts[i] += 1;
            }
        }
    }
    let gamma = one_counts
        .iter()
        .map(|x| if x > &(readings.len() / 2) { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("");

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&"1".repeat(readings[0].len()), 2).unwrap() - gamma;

    println!("part_one - {}", gamma * epsilon);
}

fn part_two(readings: &[&str]) {
    fn most_common_value(values: &[&str], i: usize) -> char {
        let one_count = values
            .iter()
            .map(|x| x.chars().nth(i).unwrap())
            .filter(|x| x == &'1')
            .count();

        match one_count {
            c if c < values.len() / 2 => '0',
            _ => '1',
        }
    }

    let find_value = |should_keep: fn(char, char) -> bool| {
        let mut values = readings.to_owned();
        let mut i = 0;
        while values.len() > 1 {
            let most_common = most_common_value(&values, i);
            values.retain(|v| should_keep(v.chars().nth(i).unwrap(), most_common));
            i += 1
        }
        u32::from_str_radix(values[0], 2).unwrap()
    };

    let oxygen = find_value(|c, most_common| c == most_common);
    let co2 = find_value(|c, most_common| c != most_common);

    println!("part_two - {}", oxygen * co2);
}
