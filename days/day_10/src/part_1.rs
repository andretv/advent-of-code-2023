pub fn solution(input: &str) -> u32 {
    let grid = Grid::from(input);
    let pipe_loop = grid.get_pipe_loop();
    let half_loop_size = pipe_loop.len() as f32 / 2.;
    half_loop_size.ceil() as u32 + 1
}

///
/// Grid
///
#[derive(Debug)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    ///
    /// Find and returns the pipe loop.
    ///
    fn get_pipe_loop(&self) -> Vec<(usize, usize)> {
        let mut pipe_loop: Vec<(usize, usize)> = vec![];
        let animal_pos = self.get_animal_position();
        pipe_loop.push(animal_pos);

        let mut cur = (animal_pos, Direction::None);

        loop {
            let (next_connection_pos, comming_from) = self.get_next_tile(cur.0, &cur.1);
            let next_tile = &self.tiles[next_connection_pos.0][next_connection_pos.1];

            if next_tile == &Tile::Animal {
                break;
            }

            pipe_loop.push(next_connection_pos);
            cur = (next_connection_pos, comming_from);
        }

        pipe_loop
    }

    ///
    /// For a given tile, returns the next tile position and direction that it is comming from in the loop.
    ///
    fn get_next_tile(
        &self,
        tile_pos: (usize, usize),
        comming_from: &Direction,
    ) -> ((usize, usize), Direction) {
        let tile = &self.tiles[tile_pos.0][tile_pos.1];

        // East
        let tile_east_row = self
            .tiles
            .get(tile_pos.0)
            .expect("East row should always exist");

        if tile_pos.1 < tile_east_row.len() - 1 && comming_from != &Direction::East {
            let tile_east = tile_east_row
                .get(tile_pos.1 + 1)
                .expect("East tile should always exist");

            if tile.is_connected(tile_east, &Direction::East) {
                return ((tile_pos.0, tile_pos.1 + 1), Direction::West);
            }
        }

        // West
        let tile_west_row = self
            .tiles
            .get(tile_pos.0)
            .expect("East row should always exist");

        if tile_pos.1 > 0 && comming_from != &Direction::West {
            let tile_west = tile_west_row
                .get(tile_pos.1 - 1)
                .expect("West tile should always exist");

            if tile.is_connected(tile_west, &Direction::West) {
                return ((tile_pos.0, tile_pos.1 - 1), Direction::East);
            }
        }

        // North
        if tile_pos.0 > 0 && comming_from != &Direction::North {
            let tile_north_row = self
                .tiles
                .get(tile_pos.0 - 1)
                .expect("North row should always exist");

            let tile_north = tile_north_row
                .get(tile_pos.1)
                .expect("North tile should always exist");

            if tile.is_connected(tile_north, &Direction::North) {
                return ((tile_pos.0 - 1, tile_pos.1), Direction::South);
            }
        }

        // South
        if tile_pos.0 < self.tiles.len() - 1 && comming_from != &Direction::South {
            let tile_south_row = self
                .tiles
                .get(tile_pos.0 + 1)
                .expect("South row should always exist");

            let tile_south = tile_south_row
                .get(tile_pos.1)
                .expect("South tile should always exist");

            if tile.is_connected(tile_south, &Direction::South) {
                return ((tile_pos.0 + 1, tile_pos.1), Direction::North);
            }
        }

        unreachable!("Get next line unreachable code");
    }

    ///
    /// Find animal row and column inside tiles.
    ///
    fn get_animal_position(&self) -> (usize, usize) {
        let animal_row_pos = self
            .tiles
            .iter()
            .position(|tiles| tiles.contains(&Tile::Animal))
            .expect("Animal tile row should always exist");

        let animal_column_pos = self.tiles[animal_row_pos]
            .iter()
            .position(|tile| tile == &Tile::Animal)
            .expect("Animal tile column should always exist");

        (animal_row_pos, animal_column_pos)
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let tiles = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '|' => Tile::VerticalPipe,
                        '-' => Tile::HorizontalPipe,
                        'L' => Tile::BendPipe([Direction::North, Direction::East]),
                        'J' => Tile::BendPipe([Direction::North, Direction::West]),
                        'F' => Tile::BendPipe([Direction::South, Direction::East]),
                        '7' => Tile::BendPipe([Direction::South, Direction::West]),
                        '.' => Tile::Ground,
                        'S' => Tile::Animal,
                        _ => unreachable!("Should not get here"),
                    })
                    .collect()
            })
            .collect();

        Self { tiles }
    }
}

///
/// Tiles
///
#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Ground,
    VerticalPipe,
    HorizontalPipe,
    BendPipe([Direction; 2]),
    Animal,
}

impl Tile {
    ///
    /// Checks if this tile is connected to another one.
    ///
    fn is_connected(&self, other: &Tile, other_direction: &Direction) -> bool {
        match self {
            // Vertical pipe
            Tile::VerticalPipe => match other {
                Tile::VerticalPipe => {
                    other_direction == &Direction::North || other_direction == &Direction::South
                }
                Tile::HorizontalPipe => false,
                Tile::BendPipe(directions) => {
                    other_direction == &Direction::North && directions.contains(&Direction::South)
                        || other_direction == &Direction::South
                            && directions.contains(&Direction::North)
                }
                Tile::Ground => false,
                Tile::Animal => true,
            },
            // Horizontal pipe
            Tile::HorizontalPipe => match other {
                Tile::VerticalPipe => false,
                Tile::HorizontalPipe => {
                    other_direction == &Direction::East || other_direction == &Direction::West
                }
                Tile::BendPipe(directions) => {
                    (other_direction == &Direction::East && directions.contains(&Direction::West))
                        || (other_direction == &Direction::West
                            && directions.contains(&Direction::East))
                }
                Tile::Ground => false,
                Tile::Animal => true,
            },
            // Bend pipe
            Tile::BendPipe(directions) => match other {
                Tile::VerticalPipe => {
                    (other_direction == &Direction::North && directions.contains(&Direction::North))
                        || (other_direction == &Direction::South
                            && directions.contains(&Direction::South))
                }
                Tile::HorizontalPipe => {
                    (other_direction == &Direction::East && directions.contains(&Direction::East))
                        || (other_direction == &Direction::West
                            && directions.contains(&Direction::West))
                }
                Tile::BendPipe(other_directions) => {
                    // North
                    if other_direction == &Direction::North
                        && directions.contains(&Direction::North)
                        && other_directions.contains(&Direction::South)
                    {
                        return true;
                    }

                    // South
                    if other_direction == &Direction::South
                        && directions.contains(&Direction::South)
                        && other_directions.contains(&Direction::North)
                    {
                        return true;
                    }

                    // East
                    if other_direction == &Direction::East
                        && directions.contains(&Direction::East)
                        && other_directions.contains(&Direction::West)
                    {
                        return true;
                    }

                    if other_direction == &Direction::West
                        && directions.contains(&Direction::West)
                        && other_directions.contains(&Direction::East)
                    {
                        return true;
                    }

                    return false;
                }
                Tile::Ground => false,
                Tile::Animal => true,
            },
            // Ground
            Tile::Ground => false,
            Tile::Animal => match other {
                Tile::VerticalPipe => true,
                Tile::HorizontalPipe => true,
                Tile::BendPipe(_) => true,
                Tile::Ground => false,
                Tile::Animal => false,
            },
        }
    }
}

///
/// Direction
///
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    West,
    East,
    None,
}
