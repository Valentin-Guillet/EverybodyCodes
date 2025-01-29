use crate::args::RunArgs;

use std::{collections::VecDeque, fs::read_to_string};

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let mut map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    if args.part == 3 {
        map[75][168] = 'L';
    }
    let map: Vec<&[char]> = map.iter().map(|line| line.as_slice()).collect();

    match args.part {
        1 => find_single_herb(&map, 'H'),
        2 => solve_maze(&map),
        3 => solve_large_maze(&map),
        _ => unreachable!(),
    }
}

#[derive(Clone)]
struct Node {
    pos: (usize, usize),
    herbs: usize,
    dist: u32,
}

fn add_herb(herbs: usize, herb: char) -> usize {
    match herb {
        'A' | 'G' | 'N' => herbs | 0b000001,
        'B' | 'H' | 'O' => herbs | 0b000010,
        'C' | 'I' | 'P' => herbs | 0b000100,
        'D' | 'J' | 'Q' => herbs | 0b001000,
        'E' | 'K' | 'R' => herbs | 0b010000,
        'L' => herbs | 0b100000,
        _ => herbs,
    }
}

fn get_neighbors(map: &[&[char]], (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 && map[x - 1][y] != '#' && map[x - 1][y] != '~' {
        neighbors.push((x - 1, y));
    }
    if y < map[0].len() - 1 && map[x][y + 1] != '#' && map[x][y + 1] != '~' {
        neighbors.push((x, y + 1));
    }
    if x < map.len() - 1 && map[x + 1][y] != '#' && map[x + 1][y] != '~' {
        neighbors.push((x + 1, y));
    }
    if y > 0 && map[x][y - 1] != '#' && map[x][y - 1] != '~' {
        neighbors.push((x, y - 1));
    }
    neighbors
}

fn find_single_herb(map: &[&[char]], herb: char) -> u32 {
    let start = (0, map[0].iter().position(|&c| c == '.').unwrap());

    let mut seen = vec![vec![false; map[0].len()]; map.len()];
    seen[start.0][start.1] = true;

    let mut queue: VecDeque<((usize, usize), u32)> = VecDeque::from([(start, 0)]);
    while let Some((pos, dist)) = queue.pop_front() {
        if map[pos.0][pos.1] == herb {
            return 2 * dist;
        }

        for (nx, ny) in get_neighbors(map, pos) {
            if seen[nx][ny] {
                continue;
            }
            seen[nx][ny] = true;
            queue.push_back(((nx, ny), dist + 1));
        }
    }
    unreachable!()
}

fn solve_maze(map: &[&[char]]) -> u32 {
    find_all_herbs(
        map,
        (0, map[0].iter().position(|&c| c == '.').unwrap()),
        0b11111,
    )
}

fn find_all_herbs(map: &[&[char]], start_pos: (usize, usize), target_herbs: usize) -> u32 {
    let start = Node {
        pos: start_pos,
        herbs: 0,
        dist: 0,
    };

    let mut seen = vec![vec![vec![false; 64]; map[0].len()]; map.len()];
    seen[start.pos.0][start.pos.1][start.herbs] = true;

    let mut queue: VecDeque<Node> = VecDeque::from([start.clone()]);
    while let Some(node) = queue.pop_front() {
        if node.pos == start.pos && node.herbs == target_herbs {
            return node.dist;
        }

        for (nx, ny) in get_neighbors(map, node.pos) {
            if seen[nx][ny][node.herbs] {
                continue;
            }
            seen[nx][ny][node.herbs] = true;

            let neighbor = Node {
                pos: (nx, ny),
                herbs: add_herb(node.herbs, map[nx][ny]),
                dist: node.dist + 1,
            };
            seen[nx][ny][neighbor.herbs] = true;
            queue.push_back(neighbor);
        }
    }
    unreachable!()
}

fn solve_large_maze(map: &[&[char]]) -> u32 {
    let n = map[0].len() / 3;

    let first_map: Vec<&[char]> = map.iter().map(|&line| &line[..n]).collect();
    let second_map: Vec<&[char]> = map.iter().map(|&line| &line[n..2 * n]).collect();
    let third_map: Vec<&[char]> = map.iter().map(|&line| &line[2 * n..]).collect();
    let mut total_dist = 0;
    total_dist += find_all_herbs(
        &second_map,
        (0, second_map[0].iter().position(|&c| c == '.').unwrap()),
        0b111111,
    );
    total_dist += find_all_herbs(&first_map, (map.len() - 2, n - 1), 0b11111) + 4;
    total_dist += find_all_herbs(&third_map, (map.len() - 2, 0), 0b11111) + 4;
    total_dist
}
