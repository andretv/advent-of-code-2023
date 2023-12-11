use rayon::prelude::*;
use std::{
    collections::{HashSet, VecDeque},
    sync::atomic::{AtomicU32, Ordering},
};

pub fn solution(input: &str) -> u64 {
    let mut grid = Grid::from(input);
    let no_expansion_pair_positions = grid.get_galaxies_pair_positions();
    let no_expansion_sum = get_pair_positions_sum(&no_expansion_pair_positions, &grid);
    grid.expand(10);
    let ten_times_expansion_pair_positions = grid.get_galaxies_pair_positions();
    let ten_times_expansion_sum =
        get_pair_positions_sum(&ten_times_expansion_pair_positions, &grid);
    let sum_diff: u64 = (ten_times_expansion_sum - no_expansion_sum) as u64;

    let mut sum: u64 = no_expansion_sum as u64;
    let expansions: &[u64] = &[1, 10, 100, 1_000, 10_000, 100_000];

    for expansion in expansions {
        sum += sum_diff * expansion;
    }

    sum
}

fn get_pair_positions_sum(pair_positions: &Vec<PairPosition>, grid: &Grid) -> u32 {
    let sum = AtomicU32::new(0);

    pair_positions
        .par_iter()
        .for_each(|(source_pos, destination_pos)| {
            let Some(distance) = source_pos.shortest_path(destination_pos, &grid) else {
                unreachable!("All pair of positions in the grid should have a path");
            };

            sum.fetch_add(distance as u32, Ordering::SeqCst);
        });

    sum.load(Ordering::SeqCst)
}

///
/// Grid
///
#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn get_galaxies_pair_positions(&self) -> Vec<PairPosition> {
        let mut galaxies_positions: Vec<Position> = vec![];
        for row_index in 0..self.tiles.len() {
            for column_index in 0..self.tiles[row_index].len() {
                if self.tiles[row_index][column_index] == Tile::Galaxy {
                    galaxies_positions.push(Position {
                        row: row_index,
                        column: column_index,
                    });
                }
            }
        }

        let mut galaxies_pair_positions: Vec<PairPosition> = vec![];
        for (index, galaxy_position) in galaxies_positions.iter().enumerate() {
            for other_galaxy_position in &galaxies_positions[index..] {
                if galaxy_position == other_galaxy_position {
                    continue;
                }

                let pair_positions = (galaxy_position.clone(), other_galaxy_position.clone());
                if !galaxies_pair_positions.contains(&pair_positions) {
                    galaxies_pair_positions.push(pair_positions);
                }
            }
        }

        galaxies_pair_positions
    }

    fn expand(&mut self, times: u32) {
        let times = match times {
            0 => 0,
            _ => times - 1,
        };

        let empty_rows = self.get_empty_rows();
        let empty_columns = self.get_empty_columns();
        let empty_row_vec = {
            let mut temp = vec![];
            for _ in 0..self.tiles.len() {
                temp.push(Tile::Empty);
            }
            temp
        };

        let mut offset = 1;
        for empty_row in &empty_rows {
            for _ in 0..times {
                let empty_row_vec = empty_row_vec.clone();
                self.tiles.insert(*empty_row + offset, empty_row_vec);
            }
            offset += times as usize;
        }

        offset = 1;
        for empty_column in &empty_columns {
            for row_index in 0..self.tiles.len() {
                for _ in 0..times {
                    self.tiles[row_index].insert(*empty_column + offset, Tile::Empty);
                }
            }
            offset += times as usize;
        }
    }

    fn get_empty_columns(&self) -> Vec<usize> {
        let mut empty_columns = vec![];

        let mut column_index = 0;
        while column_index != self.tiles[0].len() {
            let mut is_empty = true;

            for row_index in 0..self.tiles.len() {
                if self.tiles[row_index][column_index] != Tile::Empty {
                    is_empty = false;
                }
            }
            if is_empty {
                empty_columns.push(column_index);
            }

            column_index += 1;
        }

        empty_columns
    }

    fn get_empty_rows(&self) -> Vec<usize> {
        let mut empty_rows = vec![];

        for row_index in 0..self.tiles.len() {
            let row = &self.tiles[row_index];

            if row.iter().all(|tile| tile == &Tile::Empty) {
                empty_rows.push(row_index);
            }
        }

        empty_rows
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
                        '#' => Tile::Galaxy,
                        _ => unreachable!("Input invalid"),
                    })
                    .collect()
            })
            .collect();

        Self { tiles }
    }
}

///
/// Position
///
type PairPosition = (Position, Position);
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn shortest_path(&self, other: &Self, grid: &Grid) -> Option<usize> {
        let mut visited: HashSet<Self> = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((self.clone(), 0));

        while let Some((current, steps)) = queue.pop_front() {
            if &current == other {
                return Some(steps);
            }

            for neighbor in current.neighbors(grid) {
                if visited.contains(&neighbor) {
                    continue;
                }

                visited.insert(neighbor.clone());
                queue.push_back((neighbor, steps + 1));
            }
        }

        None
    }

    fn neighbors(&self, grid: &Grid) -> Vec<Self> {
        let rows = grid.tiles.len();
        let columns = grid.tiles[0].len();
        let mut neighbors = vec![];

        if self.row > 0 {
            neighbors.push(Self {
                row: self.row - 1,
                column: self.column,
            });
        }
        if self.column > 0 {
            neighbors.push(Self {
                row: self.row,
                column: self.column - 1,
            });
        }
        if self.row < rows - 1 {
            neighbors.push(Self {
                row: self.row + 1,
                column: self.column,
            });
        }
        if self.column < columns - 1 {
            neighbors.push(Self {
                row: self.row,
                column: self.column + 1,
            });
        }

        neighbors
    }
}

///
/// Tile
///
#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    Empty,
    Galaxy,
}
