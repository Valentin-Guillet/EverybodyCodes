use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");

    match args.part {
        1 => turn_dial(&data, 2025),
        2 => turn_dial_intervals(&data, 20252025),
        3 => turn_dial_intervals(&data, 202520252025),
        _ => unreachable!(),
    }
}

fn turn_dial(data: &str, nb_turns: u32) -> u32 {
    let dials: Vec<u32> = data.lines().map(|line| line.parse().unwrap()).collect();
    let n = dials.len() + 1;
    let nb_turns = nb_turns as usize % n;
    if nb_turns == 0 {
        return 1;
    }
    for (i, &dial) in dials.iter().enumerate() {
        let index = if i % 2 == 0 { i / 2 + 1 } else { n - 1 - i / 2 };
        if index == nb_turns {
            return dial;
        }
    }
    unreachable!()
}

fn turn_dial_intervals(data: &str, nb_turns: u64) -> u32 {
    let intervals: Vec<(u32, u32)> = data
        .lines()
        .map(|line| {
            let (min, max) = line.split_once('-').unwrap();
            (min.parse().unwrap(), max.parse().unwrap())
        })
        .collect();

    let lock_size = intervals
        .iter()
        .map(|(min, max)| max - min + 1)
        .sum::<u32>()
        + 1;
    let lock_index = (nb_turns % lock_size as u64) as u32;
    if lock_index == 0 {
        return 1;
    }
    let mut nb_seen = 1;
    for index in 0..(intervals.len() + 1) / 2 {
        let interval_index = 2 * index;
        let (min, max) = intervals[interval_index];
        if lock_index < nb_seen + max - min + 1 {
            return min + (lock_index - nb_seen);
        }
        nb_seen += max - min + 1;
    }
    for index in (0..intervals.len() / 2).rev() {
        let interval_index = 2 * index + 1;
        let (min, max) = intervals[interval_index];
        if lock_index < nb_seen + max - min + 1 {
            return max - (lock_index - nb_seen);
        }
        nb_seen += max - min + 1;
    }
    unreachable!()
}
