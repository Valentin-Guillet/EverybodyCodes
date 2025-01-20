use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> i32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let nails: Vec<i32> = data.lines().map(|line| line.parse().unwrap()).collect();

    match args.part {
        1 | 2 => count_min_strikes(nails),
        3 => count_min_strikes_and_pulls(nails),
        _ => unreachable!(),
  }
}

fn count_min_strikes(nails: Vec<i32>) -> i32 {
    let min_nail = nails.iter().min().unwrap();
    nails.iter().map(|nail| nail - min_nail).sum()
}

fn count_min_strikes_and_pulls(mut nails: Vec<i32>) -> i32 {
    nails.sort();
    let median = nails[nails.len() / 2];
    nails.iter().map(|nail| (nail - median).abs()).sum()
}
