use crate::args::RunArgs;

use std::fs::read_to_string;

type Target = (usize, usize, bool);

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let data: Vec<&str> = data.lines().collect::<Vec<_>>();

    match args.part {
        1 | 2 => parse_targets(&data)
            .iter()
            .copied()
            .map(get_ranking_value)
            .sum(),
        3 => parse_meteors(&data)
            .iter()
            .copied()
            .map(shoot_meteor)
            .sum(),
        _ => unreachable!(),
    }
}

fn parse_targets(data: &[&str]) -> Vec<Target> {
    data.iter()
        .rev()
        .skip(1)
        .enumerate()
        .flat_map(|(row, &line)| {
            line.chars()
                .skip(1)
                .enumerate()
                .filter(|(_, c)| *c == 'T' || *c == 'H')
                .map(move |(i, c)| (row, i, c == 'H'))
        })
        .collect()
}

fn get_ranking_value((mut x, mut y, double): Target) -> u32 {
    let mult = if double { 2 } else { 1 };
    // Can't reach target on odd column from origin
    if y % 2 == 1 {
        (x, y) = (x + 1, y - 1);
    }

    // Target is too far
    while x < y / 2 {
        (x, y) = (x + 2, y - 2);
    }

    loop {
        let x_orig = x - y / 2;
        if (0..4).contains(&x_orig) {
            return mult * ((x_orig + 1) * y / 2) as u32;
        }
        (x, y) = (x + 2, y - 2);
    }
}

fn parse_meteors(data: &[&str]) -> Vec<(usize, usize)> {
    data.iter()
        .map(|&line| {
            let (x, y) = line.split_once(' ').unwrap();
            (y.parse().unwrap(), x.parse().unwrap())
        })
        .collect()
}

fn ranking_score_to_meteor(row: usize, (x, y): (usize, usize)) -> Option<u32> {
    // Meteor too high: can't reach it with 45° angle
    if y + row < x {
        return None;
    }

    // Meteor can be reached before the projectile goes down
    if y + 2 * row <= 2 * x {
        return Some(((row + 1) * (x - row)) as u32);
    }

    // Can't reach the meteor before it reaches the ground
    if x + y <= row {
        return None;
    }

    if (x + y - row) % 3 != 0 {
        return None;
    }

    Some(((row + 1) * (x + y - row) / 3) as u32)
}

fn shoot_meteor((mut x, mut y): (usize, usize)) -> u32 {
    // Wait enough time for a projectile to be able to reach the meteor
    (x, y) = (x - y.div_ceil(2), y / 2);

    loop {
        let score_iter = (0..3).filter_map(|row| {
            ranking_score_to_meteor(row, (x, y))
        }).min();

        if let Some(ranking_score) = score_iter {
            return ranking_score;
        }
        (x, y) = (x - 1, y - 1);
    }
}
