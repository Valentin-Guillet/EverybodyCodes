use crate::args::RunArgs;

use std::collections::HashMap;
use std::fs::read_to_string;

struct Cloud {
    id: usize,
    energy: u32,
    connections: Vec<usize>,
}

impl Cloud {
    fn new(input: &str) -> Self {
        let (id, input) = input
            .split_once(", ")
            .map(|(id, rest)| (id[3..].parse::<u32>().unwrap() as usize, rest))
            .unwrap();
        let (energy, input) = input
            .split_once(", ")
            .map(|(energy, rest)| (energy[7..].parse::<u32>().unwrap(), rest))
            .unwrap();
        let connections = input[12..]
            .split(',')
            .map(|val| val.parse().unwrap())
            .collect();
        Self {
            id,
            energy,
            connections,
        }
    }
}

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let clouds: Vec<Cloud> = data
        .trim_end()
        .lines()
        .map(|line| Cloud::new(line))
        .collect();

    match args.part {
        1 => send_impulse(&clouds, 5),
        2 => send_impulse(&clouds, 10),
        3 => (1..=clouds.len())
            .map(|cloud_id| send_impulse_with_explosions(&clouds, cloud_id, 10))
            .max()
            .unwrap(),
        // 3 => send_impulse_with_explosions(&clouds, 3, 10),
        _ => unreachable!(),
    }
}

fn send_impulse(clouds: &[Cloud], init_strength: u32) -> u32 {
    let mut energies = clouds.iter().map(|cloud| cloud.energy).collect::<Vec<_>>();
    let init_cloud_id = clouds
        .iter()
        .max_by_key(|cloud| cloud.connections.len())
        .unwrap()
        .id;

    let mut next_impulses: HashMap<(usize, u32), u32> =
        HashMap::from([((init_cloud_id, init_strength), 1)]);

    while !next_impulses.is_empty() {
        let impulses = std::mem::take(&mut next_impulses);
        for ((cloud_id, strength), nb_impulses) in impulses {
            energies[cloud_id - 1] += nb_impulses * strength;
            if strength == 1 {
                continue;
            }
            for &neighbor_id in &clouds[cloud_id - 1].connections {
                next_impulses
                    .entry((neighbor_id, strength - 1))
                    .and_modify(|e| *e += nb_impulses)
                    .or_insert(nb_impulses);
            }
        }
    }

    energies.iter().sum()
}

fn send_impulse_with_explosions(clouds: &[Cloud], init_cloud_id: usize, init_strength: u32) -> u32 {
    const LIMIT: u32 = 1000000;

    let mut energies = clouds.iter().map(|cloud| cloud.energy).collect::<Vec<_>>();
    let mut next_impulses: HashMap<(usize, u32), u32> =
        HashMap::from([((init_cloud_id, init_strength), 1)]);
    let mut exploded = vec![false; clouds.len()];

    while !next_impulses.is_empty() {
        let impulses = std::mem::take(&mut next_impulses);
        for ((cloud_id, strength), nb_impulses) in impulses {
            if exploded[cloud_id - 1] {
                continue;
            }

            energies[cloud_id - 1] += nb_impulses * strength;
            if strength == 1 {
                continue;
            }
            for &neighbor_id in &clouds[cloud_id - 1].connections {
                if exploded[neighbor_id - 1] {
                    continue;
                }
                next_impulses
                    .entry((neighbor_id, strength - 1))
                    .and_modify(|e| *e += nb_impulses)
                    .or_insert(nb_impulses);
            }
        }

        for id in (0..clouds.len()).filter(|&i| energies[i] >= LIMIT) {
            if exploded[id] {
                continue;
            }
            exploded[id] = true;
            for &neighbor_id in &clouds[id].connections {
                next_impulses
                    .entry((neighbor_id, 10))
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    (0..clouds.len())
        .filter(|&i| !exploded[i])
        .map(|i| energies[i])
        .sum()
}
