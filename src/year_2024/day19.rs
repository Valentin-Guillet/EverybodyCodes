use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> String {
    let data = read_to_string(&args.input_file).expect("Error opening input file");

    let (key, grid) = parse_input(&data);

    let nb_rounds = match args.part {
        1 => 1,
        2 => 100,
        3 => 1048576000,
        _ => unreachable!(),
    };

    decrypt_message(&grid, &key, nb_rounds)
}

fn parse_input(data: &str) -> (Vec<char>, Vec<Vec<char>>) {
    let mut data_it = data.lines();
    let key = data_it.next().unwrap().chars().collect();
    data_it.next();
    let grid = data_it.map(|line| line.chars().collect()).collect();
    (key, grid)
}

fn rotate_grid<T: Copy>(grid: &mut [Vec<T>], row: usize, col: usize, rotation: char) {
    let mut positions = [
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col + 1),
        (row + 1, col + 1),
        (row + 1, col),
        (row + 1, col - 1),
        (row, col - 1),
    ];
    if rotation == 'R' {
        positions.reverse();
    }
    let tmp_value = grid[positions[0].0][positions[0].1];
    for i in 0..7 {
        grid[positions[i].0][positions[i].1] = grid[positions[i + 1].0][positions[i + 1].1];
    }
    grid[positions[7].0][positions[7].1] = tmp_value;
}

fn compute_round_mapping(grid: &[Vec<char>], key: &[char]) -> Vec<Vec<(usize, usize)>> {
    let (m, n) = (grid.len(), grid[0].len());

    let mut key_index = 0;
    let mut round_mapping: Vec<Vec<(usize, usize)>> = (0..m)
        .map(|row| (0..n).map(|col| (row, col)).collect())
        .collect();

    for row in 1..m - 1 {
        for col in 1..n - 1 {
            rotate_grid(&mut round_mapping, row, col, key[key_index]);
            key_index = (key_index + 1) % key.len();
        }
    }
    round_mapping
}

fn compute_cycles(mapping: &[Vec<(usize, usize)>]) -> Vec<Vec<(usize, usize)>> {
    let (m, n) = (mapping.len(), mapping[0].len());

    let mut seen = vec![vec![false; n]; m];
    let mut cycles: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut nb_pos_in_cycles = 0;
    'outer: for row in 0..m {
        for col in 0..n {
            if seen[row][col] {
                continue;
            }
            seen[row][col] = true;

            let start_pos = (row, col);
            let mut cycle = Vec::from([(row, col)]);
            let mut pos = mapping[start_pos.0][start_pos.1];
            while pos != start_pos {
                cycle.push(pos);
                seen[pos.0][pos.1] = true;
                nb_pos_in_cycles += 1;
                pos = mapping[pos.0][pos.1];
            }

            cycles.push(cycle);
            if nb_pos_in_cycles == m * n {
                break 'outer;
            }
        }
    }
    cycles
}

fn decrypt_grid(grid: &[Vec<char>]) -> String {
    for line in grid {
        let mut beg_ind: usize = 0;
        for ind in 0..line.len() {
            if line[ind] == '>' {
                beg_ind = ind + 1;
            }
            if line[ind] == '<' {
                return line[beg_ind..ind].iter().collect();
            }
        }
    }
    unreachable!()
}

fn decrypt_cycle(
    input_grid: &[Vec<char>],
    output_grid: &mut [Vec<char>],
    cycle: &[(usize, usize)],
    nb_rounds: u32,
) {
    let n = cycle.len();
    for i in 0..n {
        let (in_x, in_y) = cycle[(i + nb_rounds as usize) % n];
        let (out_x, out_y) = cycle[i];
        output_grid[out_x][out_y] = input_grid[in_x][in_y];
    }
}

fn decrypt_message(grid: &[Vec<char>], key: &[char], nb_rounds: u32) -> String {
    // round_mapping[x][y] == initial position of character at (x, y) after a cycle of rotation
    let round_mapping = compute_round_mapping(grid, key);

    let cycles = compute_cycles(&round_mapping);

    let mut decrypted_grid = vec![vec!['.'; grid[0].len()]; grid.len()];
    for cycle in &cycles {
        decrypt_cycle(grid, &mut decrypted_grid, cycle, nb_rounds);
    }

    decrypt_grid(&decrypted_grid)
}

