use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> i64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");

    match args.part {
        1 => run_part1(data),
        2 => run_part2(data),
        3 => run_part3(data),
        _ => unreachable!(),
    }
}

fn get_score(c: char) -> i64 {
    match c {
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => 0,
    }
}

fn run_part1(input: String) -> i64 {
    input.chars().map(get_score).sum()
}

fn run_part2(input: String) -> i64 {
    let mut score = 0;
    let input: Vec<char> = input.chars().collect();
    for i in (0..input.len() - 1).step_by(2) {
        score += match (input[i], input[i + 1]) {
            ('x', 'x') => 0,
            ('x', c) | (c, 'x') => get_score(c),
            (c, d) => get_score(c) + get_score(d) + 2,
        }
    }
    score
}

fn run_part3(input: String) -> i64 {
    let mut score = 0;
    let input: Vec<char> = input.chars().collect();
    for i in (0..input.len() - 1).step_by(3) {
        score += match (input[i], input[i + 1], input[i + 2]) {
            ('x', 'x', 'x') => 0,
            ('x', 'x', c) | ('x', c, 'x') | (c, 'x', 'x') => get_score(c),
            ('x', c, d) | (c, d, 'x') | (c, 'x', d) => get_score(c) + get_score(d) + 2,
            (c, d, e) => get_score(c) + get_score(d) + get_score(e) + 6,
        }
    }
    score
}
