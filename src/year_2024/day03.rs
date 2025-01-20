use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> i32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let grid: Vec<&str> = data.lines().collect();

    match args.part {
        1 => run_part1(&grid),
        2 => run_part2(&grid),
        3 => run_part3(&grid),
        _ => unreachable!(),
    }
}


fn run_part1(grid: &[&str]) -> i32 {
    count_diggable_blocks(grid, can_dig)
}

fn run_part2(grid: &[&str]) -> i32 {
    count_diggable_blocks(grid, can_dig)
}

fn run_part3(grid: &[&str]) -> i32 {
    count_diggable_blocks(grid, can_dig_diagonaly)
}
fn can_dig(grid: &[Vec<bool>], row: usize, col: usize) -> bool {
    (1..grid.len() - 1).contains(&row)
        && (1..grid[0].len() - 1).contains(&col)
        && grid[row - 1][col]
        && grid[row][col - 1]
        && grid[row][col]
        && grid[row][col + 1]
        && grid[row + 1][col]
}

fn can_dig_diagonaly(grid: &[Vec<bool>], row: usize, col: usize) -> bool {
    can_dig(grid, row, col)
        && grid[row - 1][col - 1]
        && grid[row - 1][col + 1]
        && grid[row + 1][col - 1]
        && grid[row + 1][col + 1]
}

fn count_diggable_blocks(grid: &[&str], dig_fn: fn(&[Vec<bool>], usize, usize) -> bool) -> i32 {
    let mut ans = 0;
    let mut level: Vec<Vec<bool>> = grid
        .iter()
        .map(|row| row.chars().map(|c| c == '#').collect())
        .collect();
    loop {
        let nb_blocks: i32 = level
            .iter()
            .map(|row| row.iter().filter(|v| **v).count() as i32)
            .sum();

        if nb_blocks == 0 {
            break;
        }
        ans += nb_blocks;

        level = (0..level.len())
            .map(|row| {
                (0..level[0].len())
                    .map(|col| dig_fn(&level, row, col))
                    .collect()
            })
            .collect();
    }
    ans
}
