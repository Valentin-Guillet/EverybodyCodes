use crate::args::RunArgs;

use std::{fmt::Display, fs::read_to_string};

type Grid = Vec<Vec<char>>;
type GridRef<'a> = Vec<&'a mut [char]>;

#[derive(Clone, PartialEq)]
enum Status {
    None,
    Filled,
    Impossible,
}

pub fn run(args: &RunArgs) -> Box<dyn Display> {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let mut data: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    match args.part {
        1 => Box::new(read_and_fill(&mut data)),
        2 => Box::new(read_and_fill_all(&mut data)),
        3 => Box::new(read_and_fill_incomplete(&mut data)),
        _ => unreachable!(),
    }
}

fn get_split_grid_ref(grid: &mut Grid, row: usize, col: usize) -> GridRef {
    let row_range = (9 * row)..(9 * (row + 1) - 1);
    let col_range = (9 * col)..(9 * (col + 1) - 1);

    grid[row_range]
        .iter_mut()
        .map(|row| &mut row[col_range.clone()])
        .collect()
}

fn get_joined_grid_ref(grid: &mut Grid, row: usize, col: usize) -> GridRef {
    let row_range = (6 * row)..(6 * (row + 1) + 2);
    let col_range = (6 * col)..(6 * (col + 1) + 2);

    grid[row_range]
        .iter_mut()
        .map(|row| &mut row[col_range.clone()])
        .collect()
}

fn fill_grid(grid: &mut GridRef) -> Status {
    let columns: Vec<Vec<char>> = (2..6)
        .map(|col| [0, 1, 6, 7].map(|row| grid[row][col]).to_vec())
        .collect();

    let mut coords_dot: Vec<(usize, usize)> = Vec::new();
    for (row, line) in grid.iter_mut().skip(2).take(4).enumerate() {
        let line_has_joker = line.contains(&'?');
        for col in 2..6 {
            if line[col] != '.' {
                continue;
            }
            let common_chars: Vec<char> = columns[col - 2]
                .iter()
                .filter(|&c| *c != '?' && line.contains(c))
                .copied()
                .collect();

            let has_joker = line_has_joker || columns[col - 2].contains(&'?');
            match common_chars.len() {
                0 if has_joker => coords_dot.push((row + 2, col)),
                1 => line[col] = common_chars[0],
                _ => return Status::Impossible,
            }
        }
    }

    let mut status = Status::Filled;
    for (dot_row, dot_col) in coords_dot {
        let mut joker_count = 0;
        let mut joker_coords: (usize, usize) = (0, 0);
        let mut joker_in_line = true;
        for i in [0, 1, 6, 7] {
            if grid[i][dot_col] == '?' {
                joker_coords = (i, dot_col);
                joker_count += 1;
                joker_in_line = false;
            }
            if grid[dot_row][i] == '?' {
                joker_coords = (dot_row, i);
                joker_count += 1;
                joker_in_line = true;
            }
        }

        if joker_count > 1 {
            status = Status::None;
            continue;
        }

        let filled_chars: Vec<char> = if joker_in_line {
            [0, 1, 6, 7]
                .iter()
                .map(|&row| grid[row as usize][dot_col])
                .filter(|c| {
                    !(2..6)
                        .map(|row| grid[row][dot_col])
                        .collect::<Vec<_>>()
                        .contains(c)
                })
                .collect()
        } else {
            [0, 1, 6, 7]
                .iter()
                .map(|&col| grid[dot_row][col as usize])
                .filter(|c| !grid[dot_row][2..6].contains(c))
                .collect()
        };

        if filled_chars.len() > 1 {
            status = Status::None;
            continue;
        }
        let missing_char = filled_chars[0];

        grid[dot_row][dot_col] = missing_char;
        grid[joker_coords.0][joker_coords.1] = missing_char;
    }
    status
}

fn get_runic_word(grid: &GridRef) -> String {
    (2..6)
        .map(|row| (2..6).map(|col| grid[row][col]).collect::<String>())
        .collect()
}

fn get_runic_power(grid: &GridRef) -> u32 {
    let runic_word = get_runic_word(grid);
    runic_word
        .chars()
        .enumerate()
        .map(|(i, c)| (i + 1) as u32 * (c as u32 - 'A' as u32 + 1))
        .sum()
}

fn read_and_fill(grid: &mut Grid) -> String {
    let mut grid_ref = get_split_grid_ref(grid, 0, 0);
    fill_grid(&mut grid_ref);
    get_runic_word(&grid_ref)
}

fn read_and_fill_all(grids: &mut Grid) -> u32 {
    let mut runic_power = 0;
    let width = (grids[0].len()) / 9 + 1;
    let height = (grids.len()) / 9 + 1;

    for grid_row in 0..height {
        for grid_col in 0..width {
            let mut grid_ref = get_split_grid_ref(grids, grid_row, grid_col);
            fill_grid(&mut grid_ref);
            runic_power += get_runic_power(&grid_ref);
        }
    }
    runic_power
}

fn read_and_fill_incomplete(wall: &mut Grid) -> u32 {
    let mut runic_power = 0;
    let width = (wall[0].len() - 2) / 6;
    let height = (wall.len() - 2) / 6;

    let mut status_grid = vec![vec![Status::None; width]; height];
    let mut filled = true;
    while filled {
        filled = false;
        for (grid_row, status_line) in status_grid.iter_mut().enumerate() {
            for (grid_col, status) in status_line.iter_mut().enumerate() {
                if *status != Status::None {
                    continue;
                }
                filled = true;

                let mut grid_ref = get_joined_grid_ref(wall, grid_row, grid_col);
                *status = fill_grid(&mut grid_ref);
                if *status == Status::Filled {
                    runic_power += get_runic_power(&grid_ref);
                }
            }
        }
    }
    runic_power
}
