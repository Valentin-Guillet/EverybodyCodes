use crate::args::RunArgs;

use std::{collections::HashMap, fs::read_to_string};

type DNA = Vec<char>;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let dnas = read_dnas(&data);

    match args.part {
        1 => compute_similarity(&dnas),
        2 => compute_all_similarities(&dnas),
        3 => get_largest_family(&dnas),
        _ => unreachable!(),
    }
}

fn read_dnas(input: &str) -> Vec<DNA> {
    input
        .lines()
        .map(|line| {
            let index = line.find(':').unwrap();
            Vec::from_iter(line[index + 1..].chars())
        })
        .collect()
}

fn get_child_index(dnas: &[DNA]) -> usize {
    let mut possible_child_idx = vec![0, 1, 2];
    for i in 0..dnas[0].len() {
        possible_child_idx = possible_child_idx
            .into_iter()
            .filter(|&possible_id| {
                let parent1_id = (possible_id + 1) % 3;
                let parent2_id = (possible_id + 2) % 3;
                dnas[possible_id][i] != dnas[parent1_id][i]
                    && dnas[possible_id][i] == dnas[parent2_id][i]
            })
            .collect();

        if possible_child_idx.len() == 1 {
            return possible_child_idx[0];
        }
    }
    unreachable!()
}

fn count_matches(dna1: &[char], dna2: &[char]) -> u32 {
    (0..dna1.len()).filter(|&i| dna1[i] == dna2[i]).count() as u32
}

fn compute_similarity(dnas: &[DNA]) -> u32 {
    let child_id = get_child_index(dnas);
    let parent1_id = (child_id + 1) % 3;
    let parent2_id = (child_id + 2) % 3;
    count_matches(&dnas[child_id], &dnas[parent1_id])
        * count_matches(&dnas[child_id], &dnas[parent2_id])
}

fn check_is_child(candidate: &DNA, parent1: &DNA, parent2: &DNA) -> bool {
    for i in 0..candidate.len() {
        if !(candidate[i] == parent1[i] || candidate[i] == parent2[i]) {
            return false;
        }
    }
    true
}

fn compute_all_similarities(dnas: &[DNA]) -> u32 {
    let dna_size = dnas[0].len() as u32;
    let mut similarity_sum = 0;
    for (i, curr_dna) in dnas.iter().enumerate() {
        let nb_commons: Vec<u32> = dnas
            .iter()
            .map(|other| count_matches(curr_dna, other))
            .collect();

        for p1_id in 0..dnas.len() {
            if p1_id == i {
                continue;
            }

            for p2_id in p1_id + 1..dnas.len() {
                if p2_id == i || nb_commons[p1_id] + nb_commons[p2_id] < dna_size {
                    continue;
                }

                if check_is_child(curr_dna, &dnas[p1_id], &dnas[p2_id]) {
                    similarity_sum += nb_commons[p1_id] * nb_commons[p2_id];
                }
            }
        }
    }
    similarity_sum
}

fn get_largest_family(dnas: &[DNA]) -> u32 {
    let dna_size = dnas[0].len() as u32;
    let mut family_idx = Vec::from_iter(0..dnas.len() as u32);
    for (i, curr_dna) in dnas.iter().enumerate() {
        let nb_commons: Vec<u32> = dnas
            .iter()
            .map(|other| count_matches(curr_dna, other))
            .collect();

        for p1_id in 0..dnas.len() {
            if p1_id == i {
                continue;
            }

            for p2_id in p1_id + 1..dnas.len() {
                if p2_id == i || nb_commons[p1_id] + nb_commons[p2_id] < dna_size {
                    continue;
                }

                if check_is_child(curr_dna, &dnas[p1_id], &dnas[p2_id]) {
                    let child_family_id = family_idx[i];
                    let p1_family_id = family_idx[p1_id];
                    let p2_family_id = family_idx[p2_id];
                    family_idx[i] = p1_family_id;
                    for elt in family_idx.iter_mut() {
                        if *elt == child_family_id || *elt == p2_family_id {
                            *elt = p1_family_id;
                        }
                    }
                }
            }
        }
    }
    let mut freq: HashMap<u32, u32> = HashMap::new();
    let mut largest_family_id = 0;
    let mut largest_family_size = 0;
    for &id in &family_idx {
        let f = freq.get(&id).unwrap_or(&0) + 1;
        freq.insert(id, f);
        if f > largest_family_size {
            largest_family_size = f + 1;
            largest_family_id = id;
        }
    }
    family_idx
        .iter()
        .enumerate()
        .filter(|&(_, id)| *id == largest_family_id)
        .map(|(i, _)| i as u32 + 1)
        .sum()
}
