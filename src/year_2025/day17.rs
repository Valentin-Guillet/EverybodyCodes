use crate::args::RunArgs;

use std::fs::read_to_string;

struct State {
    row: usize,
    col: usize,
    min_radius: u8,
}

type Grid<T> = Vec<Vec<T>>;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let grid: Grid<u8> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' | 'S' => 0,
                    c => c.to_digit(10).unwrap() as u8,
                })
                .collect()
        })
        .collect();

    match args.part {
        1 => count_destroyed_cells(&grid, 10),
        2 => count_largest_step(&grid),
        3 => make_plant_loop(&grid),
        _ => unreachable!(),
    }
}

fn count_destroyed_cells(grid: &Grid<u8>, radius: u32) -> u32 {
    let vol_row = grid.len() / 2;
    let vol_col = grid[0].len() / 2;
    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if row == vol_row && col == vol_col {
                continue;
            }
            let dist = row.abs_diff(vol_row).pow(2) + col.abs_diff(vol_col).pow(2);
            if dist as u32 <= radius * radius {
                count += grid[row][col] as u32;
            }
        }
    }
    count
}

fn get_radius(grid: &Grid<u8>, row: usize, col: usize) -> u8 {
    let vol_row = grid.len() / 2;
    let vol_col = grid[0].len() / 2;
    let dist = row.abs_diff(vol_row).pow(2) + col.abs_diff(vol_col).pow(2);
    (dist as f64).sqrt().ceil() as u8
}

fn count_largest_step(grid: &Grid<u8>) -> u32 {
    let vol_row = grid.len() / 2;
    let vol_col = grid[0].len() / 2;
    let max_radius = vol_row + vol_col;
    let mut counts = vec![0; max_radius];
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if row == vol_row && col == vol_col {
                continue;
            }

            let radius = get_radius(grid, row, col) as usize;
            counts[radius] += grid[row][col] as u32;
        }
    }

    let max_step = (0..max_radius).max_by_key(|&i| counts[i]).unwrap();
    max_step as u32 * counts[max_step]
}

fn find_start(grid: &Grid<u8>) -> State {
    let vol_row = grid.len() / 2;
    let vol_col = grid[0].len() / 2;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if (row == vol_row && col == vol_col) || grid[row][col] != 0 {
                continue;
            }
            let start_radius = get_radius(grid, row, col);
            return State {
                row,
                col,
                min_radius: start_radius,
            };
        }
    }
    unreachable!()
}

fn compute_radii_grid(grid: &Grid<u8>) -> Grid<u8> {
    (0..grid.len())
        .map(|row| {
            (0..grid[0].len())
                .map(|col| get_radius(grid, row, col))
                .collect()
        })
        .collect()
}

fn get_neighbors(
    grid: &Grid<u8>,
    radius_grid: &Grid<u8>,
    state: &State,
    max_radius: u8,
) -> Vec<State> {
    let mut neighs = Vec::new();
    for (drow, dcol) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let row = state.row as isize + drow;
        let col = state.col as isize + dcol;
        if row < 0 || row >= grid.len() as isize || col < 0 || col >= grid[0].len() as isize {
            continue;
        }
        let row = row as usize;
        let col = col as usize;

        // Skip volcano
        if grid[row][col] == 0 {
            continue;
        }

        let min_radius = state.min_radius.min(radius_grid[row][col]).min(max_radius);
        neighs.push(State {
            row,
            col,
            min_radius,
        });
    }
    neighs
}

fn make_plant_loop(grid: &Grid<u8>) -> u32 {
    let vol_row = grid.len() / 2;
    let vol_col = grid[0].len() / 2;

    let start = find_start(grid);
    let max_radius = start.min_radius as usize;

    // radius_grid stores the radius between each cell and the volcano
    let radius_grid = compute_radii_grid(&grid);

    // distance_grid is a 3D table (x, y, l) that stores the minimum distance to a cell in (x, y)
    // following a path that gets to at least lava radius l. If None, no such path exists.
    let mut distance_grid: Grid<Vec<Option<u32>>> =
        vec![vec![vec![None; max_radius + 1 as usize]; grid[0].len()]; grid.len()];
    distance_grid[start.row][start.col] = vec![Some(0); max_radius + 1];

    // We should use a priority queue to sort the states by distance, but instead we compute an
    // upper bound on the distance and use a bucket queue
    let max_cell_dist = 9 * (grid.len() + grid[0].len());
    let mut queue: Vec<Vec<State>> = vec![];
    for _ in 0..max_cell_dist {
        queue.push(Vec::new());
    }
    let mut min_index = 0;
    queue[0].push(start);

    while min_index < max_cell_dist {
        let state = queue[min_index].pop().unwrap();
        let state_dist = distance_grid[state.row][state.col][state.min_radius as usize].unwrap();

        for neigh in get_neighbors(grid, &radius_grid, &state, max_radius as u8) {
            // Stop at band under the volcano
            if neigh.col == vol_col && neigh.row > vol_row {
                continue;
            }

            let neigh_dist = state_dist + grid[neigh.row][neigh.col] as u32;

            // Invalid neighbor as we already know that reaching it uses a path that gets closer to
            // the volcano than what will flow in the time we arrive here
            if neigh.min_radius as u32 <= neigh_dist / 30 {
                continue;
            }

            let neigh_distances = &mut distance_grid[neigh.row][neigh.col];
            // Path is longer, skip
            if neigh_distances[neigh.min_radius as usize].is_some_and(|dist| dist <= neigh_dist) {
                continue;
            }

            for prev_neigh_dist in &mut neigh_distances[..=neigh.min_radius as usize] {
                if prev_neigh_dist.is_none_or(|dist| dist > neigh_dist) {
                    *prev_neigh_dist = Some(neigh_dist);
                }
            }
            queue[neigh_dist as usize].push(neigh);
            min_index = min_index.min(neigh_dist as usize);
        }

        while min_index < max_cell_dist && queue[min_index].is_empty() {
            min_index += 1;
        }
    }

    let mut min_path_dist = u32::MAX;
    let mut final_volcano_radius = 0;
    for row in vol_row + 1..grid.len() {
        // Can't look at paths whose min_radius is greater than distance to volcano
        // since they won't be able to get close enough to the volcano
        let radius_limit = (row - vol_row).min(max_radius);

        for path_radius in 0..=radius_limit {
            let Some(left_dist) = distance_grid[row][vol_col - 1][path_radius as usize] else {
                continue;
            };
            let Some(right_dist) = distance_grid[row][vol_col + 1][path_radius as usize] else {
                continue;
            };

            let total_path_dist = left_dist + right_dist + grid[row][vol_col] as u32;
            let path_lava_radius = total_path_dist / 30;
            if path_radius as u32 <= path_lava_radius {
                continue;
            }
            if total_path_dist < min_path_dist {
                min_path_dist = total_path_dist;
                final_volcano_radius = path_radius as u32 - 1;
            }
        }
    }
    min_path_dist * final_volcano_radius
}
