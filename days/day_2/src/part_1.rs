use regex::Regex;

///
/// Determines how much cubes are available in a game.
///
pub struct GameCubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

///
/// Returns the ids sum of valid games.
///
pub fn solution(input: &str, game_cubes: &GameCubes) -> u32 {
    let game_id_regex = Regex::new(r"Game (\d+):").expect("Valid regex");
    let red_cubes_regex = Regex::new(r"(\d+) red").expect("Valid regex");
    let green_cubes_regex = Regex::new(r"(\d+) green").expect("Valid regex");
    let blue_cubes_regex = Regex::new(r"(\d+) blue").expect("Valid regex");

    input
        .lines()
        .filter_map(|line| {
            let game_id = get_game_id(&game_id_regex, line);

            let red_cubes = get_cubes_count(&red_cubes_regex, line);
            for cube_count in &red_cubes {
                if cube_count > &game_cubes.red {
                    return None;
                }
            }

            let green_cubes = get_cubes_count(&green_cubes_regex, line);
            for cube_count in &green_cubes {
                if cube_count > &game_cubes.green {
                    return None;
                }
            }

            let blue_cubes = get_cubes_count(&blue_cubes_regex, line);
            for cube_count in &blue_cubes {
                if cube_count > &game_cubes.blue {
                    return None;
                }
            }

            Some(game_id)
        })
        .sum()
}

///
/// Extract game id from line.
///
fn get_game_id(regex: &Regex, input: &str) -> u32 {
    regex
        .captures(input)
        .expect("Game ID should always exists")
        .get(1)
        .expect("Game ID should always exists")
        .as_str()
        .parse()
        .expect("Game ID should always be parsed")
}

///
/// Extract the same color cubes count.
///
fn get_cubes_count(regex: &Regex, input: &str) -> Vec<u32> {
    regex
        .captures_iter(input)
        .map(|cube_count| {
            cube_count
                .get(1)
                .expect("At least one cube count match")
                .as_str()
                .parse()
                .expect("Number of cubes should always be parsed")
        })
        .collect()
}
