pub fn solution(input: &str) -> u32 {
    let mut lines = input.lines();
    let time = parse_line(lines.next().expect("Times should always exist"));
    let record_distance = parse_line(lines.next().expect("Distances should always exist"));

    let mut hold_time = 1;
    let mut could_beat_record = 0;

    while hold_time < time {
        let traveled_distance = hold_time * (time - hold_time);

        if traveled_distance > record_distance {
            could_beat_record += 1;
        }

        hold_time += 1;
    }

    could_beat_record
}

///
/// Extract line number.
///
fn parse_line(input: &str) -> u64 {
    let list: Vec<_> = input
        .split_whitespace()
        .enumerate()
        .filter_map(|(index, number)| {
            if index == 0 {
                return None;
            }
            Some(number.to_string())
        })
        .collect();

    list.join("")
        .parse::<u64>()
        .expect("Number should always be parsable")
}
