pub fn solution(input: &str) -> u32 {
    let mut patterns: Vec<Pattern> = vec![];

    let mut iterator = input.lines();
    let mut temp_string = "".to_string();

    while let Some(line) = iterator.next() {
        if line.is_empty() {
            let pattern = Pattern::from(temp_string.as_str());
            patterns.push(pattern);
            temp_string = "".to_string();
            continue;
        }

        temp_string.push_str(line);
        temp_string.push('\n');
    }

    let pattern = Pattern::from(temp_string.as_str());
    patterns.push(pattern);

    let mut sum = 0;

    for pattern in &patterns {
        sum += pattern.summarize();
    }

    sum
}

///
/// Pattern
///
#[derive(Debug)]
struct Pattern {
    tiles: Vec<Vec<Tile>>,
}

impl Pattern {
    fn summarize(&self) -> u32 {
        if let Some(row_reflection) = self.find_row_reflection(0) {
            return (row_reflection.0 * 100) as u32;
        };

        if let Some(column_reflection) = self.find_column_reflection(0) {
            return column_reflection.0 as u32;
        }

        unreachable!()
    }

    fn find_row_reflection(&self, start: usize) -> Option<(usize, usize)> {
        let mut reflection_index = start + 1;
        let mut smugdes = 0;

        for row_index in (0..=start).rev() {
            if reflection_index > self.tiles.len() - 1 {
                break;
            }

            let row = &self.tiles[row_index];
            let reflection_row = &self.tiles[reflection_index];

            for (index, tile) in row.iter().enumerate() {
                if tile != &reflection_row[index] {
                    smugdes += 1;
                }
            }

            reflection_index += 1;
        }

        if smugdes == 1 {
            return Some((start + 1, start + 2));
        }

        if start < self.tiles.len() - 1 {
            return self.find_row_reflection(start + 1);
        }

        return None;
    }

    fn find_column_reflection(&self, start: usize) -> Option<(usize, usize)> {
        let mut reflection_index = start + 1;
        let mut smudges = 0;

        for column_index in (0..=start).rev() {
            if reflection_index > self.tiles[0].len() - 1 {
                break;
            }

            for row in &self.tiles {
                let tile = &row[column_index];
                let reflection_tile = &row[reflection_index];

                if tile != reflection_tile {
                    smudges += 1;
                }
            }

            reflection_index += 1;
        }

        if smudges == 1 {
            return Some((start + 1, start + 2));
        }

        if start < self.tiles[0].len() - 1 {
            return self.find_column_reflection(start + 1);
        }

        return None;
    }
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        let tiles = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '.' => Tile::Ash,
                        '#' => Tile::Rock,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Self { tiles }
    }
}

///
/// Tile
///
#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}
