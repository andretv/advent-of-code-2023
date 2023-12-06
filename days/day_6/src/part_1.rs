pub fn solution(input: &str) -> u32 {
    let mut lines = input.lines();
    let times: Vec<_> = parse_line(lines.next().expect("Times should always exist"));
    let times_distances: Vec<_> = times
        .iter()
        .zip(parse_line(
            lines.next().expect("Distances should always exist"),
        ))
        .collect();

    let mut ways_of_beating_records: Vec<u32> = vec![];

    for (time, record_distance) in times_distances {
        let mut hold_time = 1;
        let mut could_beat_record = 0;

        while &hold_time < time {
            let traveled_distance = hold_time * (time - hold_time);

            if traveled_distance > record_distance {
                could_beat_record += 1;
            }

            hold_time += 1;
        }

        ways_of_beating_records.push(could_beat_record);
    }

    multiply_vector_elements(&ways_of_beating_records)
}

///
/// Multiply all vector elements.
///
fn multiply_vector_elements(vector: &Vec<u32>) -> u32 {
    let mut result = 0;

    for num in vector {
        if result == 0 {
            result = *num;
            continue;
        }

        result *= num;
    }

    result
}

///
/// Extract line numbers into a vector.
///
fn parse_line(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .enumerate()
        .filter_map(|(index, number)| {
            if index == 0 {
                return None;
            }

            Some(
                number
                    .parse::<u32>()
                    .expect("Number should always be parsable"),
            )
        })
        .collect()
}
