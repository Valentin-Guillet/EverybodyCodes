use crate::args::RunArgs;

use std::{collections::VecDeque, fs::read_to_string};

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    match args.part {
        1 | 2 => compute_water_time(&map),
        3 => compute_water_time_from_well(&map),
        _ => unreachable!(),
    }
}

fn parse_map(map: &[Vec<char>]) -> (Vec<(usize, usize)>, u32) {
    let mut start_positions: Vec<(usize, usize)> = Vec::new();
    let mut nb_palm_trees = 0;
    for (row, line) in map.iter().enumerate() {
        for (col, &c) in line.iter().enumerate() {
            if (row == 0 || row == map.len() - 1 || col == 0 || col == map[0].len() - 1) && c == '.'
            {
                start_positions.push((row, col));
            }

            if c == 'P' {
                nb_palm_trees += 1;
            }
        }
    }
    (start_positions, nb_palm_trees)
}

fn get_neighbors(map: &[Vec<char>], (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 && map[x - 1][y] != '#' {
        neighbors.push((x - 1, y));
    }
    if y < map[0].len() - 1 && map[x][y + 1] != '#' {
        neighbors.push((x, y + 1));
    }
    if x < map.len() - 1 && map[x + 1][y] != '#' {
        neighbors.push((x + 1, y));
    }
    if y > 0 && map[x][y - 1] != '#' {
        neighbors.push((x, y - 1));
    }
    neighbors
}

fn compute_water_time(map: &[Vec<char>]) -> u32 {
    let (start_positions, mut nb_palm_trees) = parse_map(map);

    let mut added = vec![vec![false; map[0].len()]; map.len()];
    for start_pos in &start_positions {
        added[start_pos.0][start_pos.1] = true;
    }

    let mut queue: VecDeque<((usize, usize), u32)> =
        VecDeque::from_iter(start_positions.iter().map(|&start| (start, 0)));
    while let Some(((x, y), dist)) = queue.pop_front() {
        if map[x][y] == 'P' {
            nb_palm_trees -= 1;
            if nb_palm_trees == 0 {
                return dist;
            }
        }

        for (nx, ny) in get_neighbors(map, (x, y)) {
            if added[nx][ny] {
                continue;
            }
            added[nx][ny] = true;
            queue.push_back(((nx, ny), dist + 1));
        }
    }
    unreachable!();
}

fn compute_water_time_from_well(map: &[Vec<char>]) -> u32 {
    let palm_trees: Vec<(usize, usize)> = (0..map.len())
        .flat_map(|row| {
            map[row]
                .iter()
                .enumerate()
                .filter(|&(_, &c)| c == 'P')
                .map(|(col, _)| (row, col))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    let mut palm_dists = vec![vec![vec![u32::MAX; palm_trees.len()]; map[0].len()]; map.len()];
    for (ind, &palm_tree) in palm_trees.iter().enumerate() {
        palm_dists[palm_tree.0][palm_tree.1][ind] = 0;
        queue.push_back((palm_tree, ind));
    }

    while let Some(((x, y), palm_id)) = queue.pop_front() {
        let dist = palm_dists[x][y][palm_id];
        for (nx, ny) in get_neighbors(map, (x, y)) {
            let ndist = &mut palm_dists[nx][ny][palm_id];
            if *ndist < u32::MAX {
                continue;
            }
            *ndist = dist + 1;
            queue.push_back(((nx, ny), palm_id));
        }
    }

    (0..map.len())
        .flat_map(|row| {
            (0..map[0].len())
                .filter(|&col| {
                    !palm_trees.contains(&(row, col)) && !palm_dists[row][col].contains(&u32::MAX)
                })
                .map(|col| palm_dists[row][col].iter().sum::<u32>())
                .collect::<Vec<_>>()
        })
        .min()
        .unwrap()
}
