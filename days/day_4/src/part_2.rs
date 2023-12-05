pub fn solution(input: &str) -> u32 {
    let mut games: Vec<(Scratchcard, u32)> = input
        .lines()
        .map(|line| (Scratchcard::from_line(line), 1))
        .collect();

    for index in 0..games.len() {
        let (scratchcard, parent_amount) = games.get(index).expect("Game should always exist.");
        let matches = scratchcard.matches_count();
        // `parent_amount` needs to be cloned to be used
        let parent_amount: u32 = *parent_amount;

        for x in (index + 1)..=(index + matches as usize) {
            let Some((_, amount)) = games.get_mut(x) else {
                break;
            };

            *amount += 1_u32 * parent_amount;
        }
    }

    games.iter().map(|game| game.1).sum()
}

#[derive(Debug)]
struct Scratchcard {
    winning_numbers: Vec<u32>,
    scratched_numbers: Vec<u32>,
}

impl Scratchcard {
    fn from_line(line: &str) -> Self {
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

        Scratchcard {
            winning_numbers,
            scratched_numbers,
        }
    }

    ///
    /// Counts how many matches are between two vectors of u32.
    ///
    fn matches_count(&self) -> u32 {
        let mut matches = 0;

        for first_num in &self.winning_numbers {
            for second_num in &self.scratched_numbers {
                if first_num == second_num {
                    matches += 1;
                }
            }
        }

        matches
    }
}
