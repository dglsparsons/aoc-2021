use std::fs;

#[derive(Debug)]
enum Operation {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
enum Packet {
    LiteralValue(u64, u64),
    Operator(u64, Operation, Vec<Packet>),
}

impl Packet {
    fn value(&self) -> u64 {
        match self {
            Packet::LiteralValue(_, v) => *v,
            Packet::Operator(_, op, children) => {
                let mut values = children.iter().map(|child| child.value());
                match op {
                    Operation::Sum => values.sum(),
                    Operation::Product => values.product(),
                    Operation::Min => values.min().unwrap_or(0),
                    Operation::Max => values.max().unwrap_or(0),
                    Operation::GreaterThan => {
                        if values.next() > values.next() {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::LessThan => {
                        if values.next() < values.next() {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::EqualTo => {
                        if values.next() == values.next() {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }
}

fn parse_packet(input: &[char]) -> Option<(Packet, Vec<char>)> {
    let first_3 = input.iter().take(3).collect::<String>();
    if first_3.is_empty() {
        return None;
    }
    let packet_version = u64::from_str_radix(&first_3, 2).unwrap();

    let second_3 = input.iter().skip(3).take(3).collect::<String>();
    if second_3.is_empty() {
        return None;
    }
    let packet_type = u64::from_str_radix(&second_3, 2).unwrap();
    let rest = input.iter().skip(6).copied().collect::<Vec<char>>();
    if rest.is_empty() {
        return None;
    }

    match packet_type {
        4 => Some(parse_literal_value(rest, packet_version)),
        _ => parse_operator(rest, packet_type, packet_version),
    }
}

fn parse_operator(rest: Vec<char>, packet_type: u64, version: u64) -> Option<(Packet, Vec<char>)> {
    let length_type = rest[0];
    let op = match packet_type {
        0 => Operation::Sum,
        1 => Operation::Product,
        2 => Operation::Min,
        3 => Operation::Max,
        5 => Operation::GreaterThan,
        6 => Operation::LessThan,
        _ => Operation::EqualTo,
    };
    match length_type {
        '0' => {
            let n = usize::from_str_radix(&rest[1..16].iter().collect::<String>(), 2).unwrap();
            let mut packets = Vec::new();
            let mut bleh = rest[16..16 + n].to_owned();
            loop {
                match parse_packet(&bleh) {
                    Some((packet, left)) => {
                        packets.push(packet);
                        bleh = left.to_owned();
                        if bleh.is_empty() {
                            break Some((
                                Packet::Operator(version, op, packets),
                                rest[16 + n..].to_vec(),
                            ));
                        }
                    }
                    None => panic!("Invalid packet"),
                }
            }
        }
        _ => {
            let n = usize::from_str_radix(&rest[1..12].iter().collect::<String>(), 2).unwrap();
            let mut bleh = rest[12..].to_owned();
            let mut packets = Vec::new();
            for _ in 0..n {
                let (packet, left) = parse_packet(&bleh).unwrap();
                packets.push(packet);
                bleh = left.to_owned();
            }

            Some((Packet::Operator(version, op, packets), bleh.to_vec()))
        }
    }
}

fn parse_literal_value(rest: Vec<char>, version: u64) -> (Packet, Vec<char>) {
    let mut digit = Vec::new();
    let mut more_left = true;
    let mut rest = rest.into_iter();
    while more_left {
        let x = rest.next().unwrap();
        if x == '0' {
            more_left = false;
        }

        digit.extend([
            rest.next().unwrap(),
            rest.next().unwrap(),
            rest.next().unwrap(),
            rest.next().unwrap(),
        ]);
    }
    let number = digit.iter().collect::<String>();
    let number = u64::from_str_radix(&number, 2).unwrap();

    (
        Packet::LiteralValue(version, number),
        rest.collect::<Vec<_>>(),
    )
}

fn sum_version_numbers(packets: &[Packet]) -> u64 {
    let mut sum = 0;
    for p in packets {
        match p {
            Packet::LiteralValue(version, _) => sum += version,
            Packet::Operator(version, _, packets) => sum += version + sum_version_numbers(packets),
        }
    }

    sum
}

fn parse_all(input: &[char]) -> Vec<Packet> {
    let mut packets = Vec::new();
    let mut rest = input.to_owned();
    while !rest.is_empty() {
        match parse_packet(&rest) {
            Some((packet, left)) => {
                packets.push(packet);
                rest = left;
            }
            None => break,
        }
    }

    packets
}

fn part_one(input: &[char]) {
    let packets = parse_all(input);
    println!("part one - {}", sum_version_numbers(&packets));
}

fn part_two(input: &[char]) {
    let packets = parse_all(input);
    println!("part two - {}", packets[0].value());
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let binary = file
        .trim()
        .chars()
        .map(|c| format!("{:0>4b}", c.to_digit(16).unwrap()))
        .collect::<Vec<_>>()
        .join("");

    part_one(&binary.chars().collect::<Vec<_>>());
    part_two(&binary.chars().collect::<Vec<_>>());
}
