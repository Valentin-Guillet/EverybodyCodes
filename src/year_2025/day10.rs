use crate::args::RunArgs;

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    hash::{DefaultHasher, Hash, Hasher},
};

type Grid = Vec<Vec<char>>;
type Pos = (usize, usize);

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let grid = read_grid(&data);

    match args.part {
        1 => eat_static_sheep(&grid, 4),
        2 => eat_moving_sheep(&grid, 20),
        3 => count_eating_sequences(&grid),
        _ => unreachable!(),
    }
}

fn read_grid(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_dragon(grid: &Grid) -> Pos {
    for (i, line) in grid.iter().enumerate() {
        for (j, &chr) in line.iter().enumerate() {
            if chr == 'D' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn get_neighbors(grid: &Grid, &(x, y): &Pos) -> Vec<Pos> {
    let mut neighbors = Vec::new();
    for (dx, dy) in [
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
    ] {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;
        if 0 <= new_x && new_x < grid.len() as i32 && 0 <= new_y && new_y < grid[0].len() as i32 {
            neighbors.push((new_x as usize, new_y as usize));
        }
    }
    neighbors
}

fn eat_static_sheep(grid: &Grid, steps: u64) -> u64 {
    let mut positions: HashSet<Pos> = HashSet::from([find_dragon(grid)]);
    for _ in 0..steps {
        let mut new_positions: HashSet<Pos> = positions.clone();
        for pos in &positions {
            for neighbor in get_neighbors(grid, pos) {
                new_positions.insert(neighbor);
            }
        }
        positions = new_positions;
    }
    positions
        .iter()
        .filter(|(x, y)| grid[*x][*y] == 'S')
        .count() as u64
}

fn eat_moving_sheep(grid: &Grid, steps: u64) -> u64 {
    let mut count = 0;
    let mut dragon_positions: HashSet<Pos> = HashSet::from([find_dragon(grid)]);

    let mut sheep: HashSet<Pos> = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                sheep.insert((i, j));
            }
        }
    }

    for _ in 0..steps {
        // Move dragon and eat sheep
        let mut next_positions: HashSet<Pos> = HashSet::new();
        for pos in &dragon_positions {
            for next_pos in get_neighbors(grid, pos) {
                next_positions.insert(next_pos);
                if grid[next_pos.0][next_pos.1] != '#' {
                    sheep.remove(&next_pos).then(|| count += 1);
                }
            }
        }
        dragon_positions = next_positions;

        // Move sheep
        let mut new_sheep = HashSet::new();
        for &pos in &sheep {
            if pos.0 == grid.len() - 1 {
                // Sheep escapes
                continue;
            }

            let next_pos = (pos.0 + 1, pos.1);
            if grid[next_pos.0][next_pos.1] != '#' && dragon_positions.contains(&next_pos) {
                count += 1;
            } else {
                new_sheep.insert(next_pos);
            }
        }
        sheep = new_sheep;
    }
    count
}

#[derive(Clone, Hash)]
struct State {
    dragon_pos: Pos,
    sheep: Vec<usize>,
}

impl State {
    fn from(grid: &Grid) -> Self {
        let mut sheep = vec![grid.len(); grid[0].len()];
        for col in 0..grid[0].len() {
            for row in 0..grid.len() {
                if grid[row][col] == 'S' {
                    sheep[col] = row;
                    break;
                }
            }
        }
        Self {
            dragon_pos: find_dragon(&grid),
            sheep,
        }
    }

    fn get_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

type Cache = HashMap<u64, u64>;

fn _count_eating_sequences(
    grid: &Grid,
    state: &State,
    sheep_limits: &[usize],
    cache: &mut Cache,
) -> u64 {
    let state_hash = state.get_hash();
    let ans = cache.get(&state_hash);
    if ans.is_some() {
        return *ans.unwrap();
    }

    let mut count = 0;
    if state.sheep.iter().all(|&sheep_row| sheep_row == grid.len()) {
        // println!("FINAL STATE: Path {path}");
        cache.insert(state_hash, 1);
        return 1;
    }

    let mut sheep_can_move = false;
    let mut next_sheep_rows: Vec<(usize, usize)> = Vec::new();
    for col in 0..grid[0].len() {
        // Sheep eaten
        if state.sheep[col] == grid.len() {
            continue;
        }
        let sheep_row = state.sheep[col] + 1;

        // Sheep can't move into dragon
        if (sheep_row, col) == state.dragon_pos && grid[sheep_row][col] != '#' {
            continue;
        }

        // Move sheep
        sheep_can_move = true;
        if sheep_row >= sheep_limits[col] {
            // Sheep has reached the safety limits
            continue;
        }

        next_sheep_rows.push((sheep_row, col));
    }

    // Move the dragon again if we can't move any sheep
    if !sheep_can_move {
        next_sheep_rows.push((state.sheep[0], 0));
    }

    let next_positions = get_neighbors(grid, &state.dragon_pos);
    for (sheep_row, col) in next_sheep_rows {
        for &next_pos in &next_positions {
            let mut next_sheep = state.sheep.clone();
            next_sheep[col] = sheep_row;

            // Eat sheep
            if next_sheep[next_pos.1] == next_pos.0 && grid[next_pos.0][next_pos.1] != '#' {
                next_sheep[next_pos.1] = grid.len();
            }

            let next_state = State {
                dragon_pos: next_pos,
                sheep: next_sheep,
            };

            count += _count_eating_sequences(grid, &next_state, sheep_limits, cache);
        }
    }

    cache.insert(state_hash, count);
    count
}

fn count_eating_sequences(grid: &Grid) -> u64 {
    let mut cache = Cache::new();
    let state = State::from(grid);

    let mut sheep_limits = vec![grid.len(); grid[0].len()];
    for col in 0..grid[0].len() {
        for row in (0..grid.len()).rev() {
            if grid[row][col] != '#' {
                sheep_limits[col] = row + 1;
                break;
            }
        }
    }

    _count_eating_sequences(grid, &state, &sheep_limits, &mut cache)
}
