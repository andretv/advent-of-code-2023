use regex::Regex;
use std::collections::HashMap;

pub fn solution(input: &str) -> u64 {
    let mut lines = input.lines();

    let directions = Direction::from_line(lines.next().expect("Directions should always exist"));
    let instructions = Instructions::from_lines(&mut lines).0;

    let starting_nodes: Vec<_> = instructions
        .clone()
        .into_iter()
        .filter(|(key, _)| key.ends_with("A"))
        .collect();

    // Used to walk through the instructions
    let mut cur_nodes: Vec<_> = starting_nodes.iter().map(|node| node.0).collect();

    // Holds all the steps required to cycle through the instructions
    let mut cycles: Vec<u32> = vec![];

    for cur_node in &mut cur_nodes {
        let mut steps = 0;

        for direction in directions.iter().cycle() {
            if cur_node.ends_with("Z") {
                break;
            }

            let node = instructions
                .get(cur_node)
                .expect("Current node should always exist");

            // Resolves next node
            *cur_node = match direction {
                Direction::Left => node.0,
                Direction::Right => node.1,
            };

            steps += 1;
        }

        cycles.push(steps);
    }

    // Calculates the least common multiplier between all cycles
    let mut result: u64 = cycles[0] as u64;
    for &num in &cycles[1..] {
        result = least_common_multiplier(result, num as u64);
    }

    result
}

///
/// Calculates the least common divisor between two numbers.
///
fn least_common_multiplier(x: u64, y: u64) -> u64 {
    x * y / greatest_common_divisor(x, y)
}

///
/// Calculates the greatest common divisor between two numbers.
///
fn greatest_common_divisor(x: u64, y: u64) -> u64 {
    if y == 0 {
        return x;
    }

    return greatest_common_divisor(y, x % y);
}

///
/// Possible directions.
///
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_line(input: &str) -> Vec<Self> {
        input
            .chars()
            .map(|char| match char {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!("Direction char should always be L or R"),
            })
            .collect()
    }
}

///
/// Instructions wrapper.
///
struct Instructions<'a>(HashMap<&'a str, (&'a str, &'a str)>);

impl Instructions<'_> {
    fn from_lines<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Instructions<'a> {
        let mut instructions: HashMap<&str, (&str, &str)> = HashMap::new();
        let line_regex = Regex::new(r"([A-Z|0-9]{3})").expect("Valid Regex");

        while let Some(line) = lines.next() {
            if line.is_empty() {
                continue;
            }

            let mut captures = line_regex.captures_iter(&line);
            let source_node = captures
                .next()
                .expect("Source capture node should always exist")
                .extract::<1>()
                .0;

            let destination_nodes = (
                captures
                    .next()
                    .expect("First destination capture node should always exist")
                    .extract::<1>()
                    .0,
                captures
                    .next()
                    .expect("Second destination capture node should always exist")
                    .extract::<1>()
                    .0,
            );

            instructions.insert(source_node, destination_nodes);
        }

        Instructions(instructions)
    }
}
