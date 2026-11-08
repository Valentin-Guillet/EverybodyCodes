use crate::args::RunArgs;

use std::{collections::HashMap, fs::read_to_string};

struct Plant {
    growth: u64,
    limit: u64,
}

impl Plant {
    fn new(input: &str) -> Self {
        let input = &input[input.find(',').unwrap() + 2..];
        let (growth, input) = input
            .split_once(", ")
            .map(|(growth, rest)| (growth[7..].parse::<u64>().unwrap(), rest))
            .unwrap();
        let limit = input[6..].parse().unwrap();
        Self { growth, limit }
    }
}

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let plants: Vec<Plant> = data.trim_end().lines().map(Plant::new).collect();

    match args.part {
        1 => plants
            .iter()
            .map(|plant| compute_growth_time(plant, true))
            .sum(),
        2 => plants
            .iter()
            .map(|plant| compute_growth_time(plant, false))
            .sum(),
        3 => plants.iter().map(compute_efficient_growth_time).sum(),
        _ => unreachable!(),
    }
}

fn compute_growth_time(plant: &Plant, nb_elts_limit: bool) -> u64 {
    let mut branches = vec![0];

    let mut nb_elts = 0;
    let mut nb_leaves = 1;

    let mut nb_seconds = 0;
    while if nb_elts_limit { nb_elts } else { nb_leaves } < plant.limit {
        let mut new_branches = Vec::new();
        for branch in &mut branches {
            *branch += 1;
            nb_elts += 1;
            if *branch == plant.growth {
                nb_leaves += 1;
                nb_elts += 1;
                *branch /= 2;
                new_branches.push(*branch);
                new_branches.push(1);
            }
        }
        branches.extend(new_branches);
        nb_seconds += 1;
    }
    nb_seconds
}

fn compute_efficient_growth_time(plant: &Plant) -> u64 {
    if plant.limit == 1 {
        return 1;
    }

    let mut branches: HashMap<u64, u64> = HashMap::from([(0, 1)]);
    let mut nb_leaves = 1;
    let mut nb_seconds = 0;
    while nb_leaves < plant.limit {
        let delta = branches
            .iter()
            .map(|(branch_size, _)| plant.growth - branch_size)
            .min()
            .unwrap();

        let mut new_branches = HashMap::new();
        for (&size, number) in &mut branches {
            let mut new_size = size + delta;
            if new_size == plant.growth {
                new_size /= 2;
                nb_leaves += *number;
                new_branches
                    .entry(1)
                    .and_modify(|e| *e += *number)
                    .or_insert(*number);
                new_branches
                    .entry(new_size)
                    .and_modify(|e| *e += 2 * *number)
                    .or_insert(2 * *number);
            } else {
                new_branches
                    .entry(new_size)
                    .and_modify(|e| *e += *number)
                    .or_insert(*number);
            }
        }
        branches = new_branches;
        nb_seconds += delta;
    }
    nb_seconds
}
