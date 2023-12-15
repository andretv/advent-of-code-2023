pub fn solution(input: &str) -> u32 {
    let input = input
        .lines()
        .next()
        .expect("Input should always have a first line")
        .split(",")
        .collect::<Vec<&str>>();

    let mut total_sum: u32 = 0;

    for sequence in input {
        let mut sum = 0;
        sequence
            .chars()
            .map(|char| char as u8)
            .for_each(|ascii_value| {
                sum += ascii_value as u32;
                sum *= 17;
                sum %= 256;
            });
        total_sum += sum;
    }

    total_sum
}
