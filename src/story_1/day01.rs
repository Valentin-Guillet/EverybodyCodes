use crate::args::RunArgs;

use std::collections::HashMap;
use std::fs::read_to_string;

type Input = HashMap<char, u64>;

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let values: Vec<Input> = data.trim_end().lines().map(parse_line).collect();

    let eni_fn = match args.part {
        1 => |n, e, m| compute_eni(n, e, m, 1),
        2 => compute_eni_long,
        3 => compute_eni_sum,
        _ => unreachable!(),
    };
    values
        .iter()
        .map(|val| compute_formula(val, eni_fn))
        .max()
        .unwrap()
}

fn parse_line(line: &str) -> Input {
    line.split(' ')
        .map(|assign| (assign.chars().next().unwrap(), assign[2..].parse().unwrap()))
        .collect()
}

fn compute_eni(n: u64, exp: u64, modulo: u64, mut score: u64) -> u64 {
    let mut remainder_vec = Vec::new();
    for _ in 0..exp {
        score = (score * n) % modulo;
        remainder_vec.push(score);
    }
    remainder_vec
        .iter()
        .map(|val| val.to_string())
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn compute_eni_long(n: u64, exp: u64, modulo: u64) -> u64 {
    let mut score = 1;
    for c in format!("{:b}", exp - 5).chars() {
        if c == '1' {
            score = (score * score * n) % modulo;
        } else {
            score = (score * score) % modulo;
        }
    }
    compute_eni(n, 5, modulo, score)
}

fn compute_eni_sum(n: u64, exp: u64, modulo: u64) -> u64 {
    let n = n % modulo;

    let mut remainders: HashMap<u64, usize> = HashMap::with_capacity(10000);
    let mut score = 1;
    for i in 0.. {
        score = (score * n) % modulo;
        if remainders.contains_key(&score) {
            break;
        }
        remainders.insert(score, i);
    }

    let seq_start = remainders[&score];
    let seq_len = (remainders.len() - seq_start) as u64;

    let q = (exp - seq_start as u64) / seq_len;
    let r = ((exp - seq_start as u64) % seq_len) as usize;

    let mut rem_before = 0;
    let mut rem_repeat = 0;
    let mut rem_after = 0;
    for (rem, index) in remainders {
        if index < seq_start {
            rem_before += rem;
            continue;
        }
        rem_repeat += rem;
        if index < seq_start + r {
            rem_after += rem;
        }
    }
    rem_before + q * rem_repeat + rem_after
}

fn compute_formula(input: &Input, eni_fn: fn(u64, u64, u64) -> u64) -> u64 {
    eni_fn(input[&'A'], input[&'X'], input[&'M'])
        + eni_fn(input[&'B'], input[&'Y'], input[&'M'])
        + eni_fn(input[&'C'], input[&'Z'], input[&'M'])
}
