use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let pinecones = data
        .trim_end()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let pattern_size_fn = match args.part {
        1 => |n: u32| n.pow(2),
        2 => |n: u32| if n < 3 { 8 } else { n.pow(2) - (n - 2).pow(2) },
        3 => |n: u32| if n % 2 == 0 { 0 } else { n.pow(2) - 1},
        _ => unreachable!(),
    };
    get_nb_missing_pinecones(&pinecones, pattern_size_fn)
}

fn get_nb_missing_pinecones(pinecones: &[u32], pattern_size_fn: fn(u32) -> u32) -> u32 {
    let mut pattern_sizes: Vec<u32> = Vec::new();
    let mut nb_missing_pinecones = 0;
    for &nb_pinecone in pinecones {
        while nb_pinecone > *pattern_sizes.last().unwrap_or(&0) {
            pattern_sizes.push(pattern_size_fn(pattern_sizes.len() as u32));
        }
        let min_pattern_size = pattern_sizes
            .iter()
            .find(|&&pattern_size| pattern_size >= nb_pinecone)
            .unwrap();
        nb_missing_pinecones += min_pattern_size - nb_pinecone;
    }
    nb_missing_pinecones
}
