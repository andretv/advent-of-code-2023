pub fn solution(input: &str) -> u32 {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut number_segments: Vec<_> = vec![];

    for (x, line) in matrix.iter().enumerate() {
        let mut seg_added = false;
        for (y, element) in line.iter().enumerate() {
            if !seg_added && *element != '.' && neighbor_is_symbol(&matrix, x, y) {
                let segment = get_number_segment(&line, y);
                number_segments.push(segment);
                seg_added = true;
            }

            if !element.is_numeric() {
                seg_added = false;
            }
        }
    }

    number_segments
        .iter()
        .map(|num| {
            num.parse::<u32>()
                .expect("Number should always be parsable")
        })
        .sum()
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

///
/// Checks all char neighbors to see if any of them is a symbol.
///
fn neighbor_is_symbol(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
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

    for pos in neighbors_position {
        let Some(line) = matrix.get(pos.0) else {
            continue;
        };

        let Some(neighbor) = line.get(pos.1) else {
            continue;
        };

        if !neighbor.is_numeric() && *neighbor != '.' {
            return true;
        }
    }

    return false;
}
