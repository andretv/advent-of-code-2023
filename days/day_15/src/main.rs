mod part_1;
mod part_2;

fn main() {
    let input = include_str!("assets/input.txt");

    let part_1_solution = part_1::solution(input);
    println!("Part 1 solution: {}", part_1_solution);

    let part_2_solution = part_2::solution(input);
    println!("Part 2 solution: {}", part_2_solution);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_ok() {
        let input = include_str!("assets/test_input.txt");

        let part_1_solution = part_1::solution(input);
        assert_eq!(part_1_solution, 1320);
    }

    #[test]
    fn part_2_ok() {
        let input = include_str!("assets/test_input.txt");

        let part_2_solution = part_2::solution(input);
        assert_eq!(part_2_solution, 400);
    }
}
