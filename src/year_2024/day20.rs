use crate::args::RunArgs;

use std::{collections::VecDeque, fs::read_to_string};

// (x, y, direction)
type Pos = (usize, usize, usize);

// (x, y, direction, checkpoint_seen, time)
type State = (usize, usize, usize, usize, u32);

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    match args.part {
        1 => fly_for(&map, 100),
        2 => find_path(&map),
        3 => fly_for_max(&map, 384400),
        _ => unreachable!(),
    }
}

fn get_neighbors(map: &[Vec<char>], (x, y, dir): Pos) -> Vec<Pos> {
    let mut neighbors = Vec::new();
    if x > 0 && map[x - 1][y] != '#' && map[x - 1][y] != '~' && dir != 2 {
        neighbors.push((x - 1, y, 0));
    }
    if y < map[0].len() - 1 && map[x][y + 1] != '#' && map[x][y + 1] != '~' && dir != 3 {
        neighbors.push((x, y + 1, 1));
    }
    if x < map.len() - 1 && map[x + 1][y] != '#' && map[x + 1][y] != '~' && dir != 0 {
        neighbors.push((x + 1, y, 2));
    }
    if y > 0 && map[x][y - 1] != '#' && map[x][y - 1] != '~' && dir != 1 {
        neighbors.push((x, y - 1, 3));
    }
    neighbors
}

fn find_start(map: &[Vec<char>]) -> (usize, usize) {
    for (row, line) in map.iter().enumerate() {
        for (col, &c) in line.iter().enumerate() {
            if c == 'S' {
                return (row, col);
            }
        }
    }
    unreachable!();
}

fn fly_for(map: &[Vec<char>], nb_seconds: u32) -> u32 {
    let mut min_altitudes = vec![vec![vec![0; 4]; map[0].len()]; map.len()];

    let start_pos = find_start(map);

    for dir in 0..4 {
        min_altitudes[start_pos.0][start_pos.1][dir] = 1000;
    }
    let mut states: Vec<Pos> = Vec::from_iter((0..4).map(|dir| (start_pos.0, start_pos.1, dir)));
    for _ in 0..nb_seconds {
        let mut next_states: Vec<Pos> = Vec::new();
        while let Some((x, y, dir)) = states.pop() {
            let min_alt = min_altitudes[x][y][dir];
            for (nx, ny, ndir) in get_neighbors(map, (x, y, dir)) {
                let nalt = match map[nx][ny] {
                    '+' => min_alt + 1,
                    '-' => min_alt - 2,
                    '.' | 'S' => min_alt - 1,
                    _ => unreachable!(),
                };

                if nalt <= min_altitudes[nx][ny][ndir] {
                    continue;
                }
                min_altitudes[nx][ny][ndir] = nalt;
                next_states.push((nx, ny, ndir));
            }
        }

        states = next_states;
    }

    states.iter().map(|&(x, y, dir)| min_altitudes[x][y][dir]).max().unwrap()
}

fn find_path(map: &[Vec<char>]) -> u32 {
    let mut min_altitudes = vec![vec![vec![vec![0; 4]; 4]; map[0].len()]; map.len()];

    let start_pos = find_start(map);

    for dir in 0..4 {
        min_altitudes[start_pos.0][start_pos.1][dir][0] = 10000;
    }

    let mut queue: VecDeque<State> = VecDeque::from_iter((0..4).map(|dir| (start_pos.0, start_pos.1, dir, 0, 0)));
    while let Some((x, y, dir, checkpoints, time)) = queue.pop_front() {
        let min_alt = min_altitudes[x][y][dir][checkpoints];

        for (nx, ny, ndir) in get_neighbors(map, (x, y, dir)) {
            let nalt = match map[nx][ny] {
                '+' => min_alt + 1,
                '-' => min_alt - 2,
                _ => min_alt - 1,
            };

            let ncheckpoints = match (checkpoints, map[nx][ny]) {
                (0, 'A') => 1,
                (1, 'B') => 2,
                (2, 'C') => 3,
                _ => checkpoints,
            };

            if ncheckpoints == 3 && map[nx][ny] == 'S' && nalt >= 10000 {
                return time + 1;
            }

            if nalt <= min_altitudes[nx][ny][ndir][ncheckpoints] {
                continue;
            }
            min_altitudes[nx][ny][ndir][ncheckpoints] = nalt;
            queue.push_back((nx, ny, ndir, ncheckpoints, time + 1));
        }
    }
    unreachable!()
}

fn fly_for_max(map: &[Vec<char>], mut alt: u32) -> u32 {
    let (_, start_col) = find_start(map);
    let mut max_dist = 0;

    // Map is trivial
    // Move to good column:
    alt -= 2;

    // A whole section on the good column costs 6m of altitude
    max_dist += map.len() as u32 * (alt / 6);
    alt %= 6;

    // Keep descending for the last section
    let mut row = 0;
    while alt > 0 {
        max_dist += 1;
        alt = match map[row as usize][start_col - 2] {
            '+' => alt + 1,
            '-' => alt - 2,
            _ => alt - 1,
        };
        row += 1;
    }

    max_dist
}
