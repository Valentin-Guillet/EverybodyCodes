use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");

    match args.part {
        1 => get_total_gear_ratio(&data),
        2 => get_nb_turns(&data),
        3 => get_nb_turns_double_gears(&data),
        _ => unreachable!(),
    }
}

fn get_total_gear_ratio(data: &str) -> u64 {
    let gears: Vec<u64> = data.lines().map(|line| line.parse().unwrap()).collect();
    2025 * gears.first().unwrap() / gears.last().unwrap()
}

fn get_nb_turns(data: &str) -> u64 {
    let gears: Vec<u64> = data.lines().map(|line| line.parse().unwrap()).collect();
    (10000000000000 * gears.last().unwrap()).div_ceil(*gears.first().unwrap())
}

fn get_nb_turns_double_gears(data: &str) -> u64 {
    let mut data_it = data.lines();
    let mut num = 100 * data_it.next().unwrap().parse::<u64>().unwrap();
    let mut den = 1;

    for line in data_it {
        if let Some((a, b)) = line.split_once('|') {
            den *= a.parse::<u64>().unwrap();
            num *= b.parse::<u64>().unwrap();
        } else {
            den *= line.parse::<u64>().unwrap();
        }
        let gcd = get_gcd(num, den);
        num /= gcd;
        den /= gcd;
    }
    num / den
}

fn get_gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
