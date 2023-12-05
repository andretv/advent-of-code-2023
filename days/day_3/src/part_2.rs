pub fn solution(input: &str) -> u32 {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut number_segments: Vec<u32> = vec![];

    for (x, line) in matrix.iter().enumerate() {
        for (y, element) in line.iter().enumerate() {
            if *element == '*' {
                let neighbor_numbers_pos = get_neighbor_numbers_pos(&matrix, x, y);

                if neighbor_numbers_pos.len() < 2 {
                    continue;
                }

                let mut gear_numbers: Vec<u32> = vec![];

                for pos in &neighbor_numbers_pos {
                    let line = &matrix[pos.0];
                    let segment = get_number_segment(&line, pos.1)
                        .parse::<u32>()
                        .expect("Number should always be parseble");

                    if !gear_numbers.contains(&segment) {
                        gear_numbers.push(segment);
                    }
                }

                if gear_numbers.len() == 2 {
                    let mul: u32 = multiply_vec(gear_numbers);
                    number_segments.push(mul);
                }
            }
        }
    }

    number_segments.iter().sum()
}

///
/// Multiply the vector first 2 number.
///
fn multiply_vec(vec: Vec<u32>) -> u32 {
    vec[0] * vec[1]
}

///
/// Gets gears neighbors position that are numbers.
///
fn get_neighbor_numbers_pos(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbor_numbers: Vec<(usize, usize)> = vec![];
    let mut neighbors_position = [(x + 1, y), (x, y + 1), (x + 1, y + 1)].to_vec();

    if x > 0 {
        neighbors_position.extend_from_slice(&[(x - 1, y), (x - 1, y + 1)]);
    }

    if y > 0 {
        neighbors_position.extend_from_slice(&[(x, y - 1), (x + 1, y - 1)]);
    }

    if x > 0 && y > 0 {
        neighbors_position.extend_from_slice(&[(x - 1, y - 1)]);
    }

    for pos in &neighbors_position {
        let Some(line) = matrix.get(pos.0) else {
            continue;
        };

        let Some(neighbor) = line.get(pos.1) else {
            continue;
        };

        if neighbor.is_numeric() {
            neighbor_numbers.push(*pos);
        }
    }

    neighbor_numbers
}

///
/// Concat number sequences.
///
fn get_number_segment(line: &Vec<char>, y: usize) -> String {
    let mut left_chars: Vec<char> = vec![];
    let mut right_chars: Vec<char> = vec![];

    let mut temp = y;

    // Left part
    if temp > usize::MIN {
        temp -= 1;
    }

    while temp >= usize::MIN && line[temp].is_numeric() {
        left_chars.push(line[temp]);

        if temp == 0 {
            break;
        }

        temp -= 1;
    }

    // Right part
    temp = y;
    if temp < line.len() {
        temp += 1;
    }

    while temp < line.len() && line[temp].is_numeric() {
        right_chars.push(line[temp]);
        temp += 1;
    }

    // Concat part
    left_chars.reverse();

    let left: String = left_chars.into_iter().collect();
    let right: String = right_chars.into_iter().collect();

    format!("{}{}{}", left, line[y], right)
}
