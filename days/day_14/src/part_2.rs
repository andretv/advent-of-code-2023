use std::collections::HashMap;

pub fn solution(input: &str) -> u32 {
    let mut grid = Grid::from(input);
    let mut seen_grids = HashMap::new();

    for index in 1..1_000_000_000 {
        grid.tilt_cycle();

        if let Some(seen_at) = seen_grids.insert(grid.clone(), index) {
            if (1_000_000_000 - index) % (index - seen_at) == 0 {
                break;
            }
        }
    }

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
#[derive(Clone, PartialEq, Eq, Hash)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn tilt_cycle(&mut self) {
        self.tilt(&Direction::North);
        self.tilt(&Direction::West);
        self.tilt(&Direction::South);
        self.tilt(&Direction::East);
    }

    fn tilt(&mut self, direction: &Direction) {
        match direction {
            Direction::North => {
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
            Direction::West => {
                for row_index in 0..self.tiles.len() {
                    for column_index in 0..self.tiles[row_index].len() {
                        let mut west_available_tile = None;

                        for temp_column_index in (0..column_index).rev() {
                            if &self.tiles[row_index][temp_column_index] == &Tile::Empty {
                                west_available_tile = Some(temp_column_index);
                                continue;
                            }
                            break;
                        }

                        if let Some(west_available_tile) = west_available_tile {
                            if &self.tiles[row_index][column_index] == &Tile::RoundRock {
                                self.tiles[row_index][column_index] = Tile::Empty;
                                self.tiles[row_index][west_available_tile] = Tile::RoundRock;
                            }
                        };
                    }
                }
            }
            Direction::South => {
                for column_index in 0..self.tiles[0].len() {
                    for row_index in (0..self.tiles.len()).rev() {
                        let mut south_available_tile = None;

                        for temp_row_index in (row_index + 1)..self.tiles.len() {
                            if &self.tiles[temp_row_index][column_index] == &Tile::Empty {
                                south_available_tile = Some(temp_row_index);
                                continue;
                            }
                            break;
                        }

                        if let Some(south_available_tile) = south_available_tile {
                            if &self.tiles[row_index][column_index] == &Tile::RoundRock {
                                self.tiles[row_index][column_index] = Tile::Empty;
                                self.tiles[south_available_tile][column_index] = Tile::RoundRock;
                            }
                        };
                    }
                }
            }
            Direction::East => {
                for row_index in 0..self.tiles.len() {
                    for column_index in (0..self.tiles[row_index].len()).rev() {
                        let mut east_available_tile = None;

                        for temp_column_index in (column_index + 1)..self.tiles[row_index].len() {
                            if &self.tiles[row_index][temp_column_index] == &Tile::Empty {
                                east_available_tile = Some(temp_column_index);
                                continue;
                            }
                            break;
                        }

                        if let Some(east_available_tile) = east_available_tile {
                            if &self.tiles[row_index][column_index] == &Tile::RoundRock {
                                self.tiles[row_index][column_index] = Tile::Empty;
                                self.tiles[row_index][east_available_tile] = Tile::RoundRock;
                            }
                        };
                    }
                }
            }
        };
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
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Tile {
    Empty,
    CubeRock,
    RoundRock,
}

///
/// Direction
///
#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}
