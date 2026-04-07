use crate::args::RunArgs;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    row: isize,
    col: isize,
}

impl Pos {
    fn step(&mut self, direction: u16, length: usize) {
        match direction {
            0 => self.row -= length as isize,
            1 => self.col += length as isize,
            2 => self.row += length as isize,
            3 => self.col -= length as isize,
            _ => unreachable!(),
        }
    }
}

type Grid = HashSet<Pos>;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let instructions: Vec<_> = data
        .split(",")
        .map(|s| {
            let length = s[1..].trim_end().parse::<u64>().unwrap();
            (s.chars().next().unwrap(), length)
        })
        .collect();

    let (grid, grid_size, rows, cols, start, end) = build_compressed_grid(&instructions);
    get_shortest_path(&grid, &grid_size, &rows, &cols, &start, &end)
}

fn compress_coordinates(coords: &mut Vec<isize>) -> HashMap<isize, isize> {
    coords.sort();
    coords.dedup();
    coords
        .iter()
        .enumerate()
        .map(|(i, &coord)| (coord, i as isize))
        .collect()
}

fn build_compressed_grid(
    instructions: &[(char, u64)],
) -> (Grid, (usize, usize), Vec<isize>, Vec<isize>, Pos, Pos) {
    let mut pos = Pos { row: 0, col: 0 };
    let mut dir = 0u16;

    let mut rows = vec![pos.row];
    let mut cols = vec![pos.col];
    let mut walls = vec![pos.clone()];
    for &(turn, length) in instructions {
        dir = (dir + if turn == 'L' { 3 } else { 1 }) % 4;
        pos.step(dir, length as usize);
        walls.push(pos.clone());
        rows.extend([pos.row - 1, pos.row, pos.row + 1]);
        cols.extend([pos.col - 1, pos.col, pos.col + 1]);
    }

    let compressed_rows = compress_coordinates(&mut rows);
    let compressed_cols = compress_coordinates(&mut cols);

    let mut grid = Grid::new();
    for i in 0..walls.len() - 1 {
        let start_row = compressed_rows[&walls[i].row];
        let end_row = compressed_rows[&walls[i + 1].row];
        let start_col = compressed_cols[&walls[i].col];
        let end_col = compressed_cols[&walls[i + 1].col];
        for row in start_row.min(end_row)..=start_row.max(end_row) {
            for col in start_col.min(end_col)..=start_col.max(end_col) {
                grid.insert(Pos { row, col });
            }
        }
    }

    let compressed_start = Pos {
        row: compressed_rows[&0],
        col: compressed_cols[&0],
    };
    let compressed_end = Pos {
        row: compressed_rows[&pos.row],
        col: compressed_cols[&pos.col],
    };
    let grid_size = (compressed_rows.len(), compressed_cols.len());
    (
        grid,
        grid_size,
        rows,
        cols,
        compressed_start,
        compressed_end,
    )
}

fn get_neighbors(grid: &Grid, &(height, width): &(usize, usize), end: &Pos, pos: &Pos) -> Vec<Pos> {
    let mut neighbors = Vec::new();

    for dir in 0..4 {
        let mut neigh = pos.clone();
        neigh.step(dir, 1);
        if neigh.row < 0
            || neigh.row >= height as isize
            || neigh.col < 0
            || neigh.col >= width as isize
            || (&neigh != end && grid.contains(&neigh))
        {
            continue;
        }
        neighbors.push(neigh);
    }
    neighbors
}

fn get_shortest_path(
    grid: &Grid,
    grid_size: &(usize, usize),
    rows: &[isize],
    cols: &[isize],
    start: &Pos,
    end: &Pos,
) -> u32 {
    let mut heap: BinaryHeap<(Reverse<u32>, Pos)> = BinaryHeap::new();
    heap.push((Reverse(0), start.clone()));

    let mut distances: HashMap<Pos, u32> = HashMap::new();
    distances.insert(start.clone(), 0);

    let mut seen: HashSet<Pos> = HashSet::new();
    while !heap.is_empty() {
        let (dist, pos) = heap.pop().unwrap();
        if seen.contains(&pos) {
            continue;
        }
        seen.insert(pos.clone());

        if &pos == end {
            return dist.0;
        }

        for neigh in get_neighbors(grid, grid_size, end, &pos) {
            let next_dist = dist.0
                + (rows[neigh.row as usize] - rows[pos.row as usize]).abs() as u32
                + (cols[neigh.col as usize] - cols[pos.col as usize]).abs() as u32;
            if distances.get(&neigh).is_some_and(|n| *n <= next_dist) {
                continue;
            }
            distances.insert(neigh.clone(), next_dist);
            heap.push((Reverse(next_dist), neigh));
        }
    }
    unreachable!()
}
