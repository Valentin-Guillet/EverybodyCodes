use crate::args::RunArgs;

use std::{collections::HashMap, fs::read_to_string, u32};

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let crates: Vec<u32> = data
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    match args.part {
        1 => get_largest_set(&crates, crates.len()),
        2 => get_largest_set(&crates, 20),
        3 => get_most_freq_crate(&crates),
        _ => unreachable!(),
    }
}

fn get_largest_set(crates: &[u32], limit: usize) -> u32 {
    let mut crates = Vec::from(crates);
    crates.sort_unstable();
    let mut set_sum = *crates.get(0).unwrap_or(&0);
    let mut count = 1;
    for i in 0..crates.len() - 1 {
        if crates[i] != crates[i + 1] {
            count += 1;
            set_sum += crates[i + 1];
        }
        if count == limit {
            break;
        }
    }
    set_sum
}

fn get_most_freq_crate(crates: &[u32]) -> u32 {
    let mut freq: HashMap<u32, u32> = HashMap::new();
    for &elt in crates {
        freq.entry(elt).and_modify(|e| *e += 1).or_insert(1);
    }
    *freq.values().max().unwrap()
}
