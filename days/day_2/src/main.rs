mod part_1;
mod part_2;

use part_1::GameCubes;

fn main() {
    let input = include_str!("assets/input.txt");

    let part_1_solution = part_1::solution(
        input,
        &GameCubes {
            red: 12,
            green: 13,
            blue: 14,
        },
    );
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

        let part_1_solution = part_1::solution(
            input,
            &GameCubes {
                red: 12,
                green: 13,
                blue: 14,
            },
        );
        assert_eq!(part_1_solution, 8);
    }

    #[test]
    fn part_2_ok() {
        let input = include_str!("assets/test_input.txt");

        let part_2_solution = part_2::solution(input);
        assert_eq!(part_2_solution, 2286);
    }
}
