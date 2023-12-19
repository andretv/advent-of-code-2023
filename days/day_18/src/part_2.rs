pub fn solution(input: &str) -> u64 {
    let dig_plan = DigPlan::from(input);
    dig_plan.total_area()
}

///
/// Dig plan
///
#[derive(Debug)]
struct DigPlan {
    instructions: Vec<Instruction>,
}

impl DigPlan {
    fn total_area(&self) -> u64 {
        let mut accumulator: u64 = 0;
        let (mut row, mut column) = (0, 0);

        for instruction in &self.instructions {
            let (prev_row, prev_column) = (row, column);

            match instruction.direction {
                Direction::Up => row -= instruction.meters as u64,
                Direction::Right => column += instruction.meters as u64,
                Direction::Down => row += instruction.meters as u64,
                Direction::Left => column -= instruction.meters as u64,
            };

            accumulator += (column + prev_column) * (row - prev_row) + instruction.meters as u64;
        }
        accumulator / 2 + 1
    }
}

impl From<&str> for DigPlan {
    fn from(value: &str) -> Self {
        let instructions = value
            .lines()
            .map(|line| {
                let split = line.split_whitespace();
                let hex_code = &split.last().expect("HEX code should always be present")[2..8];
                let meters = u32::from_str_radix(&hex_code[..5], 16)
                    .expect("Meters should always be parsable");
                Instruction {
                    meters,
                    direction: Direction::from(&hex_code[5..]),
                }
            })
            .collect();

        Self { instructions }
    }
}

///
/// Instruction
///
#[derive(Debug)]
struct Instruction {
    direction: Direction,
    meters: u32,
}

///
/// Direction
///
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "3" => Self::Up,
            "1" => Self::Down,
            "2" => Self::Left,
            "0" => Self::Right,
            _ => unreachable!(),
        }
    }
}
