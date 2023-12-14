pub fn solution(input: &str) -> u32 {
    let mut grid = Grid::from(input);
    grid.tilt_north();

    let mut sum = 0;
    let mut load = grid.tiles.len();
    for tile_row in &grid.tiles {
        for tile in tile_row {
            if tile == &Tile::RoundRock {
                sum += load;
            }
        }
        load -= 1;
    }

    sum as u32
}

///
/// Grid
///
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn tilt_north(&mut self) {
        for column_index in 0..self.tiles[0].len() {
            for row_index in 0..self.tiles.len() {
                let mut north_available_tile = None;

                for temp_row_index in (0..row_index).rev() {
                    if &self.tiles[temp_row_index][column_index] == &Tile::Empty {
                        north_available_tile = Some(temp_row_index);
                        continue;
                    }
                    break;
                }

                if let Some(north_available_tile) = north_available_tile {
                    if &self.tiles[row_index][column_index] == &Tile::RoundRock {
                        self.tiles[row_index][column_index] = Tile::Empty;
                        self.tiles[north_available_tile][column_index] = Tile::RoundRock;
                    }
                };
            }
        }
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let tiles = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '.' => Tile::Empty,
                        '#' => Tile::CubeRock,
                        'O' => Tile::RoundRock,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Self { tiles }
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                let tile = match tile {
                    Tile::Empty => '.',
                    Tile::CubeRock => '#',
                    Tile::RoundRock => 'O',
                };

                write!(f, "{}", tile)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

///
/// Tile
///
#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    CubeRock,
    RoundRock,
}
