pub fn solution(input: &str) -> u32 {
    let mut grid = Grid::from(input);

    let mut counts: Vec<_> = vec![];

    for row_index in 0..grid.tiles.len() {
        grid.cast_bean((row_index, 0), &Direction::Right);
        counts.push(grid.count_energized_tiles());
        grid.reset_tiles();

        let col_len = grid.tiles[row_index].len();
        grid.cast_bean((row_index, col_len - 1), &Direction::Left);
        counts.push(grid.count_energized_tiles());
        grid.reset_tiles();
    }

    for col_index in 0..grid.tiles[0].len() {
        grid.cast_bean((0, col_index), &Direction::Down);
        counts.push(grid.count_energized_tiles());
        grid.reset_tiles();

        let row_len = grid.tiles.len();
        grid.cast_bean((row_len - 1, col_index), &Direction::Up);
        counts.push(grid.count_energized_tiles());
        grid.reset_tiles();
    }

    counts
        .into_iter()
        .max()
        .expect("Should have at least one count")
}

///
/// Grid
///
#[derive(Debug)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn count_energized_tiles(&self) -> u32 {
        let mut sum = 0;

        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::Empty { energized } => {
                        if *energized > 0 {
                            sum += 1;
                        }
                    }
                    Tile::Mirror {
                        style: _,
                        energized,
                    } => {
                        if *energized > 0 {
                            sum += 1;
                        }
                    }
                    Tile::Splitter {
                        style: _,
                        energized,
                    } => {
                        if *energized > 0 {
                            sum += 1;
                        }
                    }
                }
            }
        }

        sum
    }

    fn cast_bean(&mut self, start_pos: (usize, usize), direction: &Direction) {
        match direction {
            Direction::Right => {
                for col_index in start_pos.1..self.tiles[start_pos.0].len() {
                    self.energize_tile((start_pos.0, col_index));

                    if col_index == start_pos.1 && start_pos != (0, 0) {
                        continue;
                    }

                    match self.tiles[start_pos.0][col_index] {
                        Tile::Empty { energized: _ } => continue,
                        Tile::Mirror {
                            style,
                            energized: _,
                        } => {
                            match &style {
                                MirrorStyle::RightAngle => {
                                    self.cast_bean((start_pos.0, col_index), &Direction::Up)
                                }
                                MirrorStyle::LeftAngle => {
                                    self.cast_bean((start_pos.0, col_index), &Direction::Down)
                                }
                            }
                            break;
                        }
                        Tile::Splitter { style, energized } => match &style {
                            SplitterStyle::Horizontal => continue,
                            SplitterStyle::Vertical => {
                                if energized > 1 {
                                    break;
                                }

                                self.cast_bean((start_pos.0, col_index), &Direction::Up);
                                self.cast_bean((start_pos.0, col_index), &Direction::Down);
                                break;
                            }
                        },
                    }
                }
            }
            Direction::Left => {
                for col_index in (0..=start_pos.1).rev() {
                    self.energize_tile((start_pos.0, col_index));

                    if col_index == start_pos.1 {
                        continue;
                    }

                    match self.tiles[start_pos.0][col_index] {
                        Tile::Empty { energized: _ } => continue,
                        Tile::Mirror {
                            style,
                            energized: _,
                        } => {
                            match &style {
                                MirrorStyle::RightAngle => {
                                    self.cast_bean((start_pos.0, col_index), &Direction::Down)
                                }
                                MirrorStyle::LeftAngle => {
                                    self.cast_bean((start_pos.0, col_index), &Direction::Up)
                                }
                            }
                            break;
                        }
                        Tile::Splitter { style, energized } => match &style {
                            SplitterStyle::Horizontal => continue,
                            SplitterStyle::Vertical => {
                                if energized > 1 {
                                    break;
                                }

                                self.cast_bean((start_pos.0, col_index), &Direction::Up);
                                self.cast_bean((start_pos.0, col_index), &Direction::Down);
                                break;
                            }
                        },
                    }
                }
            }
            Direction::Up => {
                for row_index in (0..=start_pos.0).rev() {
                    self.energize_tile((row_index, start_pos.1));

                    if row_index == start_pos.0 {
                        continue;
                    }

                    match self.tiles[row_index][start_pos.1] {
                        Tile::Empty { energized: _ } => continue,
                        Tile::Mirror {
                            style,
                            energized: _,
                        } => {
                            match &style {
                                MirrorStyle::RightAngle => {
                                    self.cast_bean((row_index, start_pos.1), &Direction::Right);
                                }
                                MirrorStyle::LeftAngle => {
                                    self.cast_bean((row_index, start_pos.1), &Direction::Left);
                                }
                            }
                            break;
                        }
                        Tile::Splitter { style, energized } => match &style {
                            SplitterStyle::Vertical => continue,
                            SplitterStyle::Horizontal => {
                                if energized > 1 {
                                    break;
                                }

                                self.cast_bean((row_index, start_pos.1), &Direction::Left);
                                self.cast_bean((row_index, start_pos.1), &Direction::Right);
                                break;
                            }
                        },
                    }
                }
            }
            Direction::Down => {
                for row_index in start_pos.0..self.tiles.len() {
                    self.energize_tile((row_index, start_pos.1));

                    if row_index == start_pos.0 {
                        continue;
                    }

                    match self.tiles[row_index][start_pos.1] {
                        Tile::Empty { energized: _ } => continue,
                        Tile::Mirror {
                            style,
                            energized: _,
                        } => {
                            match &style {
                                MirrorStyle::RightAngle => {
                                    self.cast_bean((row_index, start_pos.1), &Direction::Left);
                                }
                                MirrorStyle::LeftAngle => {
                                    self.cast_bean((row_index, start_pos.1), &Direction::Right);
                                }
                            }
                            break;
                        }
                        Tile::Splitter { style, energized } => match &style {
                            SplitterStyle::Vertical => continue,
                            SplitterStyle::Horizontal => {
                                if energized > 1 {
                                    break;
                                }

                                self.cast_bean((row_index, start_pos.1), &Direction::Left);
                                self.cast_bean((row_index, start_pos.1), &Direction::Right);
                                break;
                            }
                        },
                    }
                }
            }
        }
    }

    fn reset_tiles(&mut self) {
        for row_index in 0..self.tiles.len() {
            for col_index in 0..self.tiles[row_index].len() {
                match self.tiles[row_index][col_index] {
                    Tile::Empty { energized: _ } => {
                        self.tiles[row_index][col_index] = Tile::Empty { energized: 0 }
                    }
                    Tile::Mirror {
                        style,
                        energized: _,
                    } => {
                        self.tiles[row_index][col_index] = Tile::Mirror {
                            style,
                            energized: 0,
                        }
                    }
                    Tile::Splitter {
                        style,
                        energized: _,
                    } => {
                        self.tiles[row_index][col_index] = Tile::Splitter {
                            style,
                            energized: 0,
                        }
                    }
                }
            }
        }
    }

    fn energize_tile(&mut self, position: (usize, usize)) {
        match self.tiles[position.0][position.1] {
            Tile::Empty { energized } => {
                self.tiles[position.0][position.1] = Tile::Empty {
                    energized: energized + 1,
                };
            }
            Tile::Mirror { style, energized } => {
                self.tiles[position.0][position.1] = Tile::Mirror {
                    style,
                    energized: energized + 1,
                };
            }
            Tile::Splitter { style, energized } => {
                self.tiles[position.0][position.1] = Tile::Splitter {
                    style,
                    energized: energized + 1,
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
                        '.' => Tile::Empty { energized: 0 },
                        '/' => Tile::Mirror {
                            style: MirrorStyle::RightAngle,
                            energized: 0,
                        },
                        '\\' => Tile::Mirror {
                            style: MirrorStyle::LeftAngle,
                            energized: 0,
                        },
                        '|' => Tile::Splitter {
                            style: SplitterStyle::Vertical,
                            energized: 0,
                        },
                        '-' => Tile::Splitter {
                            style: SplitterStyle::Horizontal,
                            energized: 0,
                        },
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
#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty {
        energized: u32,
    },
    Mirror {
        style: MirrorStyle,
        energized: u32,
    },
    Splitter {
        style: SplitterStyle,
        energized: u32,
    },
}

///
/// Mirror style
///
#[derive(Debug, Clone, Copy)]
enum MirrorStyle {
    RightAngle,
    LeftAngle,
}

///
/// Splitter style
///
#[derive(Debug, Clone, Copy)]
enum SplitterStyle {
    Vertical,
    Horizontal,
}

///
/// Direction
///
#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}
