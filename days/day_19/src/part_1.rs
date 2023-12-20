use regex::Regex;
use std::collections::HashMap;

///
/// Not the greatest implementation, but it works!
///
pub fn solution(input: &str) -> u32 {
    let mut iter = input.lines();
    let mut workflows: HashMap<&str, Workflow> = HashMap::new();
    let mut parts_ratings: Vec<PartRatings> = Vec::new();

    let mut accepted: Vec<PartRatings> = Vec::new();

    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }

        let workflow = Workflow::from(line);
        workflows.insert(workflow.identifier, workflow);
    }

    while let Some(line) = iter.next() {
        let part_ratings = PartRatings::from(line);
        parts_ratings.push(part_ratings);
    }

    for part_ratings in parts_ratings {
        let mut result = "in";

        loop {
            if result == "A" {
                accepted.push(part_ratings);
                break;
            } else if result == "R" {
                break;
            }

            let workflow = workflows.get(result).unwrap();
            let mut temp_result: Option<&str> = None;

            for rule in &workflow.rules {
                match rule.apply_to_part_ratings(&part_ratings) {
                    Some(identifier) => {
                        temp_result = Some(identifier);
                        break;
                    }
                    None => continue,
                };
            }

            match temp_result {
                Some(temp_result) => {
                    result = temp_result;
                }
                None => {
                    result = workflow.output;
                }
            };
        }
    }

    let mut sum = 0;

    for part_rating in &accepted {
        sum += part_rating
            .parts
            .iter()
            .map(|part| match part {
                Part::X(v) => v,
                Part::M(v) => v,
                Part::A(v) => v,
                Part::S(v) => v,
            })
            .sum::<u32>();
    }

    sum
}

///
/// Workflow
///
#[derive(Debug)]
struct Workflow<'a> {
    identifier: &'a str,
    rules: Vec<Rule<'a>>,
    output: &'a str,
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(value: &'a str) -> Self {
        let regex = Regex::new(r"(\w+)\{(.+)\}").expect("Valid regex");
        let captures = regex.captures(value).expect("Capture groups");

        let identifier = captures.get(1).map(|m| m.as_str()).unwrap();
        let rules_str = captures
            .get(2)
            .map(|m| m.as_str())
            .unwrap()
            .split(",")
            .collect::<Vec<_>>();
        let output = rules_str[rules_str.len() - 1];

        let mut rules: Vec<Rule> = vec![];

        for index in 0..rules_str.len() - 1 {
            let rule = rules_str[index];
            let rule = Rule::from(rule);
            rules.push(rule);
        }

        Self {
            identifier,
            rules,
            output,
        }
    }
}

///
/// Rule
///
#[derive(Debug)]
struct Rule<'a> {
    part: Part,
    token: Token,
    output: &'a str,
}

impl Rule<'_> {
    fn apply_to_part_ratings(&self, part_ratings: &PartRatings) -> Option<&str> {
        match &self.part {
            Part::X(value) => {
                let x = part_ratings
                    .parts
                    .iter()
                    .find(|part| match part {
                        Part::X(_) => true,
                        _ => false,
                    })
                    .map(|part| match part {
                        Part::X(v) => v,
                        _ => unreachable!(),
                    })
                    .unwrap();

                match &self.token {
                    Token::LessThan => {
                        if x < value {
                            return Some(self.output);
                        }
                    }
                    Token::GreaterThan => {
                        if x > value {
                            return Some(self.output);
                        }
                    }
                }
            }
            Part::M(value) => {
                let m = part_ratings
                    .parts
                    .iter()
                    .find(|part| match part {
                        Part::M(_) => true,
                        _ => false,
                    })
                    .map(|part| match part {
                        Part::M(v) => v,
                        _ => unreachable!(),
                    })
                    .unwrap();

                match &self.token {
                    Token::LessThan => {
                        if m < value {
                            return Some(self.output);
                        }
                    }
                    Token::GreaterThan => {
                        if m > value {
                            return Some(self.output);
                        }
                    }
                }
            }
            Part::A(value) => {
                let a = part_ratings
                    .parts
                    .iter()
                    .find(|part| match part {
                        Part::A(_) => true,
                        _ => false,
                    })
                    .map(|part| match part {
                        Part::A(v) => v,
                        _ => unreachable!(),
                    })
                    .unwrap();

                match &self.token {
                    Token::LessThan => {
                        if a < value {
                            return Some(self.output);
                        }
                    }
                    Token::GreaterThan => {
                        if a > value {
                            return Some(self.output);
                        }
                    }
                }
            }
            Part::S(value) => {
                let s = part_ratings
                    .parts
                    .iter()
                    .find(|part| match part {
                        Part::S(_) => true,
                        _ => false,
                    })
                    .map(|part| match part {
                        Part::S(v) => v,
                        _ => unreachable!(),
                    })
                    .unwrap();

                match &self.token {
                    Token::LessThan => {
                        if s < value {
                            return Some(self.output);
                        }
                    }
                    Token::GreaterThan => {
                        if s > value {
                            return Some(self.output);
                        }
                    }
                }
            }
        };

        None
    }
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(value: &'a str) -> Self {
        let regex = Regex::new(r"(\w)(<|>)(\d+):(\w+)").expect("Valid regex");
        let captures = regex.captures(value).expect("Capture groups");
        let part_type = captures.get(1).unwrap().as_str();
        let token = captures.get(2).unwrap().as_str();
        let part_value = captures
            .get(3)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .expect("Part value should always be parsable");
        let output = captures.get(4).unwrap().as_str();

        Self {
            part: Part::new(part_type, part_value),
            token: Token::from(token),
            output,
        }
    }
}

///
/// Part ratings
///
#[derive(Debug)]
struct PartRatings {
    parts: [Part; 4],
}

impl From<&str> for PartRatings {
    fn from(value: &str) -> Self {
        let regex = Regex::new(r"(\w)\=(\d+)").expect("Valid regex");
        let captures = regex.captures_iter(value);
        let mut parts: [Part; 4] = [Part::X(0); 4];

        let mut index = 0;
        for capture in captures {
            let type_str = capture.get(1).unwrap().as_str();
            let type_value = capture
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .expect("Value should always be parsable");

            let part = Part::new(type_str, type_value);
            parts[index] = part;
            index += 1;
        }

        Self { parts }
    }
}

///
/// Part
///
#[derive(Debug, Clone, Copy)]
enum Part {
    X(u32),
    M(u32),
    A(u32),
    S(u32),
}

impl Part {
    fn new(type_str: &str, value: u32) -> Self {
        match type_str {
            "x" => Self::X(value),
            "m" => Self::M(value),
            "a" => Self::A(value),
            "s" => Self::S(value),
            _ => unreachable!(),
        }
    }
}

///
/// Token
///
#[derive(Debug)]
enum Token {
    LessThan,
    GreaterThan,
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            ">" => Self::GreaterThan,
            "<" => Self::LessThan,
            _ => unreachable!(),
        }
    }
}
