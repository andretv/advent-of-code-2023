//
// Incomplete!
//

pub fn solution(input: &str) -> u32 {
    let _grid = Grid::from(input);

    todo!()
}

///
/// Grid
///
#[derive(Debug)]
struct Grid {
    lines: Vec<LineInfo>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let lines = value
            .lines()
            .map(|line| {
                let (springs_str, contiguous_str) = line
                    .split_once(' ')
                    .expect("Should always split on whitespace");

                let line = springs_str
                    .chars()
                    .map(|spring| match spring {
                        '.' => Spring::Operational,
                        '#' => Spring::Damaged,
                        '?' => Spring::Unknown,
                        _ => unreachable!("Falty input"),
                    })
                    .collect();

                let contiguous = contiguous_str
                    .split(",")
                    .map(|number| {
                        number
                            .parse::<u8>()
                            .expect("Contiguous number should always be parsed")
                    })
                    .collect();

                LineInfo { line, contiguous }
            })
            .collect();

        Self { lines }
    }
}

///
/// Type for each input line.
///
#[derive(Debug)]
struct LineInfo {
    line: Vec<Spring>,
    contiguous: Vec<u8>,
}

impl LineInfo {
    // TODO
}

///
/// Spring
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}
