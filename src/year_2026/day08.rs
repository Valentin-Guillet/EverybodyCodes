use crate::args::RunArgs;

use std::fs::read_to_string;

#[derive(Debug)]
enum Direction {
    BottomRight,
    BottomLeft,
    Left,
    TopLeft,
    TopRight,
    Right,
}

const ALL_DIRECTIONS: [Direction; 6] = [
    Direction::BottomRight,
    Direction::BottomLeft,
    Direction::Left,
    Direction::TopLeft,
    Direction::TopRight,
    Direction::Right,
];

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let mut map: Vec<Vec<char>> = data
        .trim_end()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    match args.part {
        1 => count_required_energy(&map, &Direction::Left),
        2 => ALL_DIRECTIONS
            .iter()
            .map(|direction| count_required_energy(&map, direction))
            .sum(),
        3 => grow_until(&mut map, 100000),
        _ => unreachable!(),
    }
}

fn step(map_size: (usize, usize), direction: &Direction, row: &mut usize, col: &mut usize) -> bool {
    match direction {
        Direction::BottomRight => {
            if *row == map_size.0 - 1 || *col == map_size.1 - 1 {
                return false;
            }
            *row += 1;
            *col += 1;
        }
        Direction::BottomLeft => {
            if *row == map_size.0 - 1 || *col == 0 {
                return false;
            }
            *row += 1;
            *col -= 1;
        }
        Direction::Left => {
            if *col <= 1 {
                return false;
            }
            *col -= 2;
        }
        Direction::TopLeft => {
            if *row == 0 || *col == 0 {
                return false;
            }
            *row -= 1;
            *col -= 1;
        }
        Direction::TopRight => {
            if *row == 0 || *col == map_size.1 - 1 {
                return false;
            }
            *row -= 1;
            *col += 1;
        }
        Direction::Right => {
            if *col >= map_size.1 - 2 {
                return false;
            }
            *col += 2;
        }
    }
    true
}

fn get_coords(map_size: (usize, usize), direction: &Direction) -> Vec<(usize, usize)> {
    match direction {
        Direction::BottomRight => (0..map_size.0)
            .map(|row| (row, 0))
            .chain((1..map_size.1).map(|col| (0, col)))
            .collect(),
        Direction::BottomLeft => (0..map_size.0)
            .map(|row| (row, map_size.1 - 1))
            .chain((0..map_size.1 - 1).map(|col| (0, col)))
            .collect(),
        Direction::Left => (0..map_size.0)
            .map(|row| {
                (
                    row,
                    map_size.1
                        - 1
                        - if row % 2 == (map_size.0 / 2) % 2 {
                            0
                        } else {
                            1
                        },
                )
            })
            .collect(),
        Direction::TopLeft => (0..map_size.0)
            .map(|row| (row, map_size.1 - 1))
            .chain((0..map_size.1 - 1).map(|col| (map_size.0 - 1, col)))
            .collect(),
        Direction::TopRight => (0..map_size.0)
            .map(|row| (row, 0))
            .chain((1..map_size.1).map(|col| (map_size.0 - 1, col)))
            .collect(),
        Direction::Right => (0..map_size.0)
            .map(|row| {
                (
                    row,
                    if row % 2 == (map_size.0 / 2) % 2 {
                        0
                    } else {
                        1
                    },
                )
            })
            .collect(),
    }
}

fn count_required_energy(map: &[Vec<char>], direction: &Direction) -> u32 {
    let map_size = (map.len(), map[0].len());
    let init_coords = get_coords(map_size, direction);
    let mut required_energy = 0;
    for (mut row, mut col) in init_coords {
        let mut shadow = 0;
        loop {
            if let Some(height) = map[row][col].to_digit(10) {
                if height > shadow {
                    required_energy += 1;
                    shadow = height;
                }
            }
            shadow = shadow.saturating_sub(1);
            if !step(map_size, direction, &mut row, &mut col) {
                break;
            }
        }
    }
    required_energy
}

fn grow_until(map: &mut [Vec<char>], nb_required_ents: u32) -> u32 {
    let map_size = (map.len(), map[0].len());
    let mut direction_index = 0;
    let all_init_coords: Vec<Vec<(usize, usize)>> = (0..ALL_DIRECTIONS.len())
        .map(|i| get_coords(map_size, &ALL_DIRECTIONS[i]))
        .collect();
    let mut nb_ents = 0;
    let mut required_energy = 0;
    while nb_ents < nb_required_ents {
        let direction = &ALL_DIRECTIONS[direction_index];
        for (mut row, mut col) in all_init_coords[direction_index].iter().cloned() {
            let mut shadow = 0;
            loop {
                if let Some(height) = map[row][col].to_digit(10) {
                    if height > shadow {
                        required_energy += 1;
                        shadow = height;
                        if height < 9 {
                            map[row][col] = char::from_digit(height + 1, 10).unwrap();
                        } else {
                            map[row][col] = 'X';
                            nb_ents += 1;
                        }
                    }
                } else if map[row][col] == 'X' {
                    map[row][col] = 'Y';
                } else if map[row][col] == 'Y' {
                    map[row][col] = 'Z';
                } else if map[row][col] == 'Z' {
                    map[row][col] = '1';
                }

                shadow = shadow.saturating_sub(1);
                if !step(map_size, direction, &mut row, &mut col) {
                    break;
                }
            }
        }
        direction_index = (direction_index + 1) % ALL_DIRECTIONS.len();
    }
    required_energy
}
