pub fn solution(input: &str) -> u32 {
    let dig_plan = DigPlan::from(input);
    let mut trench = Trench::new(&dig_plan);
    trench.dig_inside();
    trench.digged.len() as u32
}

///
/// Trench
///
#[derive(Debug)]
struct Trench {
    digged: Vec<(isize, isize)>,
}

impl Trench {
    fn dig_inside(&mut self) {
        let mut inside_points = vec![];

        let mut min_line = isize::MAX;
        let mut max_line = 0;
        let mut min_column = isize::MAX;
        let mut max_column = 0;
        for (line, column) in &self.digged {
            if line > &max_line {
                max_line = *line;
            }
            if line < &min_line {
                min_line = *line;
            }
            if column > &max_column {
                max_column = *column;
            }
            if column < &min_column {
                min_column = *column;
            }
        }

        for line_index in min_line..=max_line {
            for column_index in min_column..=max_column {
                let point = (line_index, column_index);

                if self.digged.contains(&point) {
                    continue;
                }

                if is_point_inside_polygon(&point, &self.digged) {
                    inside_points.push(point);
                }
            }
        }

        self.digged.extend(inside_points);
    }

    fn new(dig_plan: &DigPlan) -> Self {
        let mut digged: Vec<(isize, isize)> = Vec::new();

        let mut cur_line_index = 0;
        let mut cur_column_index = 0;

        for instruction in &dig_plan.instructions {
            match instruction.direction {
                Direction::Up => {
                    let line_start_index = (cur_line_index as isize)
                        .checked_sub(instruction.meters as isize)
                        .unwrap_or(0);
                    for line_index in (line_start_index..=cur_line_index).rev() {
                        let point = (line_index, cur_column_index);
                        if !digged.contains(&point) {
                            digged.push(point);
                        }
                    }
                    cur_line_index = cur_line_index
                        .checked_sub(instruction.meters as isize)
                        .unwrap_or(0);
                }
                Direction::Down => {
                    for line_index in cur_line_index..=cur_line_index + instruction.meters as isize
                    {
                        let point = (line_index, cur_column_index);
                        if !digged.contains(&point) {
                            digged.push(point);
                        }
                    }
                    cur_line_index += instruction.meters as isize;
                }
                Direction::Left => {
                    let start_index = cur_column_index
                        .checked_sub(instruction.meters as isize)
                        .unwrap_or(0);
                    for column_index in (start_index..cur_column_index).rev() {
                        let point = (cur_line_index, column_index);
                        if !digged.contains(&point) {
                            digged.push(point);
                        }
                    }
                    cur_column_index = cur_column_index
                        .checked_sub(instruction.meters as isize)
                        .unwrap_or(0);
                }
                Direction::Right => {
                    for column_index in
                        cur_column_index..=cur_column_index + instruction.meters as isize
                    {
                        let point = (cur_line_index, column_index);
                        if !digged.contains(&point) {
                            digged.push(point);
                        }
                    }
                    cur_column_index += instruction.meters as isize;
                }
            };
        }

        Self { digged }
    }
}

///
/// Dig plan
///
#[derive(Debug)]
struct DigPlan {
    instructions: Vec<Instruction>,
}

impl From<&str> for DigPlan {
    fn from(value: &str) -> Self {
        let instructions = value
            .lines()
            .map(|line| {
                let mut split = line.split_whitespace();
                let direction_str = split
                    .next()
                    .expect("Direction str should always be present");
                let meters_str = split.next().expect("Meters str should always be present");

                Instruction {
                    direction: Direction::from(direction_str),
                    meters: meters_str
                        .parse()
                        .expect("Meters should always be parsable"),
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
    meters: u8,
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
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }
}

///
/// Helper function to check if a point is inside a polygon.
///
fn is_point_inside_polygon(point: &(isize, isize), polygon: &Vec<(isize, isize)>) -> bool {
    let x = point.0;
    let y = point.1;

    let mut inside = false;
    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let xi = polygon[i].0;
        let yi = polygon[i].1;
        let xj = polygon[j].0;
        let yj = polygon[j].1;

        let intersect = ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);

        if intersect {
            inside = !inside;
        }

        j = i;
    }

    inside
}
