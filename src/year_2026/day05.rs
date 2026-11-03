use crate::args::RunArgs;

use std::collections::VecDeque;
use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let map: Vec<Vec<char>> = data
        .trim_end()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut land = reconstruct_land(&map);
    match args.part {
        1 => count_terrain(&land, '#'),
        2 => {
            reconstruct_sand(&mut land);
            count_terrain(&land, '=')
        }
        3 => {
            reconstruct_sand(&mut land);
            reconstruct_trees(&mut land);
            count_terrain(&land, '@')
        }
        _ => unreachable!(),
    }
}

fn count_terrain(land: &[Vec<char>], terrain: char) -> u32 {
    land.iter()
        .map(|line| line.iter().filter(|&&c| c == terrain).count())
        .sum::<usize>() as u32
}

fn get_neighbors((m, n): (usize, usize), (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            if new_x >= 0 && new_x < m as isize && new_y >= 0 && new_y < n as isize {
                neighbors.push((new_x as usize, new_y as usize));
            }
        }
    }
    neighbors
}

fn get_tree_neighbors((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    for dx in -2..=2isize {
        for dy in -2..=2isize {
            if (dx == 0 && dy == 0) || (dx.abs() == 2 && dy.abs() == 2) {
                continue;
            }
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            neighbors.push((new_x as usize, new_y as usize));
        }
    }
    neighbors
}

fn reconstruct_land(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let m = map.len();
    let n = map[0].len();
    let mut land: Vec<Vec<char>> = vec![vec!['.'; n]; m];
    land[m / 2][n / 2] = '#';

    let mut to_see: VecDeque<(usize, usize)> =
        VecDeque::from(get_neighbors((m, n), (m / 2, n / 2)));
    while let Some((x, y)) = to_see.pop_front() {
        if land[x][y] == '#' {
            continue;
        }
        let neighbors = get_neighbors((m, n), (x, y));
        let count = neighbors
            .iter()
            .filter(|&&(neigh_x, neigh_y)| {
                land[neigh_x][neigh_y] == '#' || map[neigh_x][neigh_y] == '1'
            })
            .count();

        if count < 5 {
            continue;
        }
        land[x][y] = '#';
        to_see.extend(
            neighbors
                .iter()
                .filter(|&&(neigh_x, neigh_y)| land[neigh_x][neigh_y] == '.'),
        );
    }
    land
}

fn reconstruct_sand(land: &mut Vec<Vec<char>>) {
    let m = land.len();
    let n = land[0].len();

    let mut to_see: VecDeque<(usize, usize)> = VecDeque::new();
    for x in 0..m {
        for y in 0..n {
            if land[x][y] != '#' {
                continue;
            }
            let neighbors = get_neighbors((m, n), (x, y));
            if !neighbors.iter().all(|&(neigh_x, neigh_y)| {
                land[neigh_x][neigh_y] == '#' || land[neigh_x][neigh_y] == '='
            }) {
                land[x][y] = '=';
                to_see.extend(
                    neighbors
                        .iter()
                        .filter(|&&(neigh_x, neigh_y)| land[neigh_x][neigh_y] == '#'),
                );
            }
        }
    }

    while let Some((x, y)) = to_see.pop_front() {
        if land[x][y] != '#' {
            continue;
        }
        let neighbors = get_neighbors((m, n), (x, y));
        let count = neighbors
            .iter()
            .filter(|&&(neigh_x, neigh_y)| land[neigh_x][neigh_y] == '=')
            .count();
        if count < 5 {
            continue;
        }
        land[x][y] = '=';
        to_see.extend(
            neighbors
                .iter()
                .filter(|&&(neigh_x, neigh_y)| land[neigh_x][neigh_y] == '#'),
        );
    }
}

fn reconstruct_trees(land: &mut Vec<Vec<char>>) {
    let m = land.len();
    let n = land[0].len();

    for x in 2..m - 2 {
        for y in 2..n - 2 {
            if get_tree_neighbors((x, y))
                .iter()
                .all(|&(neigh_x, neigh_y)| {
                    land[neigh_x][neigh_y] == '#' || land[neigh_x][neigh_y] == '@'
                })
            {
                land[x][y] = '@';
            }
        }
    }

    for _ in 0..2 {
        let mut next_trees = Vec::new();
        for x in 0..m {
            for y in 0..n {
                if land[x][y] != '#' {
                    continue;
                }
                let neighbors = get_neighbors((m, n), (x, y));
                if neighbors.iter().any(|&(neigh_x, neigh_y)| land[neigh_x][neigh_y] == '=') {
                    continue;
                }
                let count = neighbors
                    .iter()
                    .filter(|&&(neigh_x, neigh_y)| land[neigh_x][neigh_y] == '@')
                    .count();
                if count >= 4 {
                    next_trees.push((x, y));
                }
            }
        }
        for (x, y) in next_trees {
            land[x][y] = '@';
        }
    }
}
