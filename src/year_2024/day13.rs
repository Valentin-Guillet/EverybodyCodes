use crate::args::RunArgs;

use std::fs::read_to_string;

type Pos = (usize, usize);

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let maze: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    match args.part {
        1 | 2 => get_shortest_path(&maze, 'S', 'E'),
        3 => get_shortest_path(&maze, 'E', 'S'),
        _ => unreachable!(),
    }
}

fn get_pos(maze: &[Vec<char>], needle: char) -> Pos {
    maze.iter()
        .enumerate()
        .filter_map(|(row, line)| line.iter().position(|&c| c == needle).map(|col| (row, col)))
        .next()
        .unwrap()
}

fn get_neighbors(maze: &[Vec<char>], (x, y): Pos) -> Vec<Pos> {
    let mut neighbors = Vec::new();
    if x > 0 && maze[x - 1][y] != '#' {
        neighbors.push((x - 1, y));
    }
    if y < maze[0].len() - 1 && maze[x][y + 1] != '#' {
        neighbors.push((x, y + 1));
    }
    if x < maze.len() - 1 && maze[x + 1][y] != '#' {
        neighbors.push((x + 1, y));
    }
    if y > 0 && maze[x][y - 1] != '#' {
        neighbors.push((x, y - 1));
    }
    neighbors
}

fn get_height(c: char) -> i32 {
    match c {
        'S' | 'E' => 0,
        c => c as i32 - '0' as i32,
    }
}

fn get_dist(c1: char, c2: char) -> u32 {
    let h1 = get_height(c1);
    let h2 = get_height(c2);

    let dh = h1.abs_diff(h2);
    dh.min(10 - dh) + 1
}

fn get_shortest_path(maze: &[Vec<char>], start_char: char, end_char: char) -> u32 {
    let start = get_pos(maze, start_char);

    let mut seen: Vec<Vec<bool>> = vec![vec![false; maze[0].len()]; maze.len()];
    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; maze[0].len()]; maze.len()];
    dist[start.0][start.1] = 0;

    let mut queue: Vec<Pos> = vec![start];
    while !queue.is_empty() {
        let (x, y) = queue.swap_remove(
            (0..queue.len())
                .min_by_key(|&i| dist[queue[i].0][queue[i].1])
                .unwrap(),
        );

        if maze[x][y] == end_char {
            return dist[x][y];
        }

        if seen[x][y] {
            continue;
        }
        seen[x][y] = true;

        for (nx, ny) in get_neighbors(maze, (x, y)) {
            if seen[nx][ny] {
                continue;
            }

            let ndist = dist[x][y] + get_dist(maze[x][y], maze[nx][ny]);
            if ndist < dist[nx][ny] {
                dist[nx][ny] = ndist;
                queue.push((nx, ny));
            }
        }
    }

    unreachable!();
}
