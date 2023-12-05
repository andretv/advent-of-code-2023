pub fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut iterator = line.chars().filter_map(|char| char.to_digit(10));

            let first_digit = iterator.next().expect("Should be a number");
            let last_digit = iterator.last();

            let stringifyed_result = match last_digit {
                Some(digit) => format!("{first_digit}{digit}"),
                None => format!("{first_digit}{first_digit}"),
            };

            stringifyed_result
                .parse::<u32>()
                .expect("Should parse to a valid number")
        })
        .sum()
}
