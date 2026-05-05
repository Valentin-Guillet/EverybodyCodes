use crate::args::RunArgs;

use std::fs::read_to_string;

struct Die {
    faces: Vec<i32>,
    seed: usize,
    face_id: usize,
    pulse: usize,
    roll_nb: usize,
}

impl Die {
    fn from(input: &str) -> Self {
        let words: Vec<&str> = input.split(' ').skip(1).collect();
        let faces = words[0][7..words[0].len() - 1]
            .split(',')
            .map(|v| v.parse::<i32>().unwrap())
            .collect();
        let seed = words[1][5..].parse().unwrap();
        Self {
            faces,
            seed,
            face_id: 0,
            pulse: seed,
            roll_nb: 0,
        }
    }

    fn roll(&mut self) -> i32 {
        self.roll_nb += 1;
        let spin = self.roll_nb * self.pulse;
        self.face_id = (self.face_id + spin) % self.faces.len();
        self.pulse = (self.pulse + spin) % self.seed + 1 + self.roll_nb + self.seed;
        self.faces[self.face_id]
    }
}

pub fn run(args: &RunArgs) -> String {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let mut input_it = data.trim_end().lines();
    let dice: Vec<Die> = input_it
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(Die::from)
        .collect();

    match args.part {
        1 => reach_above(dice, 10000).to_string(),
        2 => {
            let racetrack = input_it
                .next()
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>();
            get_race_order(dice, &racetrack)
        }
        3 => {
            let grid: Vec<Vec<u8>> = input_it
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect()
                })
                .collect::<Vec<_>>();
            get_max_prize_pool(dice, &grid)
        }
        _ => unreachable!(),
    }
}

fn reach_above(mut dice: Vec<Die>, target: u32) -> u32 {
    let mut total = 0;
    let mut nb_rolls = 0;
    while total < target as i32 {
        let values = dice.iter_mut().map(|die| die.roll()).collect::<Vec<_>>();
        total += values.iter().sum::<i32>();
        nb_rolls += 1;
    }
    nb_rolls
}

fn get_race_order(mut dice: Vec<Die>, racetrack: &[u8]) -> String {
    let nb_dice = dice.len();
    let racetrack_length = racetrack.len();

    let mut order: Vec<usize> = Vec::new();
    let mut progress: Vec<usize> = vec![0; nb_dice];
    while order.len() < nb_dice {
        for (i, die) in dice.iter_mut().enumerate() {
            let position = &mut progress[i];
            if *position == racetrack_length {
                continue;
            }
            if die.roll() == racetrack[*position] as i32 {
                *position += 1;
                if *position == racetrack_length {
                    order.push(i);
                }
            }
        }
    }

    let mut order_str = order
        .iter()
        .map(|o| (o + 1).to_string() + ",")
        .collect::<String>();
    order_str.pop();
    order_str
}

fn get_init_coords(grid: &[Vec<u8>], value: u8) -> Vec<(usize, usize)> {
    let mut paths = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == value {
                paths.push((row, col));
            }
        }
    }
    paths
}

fn get_neighbors(grid: &[Vec<u8>], row: usize, col: usize, value: u8) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if grid[row][col] == value {
        neighbors.push((row, col));
    }
    if row > 0 && grid[row - 1][col] == value {
        neighbors.push((row - 1, col));
    }
    if row < grid.len() - 1 && grid[row + 1][col] == value {
        neighbors.push((row + 1, col));
    }
    if col > 0 && grid[row][col - 1] == value {
        neighbors.push((row, col - 1));
    }
    if col < grid[0].len() - 1 && grid[row][col + 1] == value {
        neighbors.push((row, col + 1));
    }

    neighbors
}

fn get_max_prize_pool(mut dice: Vec<Die>, grid: &[Vec<u8>]) -> String {
    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
    for die in &mut dice {
        let mut value = die.roll() as u8;
        let mut coords = get_init_coords(grid, value);
        while !coords.is_empty() {
            let mut next_coords = Vec::new();

            value = die.roll() as u8;
            for &(row, col) in &coords {
                seen[row][col] = true;
                next_coords.extend(get_neighbors(grid, row, col, value));
            }
            next_coords.sort_unstable();
            next_coords.dedup();
            coords = next_coords;
        }
    }
    seen.iter().flatten().filter(|&&v| v).count().to_string()
}
