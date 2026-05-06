use crate::args::RunArgs;

use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    UP,
    RIGHT,
    DOWN,
    LEFT,
    NONE,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::UP => Dir::DOWN,
            Dir::RIGHT => Dir::LEFT,
            Dir::DOWN => Dir::UP,
            Dir::LEFT => Dir::RIGHT,
            Dir::NONE => Dir::NONE,
        }
    }
}

#[derive(Clone, Copy)]
enum Node {
    Empty(Dir),
    Seen,
    Bone,
}

struct Grid {
    prows: Vec<(Vec<Node>, Vec<Node>)>,
    nrows: Vec<(Vec<Node>, Vec<Node>)>,
}

impl Grid {
    fn new(nb_prows: usize, nb_nrows: usize, nb_pcols: usize, nb_ncols: usize) -> Self {
        let prows = vec![
            (
                vec!(Node::Empty(Dir::NONE); nb_pcols),
                vec!(Node::Empty(Dir::NONE); nb_ncols),
            );
            nb_prows
        ];
        let nrows = vec![
            (
                vec!(Node::Empty(Dir::NONE); nb_pcols),
                vec!(Node::Empty(Dir::NONE); nb_ncols),
            );
            nb_nrows
        ];

        Self { prows, nrows }
    }

    fn len(&self) -> (isize, isize, isize, isize) {
        let (nb_prows, nb_nrows) = (self.prows.len() as isize, -(self.nrows.len() as isize));
        let (nb_pcols, nb_ncols) = self.prows.get(0).map_or((0, 0), |row| {
            (row.0.len() as isize, -(row.1.len() as isize))
        });
        (nb_prows, nb_nrows, nb_pcols, nb_ncols)
    }

    fn resize(&mut self, new_row: isize, new_col: isize) {
        let (nb_prows, nb_nrows, nb_pcols, nb_ncols) = self.len();
        // Add new row below
        if new_row >= nb_prows {
            for _ in 0..=(new_row + 1 - nb_prows) {
                self.prows.push((
                    vec![Node::Empty(Dir::DOWN); nb_pcols as usize],
                    vec![Node::Empty(Dir::DOWN); nb_ncols.abs() as usize],
                ));
            }

        // Add new row above
        } else if new_row < nb_nrows {
            for _ in 0..(new_row.abs_diff(nb_nrows)) {
                self.nrows.push((
                    vec![Node::Empty(Dir::UP); nb_pcols as usize],
                    vec![Node::Empty(Dir::UP); nb_ncols.abs() as usize],
                ));
            }
        }

        // Add new column to the right
        if new_col >= nb_pcols {
            for row in self.prows.iter_mut().chain(self.nrows.iter_mut()) {
                row.0
                    .extend([Node::Empty(Dir::RIGHT)].repeat(new_col.abs_diff(nb_pcols) + 1));
            }

        // Add new column to the left
        } else if new_col < nb_ncols {
            for row in self.prows.iter_mut().chain(self.nrows.iter_mut()) {
                row.1
                    .extend([Node::Empty(Dir::LEFT)].repeat(new_col.abs_diff(nb_ncols)));
            }
        }
    }

    fn rows(&self) -> std::ops::Range<isize> {
        let (nb_prows, nb_nrows) = (self.prows.len() as isize, -(self.nrows.len() as isize));
        nb_nrows..nb_prows
    }

    fn cols(&self) -> std::ops::Range<isize> {
        let (nb_pcols, nb_ncols) = self.prows.get(0).map_or((0, 0), |row| {
            (row.0.len() as isize, -(row.1.len() as isize))
        });
        nb_ncols..nb_pcols
    }
}

impl Index<(isize, isize)> for Grid {
    type Output = Node;

    fn index(&self, (row, col): (isize, isize)) -> &Self::Output {
        let rows = if row >= 0 {
            &self.prows[row as usize]
        } else {
            &self.nrows[row.abs() as usize - 1]
        };
        if col >= 0 {
            &rows.0[col as usize]
        } else {
            &rows.1[col.abs() as usize - 1]
        }
    }
}

impl IndexMut<(isize, isize)> for Grid {
    fn index_mut(&mut self, (row, col): (isize, isize)) -> &mut Self::Output {
        self.resize(row, col);
        let rows = if row >= 0 {
            &mut self.prows[row as usize]
        } else {
            &mut self.nrows[row.abs() as usize - 1]
        };
        if col >= 0 {
            &mut rows.0[col as usize]
        } else {
            &mut rows.1[col.abs() as usize - 1]
        }
    }
}

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let mut grid = parse_grid(data.trim_end());

    match args.part {
        1 => count_steps_to_bone(&mut grid),
        2 => counts_steps_to_surround(&mut grid, 1),
        3 => counts_steps_to_surround(&mut grid, 3),
        _ => unreachable!(),
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut start = (0, 0);
    let mut bones = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (index, c) in line.chars().enumerate() {
            if c == '@' {
                start = (row, index);
            } else if c == '#' {
                bones.push((row, index));
            }
        }
    }

    // Need to add one line of row all around to contain all targets
    let nb_prows = input.lines().count() - start.0 + 1;
    let nb_nrows = start.0 + 1;
    let nb_pcols = input.lines().next().unwrap().len() - start.1 + 1;
    let nb_ncols = start.1 + 1;

    let mut grid = Grid::new(nb_prows, nb_nrows, nb_pcols, nb_ncols);
    for (row, col) in bones {
        grid[(
            row as isize - start.0 as isize,
            col as isize - start.1 as isize,
        )] = Node::Bone;
    }
    grid
}

fn count_steps_to_bone(grid: &mut Grid) -> u32 {
    let mut step_it = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().cycle();

    let (mut row, mut col) = (0, 0);
    let mut nb_steps = 0;
    loop {
        let step = step_it.next().unwrap();
        grid.resize(row + step.0, col + step.1);
        match grid[(row + step.0, col + step.1)] {
            Node::Bone => return nb_steps + 1,
            Node::Seen => continue,
            Node::Empty(..) => {
                grid[(row, col)] = Node::Seen;
                (row, col) = (row + step.0, col + step.1);
            }
        }
        nb_steps += 1;
    }
}

fn get_neighbors(
    grid: &Grid,
    (row, col): (isize, isize),
) -> impl Iterator<Item = ((isize, isize), Dir)> {
    [
        ((row - 1, col), Dir::DOWN),
        ((row + 1, col), Dir::UP),
        ((row, col - 1), Dir::RIGHT),
        ((row, col + 1), Dir::LEFT),
    ]
    .into_iter()
    .filter(|((r, c), _)| grid.rows().contains(r) && grid.cols().contains(c))
}

fn init_paths(grid: &mut Grid) {
    let mut queue = VecDeque::new();
    let (bot_row, top_row, right_col, left_col) = grid.len();

    for row in grid.rows() {
        for (col, dir) in [(left_col, Dir::LEFT), (right_col - 1, Dir::RIGHT)] {
            if let Node::Empty(cell_dir) = &mut grid[(row, col)] {
                *cell_dir = dir;
                queue.push_back((row, col));
            }
        }
    }
    for col in grid.cols() {
        for (row, dir) in [(top_row, Dir::UP), (bot_row - 1, Dir::DOWN)] {
            if let Node::Empty(cell_dir) = &mut grid[(row, col)] {
                *cell_dir = dir;
                queue.push_back((row, col));
            }
        }
    }

    while let Some((row, col)) = queue.pop_front() {
        for (neighbor, dir) in get_neighbors(grid, (row, col)).collect::<Vec<_>>() {
            let Node::Empty(ndir @ Dir::NONE) = &mut grid[neighbor] else {
                continue;
            };
            *ndir = dir;
            queue.push_back(neighbor);
        }
    }
}

fn get_parent_nodes(grid: &mut Grid, row: isize, col: isize) -> Vec<(isize, isize)> {
    let mut parents = Vec::new();
    for (neighbor, dir) in get_neighbors(grid, (row, col)) {
        if let Node::Empty(ndir) = grid[neighbor]
            && ndir == dir
        {
            parents.push(neighbor);
        }
    }
    parents
}

fn mark_node_as_seen(
    grid: &mut Grid,
    seen_row: isize,
    seen_col: isize,
    targets: &mut HashSet<(isize, isize)>,
) {
    grid[(seen_row, seen_col)] = Node::Seen;
    targets.remove(&(seen_row, seen_col));

    for parent in get_parent_nodes(grid, seen_row, seen_col) {
        // Parent has been seen when processing an earlier one
        if let Node::Seen = grid[parent] {
            continue;
        }

        let mut children: HashSet<(isize, isize)> = HashSet::new();
        let mut border: HashSet<(isize, isize)> = HashSet::new();
        let mut queue = vec![parent];
        while let Some((row, col)) = queue.pop() {
            children.insert((row, col));
            for (neighbor, dir) in get_neighbors(grid, (row, col)) {
                if children.contains(&neighbor) {
                    continue;
                }
                let Node::Empty(ndir) = grid[neighbor] else {
                    continue;
                };
                if ndir != dir {
                    border.insert(neighbor);
                } else {
                    queue.push(neighbor);
                }
            }
        }

        let Some(&exit_pos) = border.difference(&children).next() else {
            // No exit found, we mark all nodes as SEEN
            for pos in children {
                grid[pos] = Node::Seen;
                targets.remove(&pos);
            }
            continue;
        };

        // Found new exit! Create new path
        let (last_node, last_dir) = get_neighbors(grid, exit_pos)
            .filter(|(pos, _)| children.contains(pos))
            .next()
            .unwrap();

        let (mut path_row, mut path_col) = last_node;
        let mut dir = last_dir.opposite();
        loop {
            let Node::Empty(cell_dir) = &mut grid[(path_row, path_col)] else {
                unreachable!()
            };
            (*cell_dir, dir) = (dir.opposite(), *cell_dir);

            if (path_row, path_col) == parent {
                break;
            }
            (path_row, path_col) = match dir {
                Dir::UP => (path_row - 1, path_col),
                Dir::RIGHT => (path_row, path_col + 1),
                Dir::DOWN => (path_row + 1, path_col),
                Dir::LEFT => (path_row, path_col - 1),
                Dir::NONE => unreachable!(),
            };
        }
    }
}

fn get_all_targets(grid: &Grid) -> HashSet<(isize, isize)> {
    let mut targets = HashSet::new();
    for row in grid.rows() {
        for col in grid.cols() {
            let Node::Bone = grid[(row, col)] else {
                continue;
            };
            for (neighbor, _) in get_neighbors(grid, (row, col)) {
                if let Node::Empty(dir) = grid[neighbor]
                    && dir != Dir::NONE
                {
                    targets.insert(neighbor);
                }
            }
        }
    }
    targets
}

fn counts_steps_to_surround(grid: &mut Grid, repeat_steps: usize) -> u32 {
    let mut step_it = [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .flat_map(|pos| std::iter::repeat_n(pos, repeat_steps))
        .cycle();

    init_paths(grid);
    let mut targets = get_all_targets(grid);

    let (mut row, mut col) = (0, 0);
    mark_node_as_seen(grid, row, col, &mut targets);

    let mut nb_steps = 0;
    while !targets.is_empty() {
        let step = step_it.next().unwrap();
        let (new_row, new_col) = (row + step.0, col + step.1);
        grid.resize(new_row, new_col);
        let Node::Empty(..) = grid[(new_row, new_col)] else {
            continue;
        };

        (row, col) = (new_row, new_col);
        mark_node_as_seen(grid, row, col, &mut targets);

        nb_steps += 1;
    }
    nb_steps
}
