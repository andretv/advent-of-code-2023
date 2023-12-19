use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub fn solution(input: &str) -> u16 {
    let city = City::from(input);
    let starting_position = (0, 0);
    let destination_position = (city.blocks.len() - 1, city.blocks[0].len() - 1);
    city.get_minimized_heat_loss_path(starting_position, destination_position)
}

type Position = (usize, usize);

///
/// City
///
#[derive(Debug)]
struct City {
    blocks: Vec<Vec<Block>>,
}

impl City {
    fn get_minimized_heat_loss_path(&self, source_pos: Position, destination_pos: Position) -> u16 {
        let mut costs: HashMap<Crucible, u16> = HashMap::new();
        let mut binary_heap: BinaryHeap<Reverse<(u16, Crucible)>> = BinaryHeap::new();

        let crucible = Crucible::new(source_pos, Direction::Right);

        costs.insert(crucible, 0);
        binary_heap.push(Reverse((0, crucible)));

        while let Some(Reverse((current_cost, current_crucible))) = binary_heap.pop() {
            if current_crucible.position == destination_pos {
                return current_cost;
            }

            if current_cost > *costs.get(&current_crucible).unwrap_or(&u16::MAX) {
                continue;
            }

            let possible_routes = current_crucible.get_posible_routes(&self.blocks);

            for possibility in possible_routes {
                let next_position = possibility.position;
                let next_cost =
                    current_cost + self.blocks[next_position.0][next_position.1].0 as u16;
                if next_cost < *costs.get(&possibility).unwrap_or(&u16::MAX) {
                    binary_heap.push(Reverse((next_cost, possibility)));
                    costs.insert(possibility, next_cost);
                }
            }
        }

        unreachable!()
    }
}

impl From<&str> for City {
    fn from(value: &str) -> Self {
        let blocks = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| {
                        Block(char.to_digit(10).expect("Number should always be parsable") as u8)
                    })
                    .collect()
            })
            .collect();

        Self { blocks }
    }
}

///
/// Block
///
#[derive(Debug)]
struct Block(u8);

///
/// Crucible
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Crucible {
    position: Position,
    direction: Direction,
    steps_up: u8,
    steps_down: u8,
    steps_left: u8,
    steps_right: u8,
}

impl Crucible {
    fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
            steps_up: 0,
            steps_down: 0,
            steps_left: 0,
            steps_right: 0,
        }
    }

    fn get_posible_routes(&self, city_blocks: &Vec<Vec<Block>>) -> Vec<Self> {
        let mut possibilities = vec![];

        // Up
        if self.steps_up < 3 && self.position.0 > 0 && self.direction != Direction::Down {
            possibilities.push(Crucible {
                position: (self.position.0 - 1, self.position.1),
                direction: Direction::Up,
                steps_up: self.steps_up + 1,
                steps_down: 0,
                steps_left: 0,
                steps_right: 0,
            });
        }

        // Down
        if self.steps_down < 3
            && self.position.0 < city_blocks.len() - 1
            && self.direction != Direction::Up
        {
            possibilities.push(Crucible {
                position: (self.position.0 + 1, self.position.1),
                direction: Direction::Down,
                steps_up: 0,
                steps_down: self.steps_down + 1,
                steps_left: 0,
                steps_right: 0,
            });
        }

        // Left
        if self.steps_left < 3 && self.position.1 > 0 && self.direction != Direction::Right {
            possibilities.push(Crucible {
                position: (self.position.0, self.position.1 - 1),
                direction: Direction::Left,
                steps_up: 0,
                steps_down: 0,
                steps_left: self.steps_left + 1,
                steps_right: 0,
            });
        }

        // Right
        if self.steps_right < 3
            && self.position.1 < city_blocks[0].len() - 1
            && self.direction != Direction::Left
        {
            possibilities.push(Crucible {
                position: (self.position.0, self.position.1 + 1),
                direction: Direction::Right,
                steps_up: 0,
                steps_down: 0,
                steps_left: 0,
                steps_right: self.steps_right + 1,
            });
        }

        possibilities
    }
}

///
/// Direction
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
