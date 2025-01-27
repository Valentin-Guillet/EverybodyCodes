use crate::args::RunArgs;

use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

type Pos = (i32, i32, i32);

struct Segment {
    direction: char,
    length: u32,
}

impl Segment {
    fn new(input: &str) -> Self {
        Self {
            direction: input.chars().next().unwrap(),
            length: input[1..].trim().parse().unwrap(),
        }
    }
}

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let branches: Vec<Vec<Segment>> = data
        .lines()
        .map(|line| line.split(',').map(Segment::new).collect())
        .collect();

    match args.part {
        1 => get_height(&branches[0]),
        2 => build_tree(&branches).0.len() as u32,
        3 => get_murkiness_level(&branches),
        _ => unreachable!(),
    }
}

fn get_height(segments: &[Segment]) -> u32 {
    let mut max_height = 0;
    let mut height = 0;
    for segment in segments {
        if segment.direction == 'U' {
            height += segment.length;
            max_height = max_height.max(height);
        } else if segment.direction == 'D' {
            height -= segment.length;
        }
    }
    max_height
}

fn step((x, y, z): (i32, i32, i32), dir: char) -> (i32, i32, i32) {
    match dir {
        'U' => (x + 1, y, z),
        'D' => (x - 1, y, z),
        'R' => (x, y + 1, z),
        'L' => (x, y - 1, z),
        'F' => (x, y, z + 1),
        'B' => (x, y, z - 1),
        _ => unreachable!(),
    }
}

fn build_tree(branches: &[Vec<Segment>]) -> (HashSet<Pos>, Vec<Pos>) {
    let mut seg_pos: HashSet<Pos> = HashSet::new();
    let mut leave_pos: Vec<Pos> = Vec::new();
    for branch in branches {
        let mut pos = (0, 0, 0);
        for segment in branch {
            for _ in 0..segment.length {
                pos = step(pos, segment.direction);
                seg_pos.insert(pos);
            }
        }
        leave_pos.push(pos);
    }

    (seg_pos, leave_pos)
}

fn get_leaf_distances(segments: &HashSet<Pos>, leaves: &[Pos], height: u32) -> u32 {
    let mut total_dist = 0;
    let mut nb_leaves_found = 0;

    let start = segments.get(&(height as i32, 0, 0)).unwrap();
    let mut added: HashSet<&Pos> = HashSet::from([start]);
    let mut queue: VecDeque<(&Pos, u32)> = VecDeque::from([(start, 0)]);
    while let Some((pos, dist)) = queue.pop_front() {
        if leaves.contains(pos) {
            total_dist += dist;
            nb_leaves_found += 1;
            if nb_leaves_found == leaves.len() {
                return total_dist;
            }
        }

        for neighbor in [
            (pos.0 + 1, pos.1, pos.2),
            (pos.0 - 1, pos.1, pos.2),
            (pos.0, pos.1 + 1, pos.2),
            (pos.0, pos.1 - 1, pos.2),
            (pos.0, pos.1, pos.2 + 1),
            (pos.0, pos.1, pos.2 - 1),
        ] {
            let neighbor = match segments.get(&neighbor) {
                Some(neigh) => neigh,
                None => continue,
            };

            if !added.contains(neighbor) {
                queue.push_back((neighbor, dist + 1));
                added.insert(neighbor);
            }
        }
    }
    unreachable!();
}

fn get_murkiness_level(branches: &[Vec<Segment>]) -> u32 {
    let (segments, leaves) = build_tree(branches);
    let max_height = segments
        .iter()
        .filter(|pos| pos.1 == 0 && pos.2 == 0)
        .map(|pos| pos.0 as u32)
        .max()
        .unwrap();
    (0..max_height)
        .filter(|&height| segments.contains(&(height as i32, 0, 0)))
        .map(|height| get_leaf_distances(&segments, &leaves, height))
        .min()
        .unwrap()
}
