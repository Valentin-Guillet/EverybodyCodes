use crate::args::RunArgs;

use std::{collections::HashSet, fs::read_to_string};

type Barrel = u8;
type Grid<T> = Vec<Vec<T>>;
type Pos = (usize, usize);

#[derive(Debug)]
struct Zone {
    size: u32,
    children: HashSet<usize>,
    ignited: bool,
}

impl Zone {
    fn new() -> Self {
        Self {
            size: 0,
            children: HashSet::new(),
            ignited: false,
        }
    }
}

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let mut barrels: Grid<Barrel> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as Barrel)
                .collect()
        })
        .collect();

    match args.part {
        1 => count_ignited(&barrels, &vec![(0, 0)]),
        2 => count_ignited(
            &barrels,
            &vec![(0, 0), (barrels.len() - 1, barrels[0].len() - 1)],
        ),
        3 => find_best_reaction(&mut barrels),
        _ => unreachable!(),
    }
}

fn get_neighbors<T>(grid: &Grid<T>, (x, y): Pos) -> Vec<Pos> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if y < grid[0].len() - 1 {
        neighbors.push((x, y + 1));
    }
    if x < grid.len() - 1 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    neighbors
}

fn count_ignited(barrels: &Grid<Barrel>, init_positions: &[Pos]) -> u32 {
    let mut nb_ignited = 0;
    let mut seen: Grid<bool> = vec![vec![false; barrels[0].len()]; barrels.len()];
    for pos in init_positions {
        seen[pos.0][pos.1] = true;
    }

    let mut positions = Vec::from(init_positions);
    while !positions.is_empty() {
        let pos = positions.pop().unwrap();
        nb_ignited += 1;

        let height = barrels[pos.0][pos.1];
        for neigh in get_neighbors(barrels, pos) {
            if !seen[neigh.0][neigh.1] && barrels[neigh.0][neigh.1] <= height {
                positions.push(neigh);
                seen[neigh.0][neigh.1] = true;
            }
        }
    }
    nb_ignited
}

fn build_zone(
    barrels: &mut Grid<Barrel>,
    init_pos: Pos,
    zones: &mut Vec<Zone>,
    zone_map: &mut Grid<Option<usize>>,
) -> usize {
    let zone_id = zones.len();
    let zone_height = barrels[init_pos.0][init_pos.1];

    zones.push(Zone::new());

    let mut positions = vec![init_pos];
    while !positions.is_empty() {
        let pos = positions.pop().unwrap();
        if zone_map[pos.0][pos.1].is_some() {
            continue;
        }
        zone_map[pos.0][pos.1] = Some(zone_id);
        zones[zone_id].size += 1;

        for (nx, ny) in get_neighbors(barrels, pos) {
            let neigh = barrels[nx][ny];
            if zone_map[nx][ny].is_none() && neigh == zone_height {
                positions.push((nx, ny));
            } else if neigh < zone_height {
                let neigh_zone_id = zone_map[nx][ny]
                    .unwrap_or_else(|| build_zone(barrels, (nx, ny), zones, zone_map));
                zones[zone_id].children.insert(neigh_zone_id);
            }
        }
    }
    zone_id
}

fn build_zones(barrels: &mut Grid<Barrel>) -> Vec<Zone> {
    let mut zones: Vec<Zone> = Vec::new();
    let mut zone_map: Grid<Option<usize>> = vec![vec![None; barrels[0].len()]; barrels.len()];
    for row in 0..barrels.len() {
        for col in 0..barrels[0].len() {
            if zone_map[row][col].is_some() {
                continue;
            }

            let _ = build_zone(barrels, (row, col), &mut zones, &mut zone_map);
        }
    }
    zones
}

fn get_reaction_zones(zones: &Vec<Zone>, init_zone_id: usize) -> HashSet<usize> {
    let mut reaction_ids: HashSet<usize> = HashSet::new();
    let mut zones_ids = vec![init_zone_id];
    while !zones_ids.is_empty() {
        let zone_id = zones_ids.pop().unwrap();
        if zones[zone_id].ignited {
            continue;
        }
        reaction_ids.insert(zone_id);
        for &child_id in &zones[zone_id].children {
            zones_ids.push(child_id);
        }
    }
    reaction_ids
}

fn find_best_zone(zones: &Vec<Zone>) -> usize {
    let mut max_size = 0;
    let mut best_zone_id = 0;
    for zone_id in 0..zones.len() {
        if zones[zone_id].ignited {
            continue;
        }
        let reaction_ids = get_reaction_zones(zones, zone_id);
        let size = reaction_ids
            .iter()
            .map(|&zone_id| zones[zone_id].size)
            .sum::<u32>();
        if size > max_size {
            max_size = size;
            best_zone_id = zone_id;
        }
    }
    best_zone_id
}

fn ignite_zone(zones: &mut Vec<Zone>, zone_id: usize) -> u32 {
    let mut ignition_size = 0;
    for reaction_id in get_reaction_zones(zones, zone_id) {
        let reaction_zone = &mut zones[reaction_id];
        ignition_size += reaction_zone.size;
        reaction_zone.ignited = true;
    }
    ignition_size
}

fn find_best_reaction(barrels: &mut Grid<Barrel>) -> u32 {
    let mut zones: Vec<Zone> = build_zones(barrels);
    let mut nb_ignited = 0;
    for _ in 0..3 {
        let zone_id = find_best_zone(&zones);
        nb_ignited += ignite_zone(&mut zones, zone_id);
    }
    nb_ignited
}
