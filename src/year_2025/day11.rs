use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let mut flock: Vec<u64> = data.lines().map(|s| s.parse().unwrap()).collect();

    match args.part {
        1 => {
            run_for(&mut flock, 10);
            get_checksum(&flock)
        }
        2 | 3 => count_balancing_rounds(&mut flock),
        _ => unreachable!(),
    }
}

fn get_checksum(flock: &[u64]) -> u64 {
    flock
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &nb)| acc + (i as u64 + 1) * nb as u64)
}

fn run_for(flock: &mut [u64], mut nb_rounds: u64) {
    let mut is_sorted = flock.is_sorted();

    // First phase
    while !is_sorted {
        is_sorted = true;
        for i in 0..flock.len() - 1 {
            if flock[i] > flock[i + 1] {
                flock[i] -= 1;
                flock[i + 1] += 1;
                is_sorted = false;
            }
        }
        nb_rounds -= 1;
        if nb_rounds == 0 {
            return;
        }
    }

    // Second phase
    while nb_rounds > 0 {
        for i in 0..flock.len() - 1 {
            if flock[i] < flock[i + 1] {
                flock[i] += 1;
                flock[i + 1] -= 1;
            }
        }
        nb_rounds -= 1;
    }
}

fn count_balancing_rounds(flock: &mut [u64]) -> u64 {
    let mut nb_rounds = 0;

    // First phase
    let mut i = flock.len() - 1;
    while i > 0 {
        let mut spread_col = 0;
        let mut max_spread = 0;
        let mut duck_nb: u64 = flock[0..=i].iter().sum();
        let mut max_duck_nb = duck_nb;
        for j in 0..=i {
            let duck_spread: u64 = duck_nb.div_ceil((i - j + 1) as u64);
            if duck_spread > max_spread {
                max_spread = duck_spread;
                max_duck_nb = duck_nb;
                spread_col = j;
            }
            duck_nb -= flock[j];
        }
        if i == spread_col {
            i -= 1;
            continue;
        }

        let new_duck_nb = max_duck_nb / ((i - spread_col + 1) as u64);
        let duck_rest = max_duck_nb as usize % (i - spread_col + 1);
        let mut rounds = 0;
        let mut moving_ducks: i64 = 0;
        for k in spread_col..=i {
            let col_duck_nb = new_duck_nb + (if k > i - duck_rest { 1 } else { 0 });
            moving_ducks += flock[k] as i64 - col_duck_nb as i64;
            rounds = rounds.max(moving_ducks);
            flock[k] = col_duck_nb;
        }
        nb_rounds = nb_rounds.max(rounds as u64);
        i = spread_col.saturating_sub(1);
    }

    // Second phase
    let final_duck_nb = flock.iter().sum::<u64>() / flock.len() as u64;
    for j in 0..flock.len() {
        if flock[j] > final_duck_nb {
            break;
        }
        nb_rounds += final_duck_nb - flock[j];
    }
    nb_rounds
}
