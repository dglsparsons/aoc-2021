use std::fs;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Element {
    value: usize,
    depth: usize,
}

fn explode(input: &mut Vec<Element>) -> bool {
    let mut has_changed = false;
    for i in 0..input.len() {
        let Element { depth, value } = input[i];
        if depth < 5 {
            continue;
        }
        has_changed = true;
        // else we need to explode :)
        if i > 0 {
            // Add value to the lefternmost neighbour
            input[i - 1].value += value;
        }
        if i + 2 < input.len() {
            input[i + 2].value += input[i + 1].value;
        }
        input[i + 1].value = 0;
        input[i + 1].depth -= 1;
        input.remove(i);

        break;
    }

    has_changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![
                Element { value: 0, depth: 4 },
                Element { value: 9, depth: 4 },
                Element { value: 2, depth: 3 },
                Element { value: 3, depth: 2 },
                Element { value: 4, depth: 1 },
            ],
            parse("[[[[0,9],2],3],4]"),
        );
    }

    #[test]
    fn test_explode() {
        let mut input = parse("[[[[[9,8],1],2],3],4]");
        println!("input {:?}", input);
        let did_explode = explode(&mut input);
        assert!(did_explode);
        assert_eq!(input, parse("[[[[0,9],2],3],4]"));
    }

    #[test]
    fn test_reduce() {
        let mut input = parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        let did_split = reduce(&mut input);
        assert!(did_split);
        assert_eq!(parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"), input,);
    }

    #[test]
    fn test_add() {
        let lhs = parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let rhs = parse("[1,1]");
        assert_eq!(parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), add(lhs, rhs));
    }

    #[test]
    fn test_magnitude() {
        let input = parse("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(1137, magnitude(input));
    }
}

fn reduce(input: &mut Vec<Element>) -> bool {
    let mut has_changed = false;
    for i in 0..input.len() {
        let Element { depth, value } = input[i];
        if value < 10 {
            continue;
        }
        has_changed = true;
        let (lhs, rhs) = (value / 2, value / 2 + value % 2);

        input[i].value = lhs;
        input[i].depth += 1;

        input.insert(
            i + 1,
            Element {
                depth: depth + 1,
                value: rhs,
            },
        );

        break;
    }

    has_changed
}

fn parse(line: &str) -> Vec<Element> {
    let mut depth = 0;
    let mut values = Vec::new();
    let mut chars = line.trim().chars();
    let mut to_parse = String::new();
    loop {
        let c = chars.next();
        if c == None {
            break values;
        }
        let c = c.unwrap();
        if c == '[' {
            depth += 1;
            continue;
        }
        if c == ',' || c == ']' {
            if to_parse.is_empty() {
                if c == ']' {
                    depth -= 1;
                }
                continue;
            }
            values.push(Element {
                depth,
                value: to_parse.parse::<usize>().unwrap(),
            });
            to_parse = String::new();
            if c == ']' {
                depth -= 1;
            }
            continue;
        }

        // else it's a number.
        to_parse.push(c);
    }
}

fn add(a: Vec<Element>, b: Vec<Element>) -> Vec<Element> {
    let mut out = Vec::new();

    for Element { depth, value } in a.iter().chain(b.iter()) {
        out.push(Element {
            value: *value,
            depth: depth + 1,
        });
    }

    loop {
        if explode(&mut out) {
            continue;
        }
        if reduce(&mut out) {
            continue;
        }
        break;
    }

    out
}

fn magnitude(mut input: Vec<Element>) -> usize {
    while input.len() > 1 {
        for i in 0..(input.len() - 1) {
            if input[i].depth == input[i + 1].depth {
                let value = 3 * input[i].value + 2 * input[i + 1].value;
                input[i].value = value;
                input[i].depth -= 1;
                input.remove(i + 1);
                break;
            }
        }
    }

    input[0].value
}

fn part_two(input: Vec<Vec<Element>>) {
    let mut max = 0;
    for (i, a) in input.iter().enumerate() {
        for (j, b) in input.iter().enumerate() {
            if i == j {
                continue;
            }

            let maga = magnitude(add(a.to_vec(), b.to_vec()));
            if maga > max {
                max = maga;
            }
            let magb = magnitude(add(b.to_vec(), a.to_vec()));
            if magb > max {
                max = magb;
            }
        }
    }

    println!("part two - {}", max);
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let input = file.lines().map(|l| parse(l)).collect::<Vec<_>>();

    part_one(input.clone());
    part_two(input);
}

fn part_one(input: Vec<Vec<Element>>) {
    let x = magnitude(
        input
            .iter()
            .cloned()
            .reduce(|a, b| add(a.to_vec(), b.to_vec()))
            .unwrap(),
    );

    println!("part one - {}", x);
}
