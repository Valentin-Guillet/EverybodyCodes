use crate::args::RunArgs;

use std::fs::read_to_string;
use std::hash::{DefaultHasher, Hasher};
use std::mem::swap;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let tiles: Vec<Vec<bool>> = data
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect();

    match args.part {
        1 => count_tiles(&tiles, 10),
        2 => count_tiles(&tiles, 2025),
        3 => count_tiles_on_pattern(&tiles, 1000000000),
        _ => unreachable!(),
    }
}

fn count_neighbors(tiles: &[Vec<bool>], m: usize, n: usize, row: usize, col: usize) -> u32 {
    let mut neigh_count = 0;
    if row > 0 && col > 0 && tiles[row - 1][col - 1] {
        neigh_count += 1;
    }
    if row < m - 1 && col > 0 && tiles[row + 1][col - 1] {
        neigh_count += 1;
    }
    if row > 0 && col < n - 1 && tiles[row - 1][col + 1] {
        neigh_count += 1;
    }
    if row < m - 1 && col < n - 1 && tiles[row + 1][col + 1] {
        neigh_count += 1;
    }
    neigh_count
}

fn run_step(
    tiles: &mut Vec<Vec<bool>>,
    next_tiles: &mut Vec<Vec<bool>>,
    m: usize,
    n: usize,
) -> u32 {
    let mut tile_count = 0;
    for row in 0..m {
        for col in 0..n {
            let neigh_count = count_neighbors(&tiles, m, n, row, col);
            if tiles[row][col] == (neigh_count % 2 == 1) {
                next_tiles[row][col] = true;
                tile_count += 1;
            } else {
                next_tiles[row][col] = false;
            }
        }
    }

    swap(tiles, next_tiles);
    tile_count
}

fn count_tiles(tiles: &[Vec<bool>], nb_rounds: usize) -> u32 {
    let m: usize = tiles.len();
    let n: usize = tiles[0].len();

    let mut tiles: Vec<Vec<bool>> = tiles.iter().cloned().collect();
    let mut next_tiles: Vec<Vec<bool>> = vec![vec![false; n]; m];

    let mut tile_count = 0;
    for _ in 0..nb_rounds {
        tile_count += run_step(&mut tiles, &mut next_tiles, m, n);
    }
    tile_count
}

fn get_hash(tiles: &[Vec<bool>]) -> u64 {
    let mut hasher = DefaultHasher::new();
    for line in tiles {
        let mut line_hash: u64 = 0;
        for b in line {
            line_hash <<= 1;
            if *b {
                line_hash |= 1;
            }
        }
        hasher.write_u64(line_hash);
    }
    hasher.finish()
}

fn count_trunc_neighbors(tiles: &[Vec<bool>], m: usize, n: usize, row: usize, col: usize) -> u32 {
    let mut neigh_count = 0;
    // North West
    if row > 0 && col > 0 && tiles[row - 1][col - 1] {
        neigh_count += 1;
    }

    // South West
    if col > 0 && (row < m - 1 && tiles[row + 1][col - 1] || row == m - 1 && tiles[row][col - 1]) {
        neigh_count += 1;
    }

    // North East
    if row > 0 && (col < n - 1 && tiles[row - 1][col + 1] || col == n - 1 && tiles[row - 1][col]) {
        neigh_count += 1;
    }

    // South East
    if tiles[(row + 1).min(m - 1)][(col + 1).min(n - 1)] {
        neigh_count += 1;
    }
    neigh_count
}

fn run_trunc_step(
    tiles: &mut Vec<Vec<bool>>,
    next_tiles: &mut Vec<Vec<bool>>,
    m: usize,
    n: usize,
) -> u32 {
    let mut tile_count = 0;
    for row in 0..m {
        for col in 0..n {
            let neigh_count = count_trunc_neighbors(&tiles, m, n, row, col);
            if tiles[row][col] == (neigh_count % 2 == 1) {
                next_tiles[row][col] = true;
                tile_count += 1;
            } else {
                next_tiles[row][col] = false;
            }
        }
    }

    swap(tiles, next_tiles);
    tile_count
}

fn tiles_have_pattern(tiles: &[Vec<bool>], m: usize, n: usize, pattern: &[Vec<bool>]) -> bool {
    let pm = pattern.len() / 2;
    let pn = pattern[0].len() / 2;

    for row in 0..pm {
        for col in 0..pn {
            if pattern[row][col] != tiles[m - pm + row][n - pn + col] {
                return false;
            }
        }
    }
    true
}

fn count_tiles_on_pattern(pattern: &[Vec<bool>], nb_rounds: usize) -> u32 {
    let m: usize = 17;
    let n: usize = 17;

    let mut tiles: Vec<Vec<bool>> = vec![vec![false; n]; m];
    let mut next_tiles: Vec<Vec<bool>> = vec![vec![false; n]; m];
    let mut seen_hashes: Vec<u64> = vec![get_hash(&tiles)];
    let mut valid_rounds_data: Vec<(usize, u32)> = Vec::new();

    let mut round = 0;
    let mut found_index = 0;
    while round < nb_rounds {
        let nb_tiles = run_trunc_step(&mut tiles, &mut next_tiles, m, n);

        let tile_hash = get_hash(&tiles);
        if let Some(id) = (0..valid_rounds_data.len()).find(|&i| seen_hashes[i] == tile_hash) {
            found_index = id;
            break;
        }
        seen_hashes.push(tile_hash);
        round += 1;

        let has_pattern = tiles_have_pattern(&tiles, m, n, pattern);
        if has_pattern {
            valid_rounds_data.push((round, 4 * nb_tiles));
        }
    }

    let end_index = (nb_rounds - found_index) % round;
    let nb_cycles = (nb_rounds - found_index) / round;
    let mut total_tiles = 0;
    for (index, nb_tiles) in valid_rounds_data {
        if index <= found_index {
            total_tiles += nb_tiles;
        } else if index <= end_index {
            total_tiles += (nb_cycles as u32 + 1) * nb_tiles;
        } else {
            total_tiles += nb_cycles as u32 * nb_tiles;
        }
    }

    total_tiles
}
