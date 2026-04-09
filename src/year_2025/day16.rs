use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let data: Vec<u64> = data
        .trim_end()
        .split(',')
        .map(|freq| freq.parse().unwrap())
        .collect();

    match args.part {
        1 => count_total_bricks(&data, 90),
        2 => get_spell(data).iter().product(),
        3 => get_wall_length(data),
        _ => unreachable!(),
    }
}

fn count_total_bricks(spell: &[u64], wall_length: u64) -> u64 {
    spell.iter().map(|&freq| wall_length / freq).sum()
}

fn get_spell(mut wall: Vec<u64>) -> Vec<u64> {
    let mut patterns: Vec<u64> = Vec::new();
    for brick in 1..=wall.len() {
        if wall[brick - 1] == 0 {
            continue;
        }

        patterns.push(brick as u64);
        for mult in 1..=(wall.len() / brick) {
            wall[mult * brick - 1] -= 1;
        }
    }
    patterns
}

fn get_wall_length(wall_beginning: Vec<u64>) -> u64 {
    const NB_BRICKS: u64 = 202520252025000;

    let spell = get_spell(wall_beginning);
    let inv_den = spell.iter().map(|&x| 1.0 / x as f64).sum::<f64>();
    let min_col_nb = (NB_BRICKS as f64 / inv_den).floor() as u64;
    let min_col_total_bricks = count_total_bricks(&spell, min_col_nb);
    let mut brick_rest = NB_BRICKS - min_col_total_bricks;

    let mut col_nb = min_col_nb;
    loop {
        let col_brick_nb = spell.iter().filter(|&freq| (col_nb + 1) % freq == 0).count() as u64;
        if brick_rest < col_brick_nb {
            break;
        } else if brick_rest == col_brick_nb {
            col_nb += 1;
            break;
        } else {
            col_nb += 1;
            brick_rest -= col_brick_nb;
        }
    }

    col_nb
}
