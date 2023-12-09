use regex::Regex;
use std::collections::HashMap;

pub fn solution(input: &str) -> u32 {
    let mut lines = input.lines();

    let directions = Direction::from_line(lines.next().expect("Directions should always exist"));
    let instructions = Instructions::from_lines(&mut lines).0;

    let mut cur_node = "AAA";
    let mut steps = 0;

    for direction in directions.iter().cycle() {
        if cur_node == "ZZZ" {
            break;
        }

        let node = instructions
            .get(cur_node)
            .expect("Current node should always exist");

        // Resolves next node
        cur_node = match direction {
            Direction::Left => node.0,
            Direction::Right => node.1,
        };
        // Increment steps
        steps += 1;
    }

    steps
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
        let line_regex = Regex::new(r"([A-Z]{3})").expect("Valid Regex");

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
