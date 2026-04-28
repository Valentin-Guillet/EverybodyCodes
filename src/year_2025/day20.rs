use crate::args::RunArgs;

use std::{collections::VecDeque, fs::read_to_string};

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let (maze, start, end) = parse_maze(&data);

    match args.part {
        1 => count_trampo_pairs(&maze),
        2 => get_shortest_path(&maze, start, end, false),
        3 => get_shortest_path(&maze, start, end, true),
        _ => unreachable!(),
    }
}

fn parse_maze(input: &str) -> (Vec<Vec<bool>>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut lines = Vec::new();
    for (row, line) in input.trim_end().lines().enumerate() {
        let mut upper = Vec::new();
        let mut lower = Vec::new();

        for (col, c) in line[row..line.len() - row].chars().enumerate() {
            if col % 2 == 0 {
                upper.push(c != '#');
            } else {
                lower.push(c != '#');
            }
            if c == 'S' {
                start = (if col % 2 == 0 { 2 * row } else { 2 * row + 1 }, col / 2);
            } else if c == 'E' {
                end = (if col % 2 == 0 { 2 * row } else { 2 * row + 1 }, col / 2);
            }
        }

        lines.push(upper);
        if !lower.is_empty() {
            lines.push(lower);
        }
    }

    (lines, start, end)
}

fn count_trampo_pairs(maze: &[Vec<bool>]) -> u32 {
    let mut count = 0;
    for row in 0..maze.len() - 1 {
        let row_length = maze[row].len();
        let next_row_length = maze[row + 1].len();
        for col in 0..next_row_length {
            if !maze[row + 1][col] {
                continue;
            }
            count += maze[row][col] as u32;
            if row_length != next_row_length && col < row_length - 1 {
                count += maze[row][col + 1] as u32;
            }
        }
    }
    count
}

fn get_neighbors(maze: &[Vec<bool>], row: usize, col: usize) -> Vec<(usize, usize)> {
    // Odd rows: all cells have 3 neighbors so no need to check
    if row % 2 == 1 {
        return vec![
            (row + 1, col),     // below
            (row - 1, col),     // left
            (row - 1, col + 1), // right
        ];
    }

    let mut neighbors = Vec::new();
    // Above
    if row > 0 {
        neighbors.push((row - 1, col));
    }
    // Left
    if col > 0 {
        neighbors.push((row + 1, col - 1));
    }
    // Right
    if col < maze[row].len() - 1 {
        neighbors.push((row + 1, col));
    }
    neighbors
}

fn get_shortest_path(
    maze: &[Vec<bool>],
    start: (usize, usize),
    end: (usize, usize),
    rotate: bool,
) -> u32 {
    let nb_rows = maze.len();

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut seen = Vec::new();
    for line in maze {
        seen.push(vec![false; line.len()]);
    }

    while !queue.is_empty() {
        let ((x, y), dist) = queue.pop_front().unwrap();
        if seen[x][y] {
            continue;
        }
        seen[x][y] = true;

        let mut neighbors = get_neighbors(maze, x, y);
        // We can jump in place if the map rotates
        if rotate {
            neighbors.push((x, y));
        }
        for (mut neigh_x, mut neigh_y) in neighbors {
            // Translate coordinates into rotated frame
            if rotate {
                (neigh_x, neigh_y) = (nb_rows - neigh_x - 2 * neigh_y - 1, neigh_x / 2);
            }

            // Not a trampoline
            if !maze[neigh_x][neigh_y] {
                continue;
            }
            if (neigh_x, neigh_y) == end {
                return dist + 1;
            }

            if seen[neigh_x][neigh_y] {
                continue;
            }
            queue.push_back(((neigh_x, neigh_y), dist + 1));
        }
    }
    unreachable!()
}
