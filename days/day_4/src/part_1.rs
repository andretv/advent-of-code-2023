pub fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line_split = line
                .split_once("|")
                .expect("\"|\" should always be in line");

            let winning_numbers = line_split
                .0
                .split_once(":")
                .expect("Split once should always work")
                .1
                .split_whitespace()
                .map(|number| {
                    number
                        .parse::<u32>()
                        .expect("Number should always be parsed")
                })
                .collect::<Vec<u32>>();

            let scratched_numbers = line_split
                .1
                .split_whitespace()
                .map(|number| {
                    number
                        .parse::<u32>()
                        .expect("Number should always be parsed")
                })
                .collect::<Vec<u32>>();

            let matches = matches_count(&winning_numbers, &scratched_numbers);

            calculate_points(matches)
        })
        .sum::<u32>()
}

///
/// Calculate total points.
///
fn calculate_points(matches: u32) -> u32 {
    if matches <= 1 {
        return matches;
    }

    return 2_u32.pow(matches - 1);
}

///
/// Counts how many matches are between two vectors of u32.
///
fn matches_count(first_vec: &Vec<u32>, second_vec: &Vec<u32>) -> u32 {
    let mut matches = 0;

    for first_num in first_vec {
        for second_num in second_vec {
            if first_num == second_num {
                matches += 1;
            }
        }
    }

    matches
}
