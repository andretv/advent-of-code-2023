use regex::Regex;

///
/// Calculates the sum of the power of the minimum set of cubes that must be present in a game.
///
pub fn solution(input: &str) -> u32 {
    let red_cubes_regex = Regex::new(r"(\d+) red").expect("Valid regex");
    let green_cubes_regex = Regex::new(r"(\d+) green").expect("Valid regex");
    let blue_cubes_regex = Regex::new(r"(\d+) blue").expect("Valid regex");

    input
        .lines()
        .map(|line| {
            let red_cubes = get_max_cubes_count(line, &red_cubes_regex);
            let green_cubes = get_max_cubes_count(line, &green_cubes_regex);
            let blue_cubes = get_max_cubes_count(line, &blue_cubes_regex);
            red_cubes * green_cubes * blue_cubes
        })
        .sum()
}

///
/// Extract the same color max cubes count.
///
fn get_max_cubes_count(input: &str, regex: &Regex) -> u32 {
    regex
        .captures_iter(input)
        .map(|cube_count| {
            cube_count
                .get(1)
                .expect("At least one cube count regex match")
                .as_str()
                .parse::<u32>()
                .expect("Number of cubes should always be parsed")
        })
        .max()
        .expect("Red cubes should always have a maximum value")
}
