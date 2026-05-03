use crate::args::RunArgs;

use std::fs::read_to_string;

struct Pos {
    x: u64,
    y: u64,
}

impl Pos {
    fn from(input: &str) -> Self {
        let words = input.split(' ').collect::<Vec<_>>();
        Self {
            x: words[0][2..].parse().unwrap(),
            y: words[1][2..].parse().unwrap(),
        }
    }
}

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let snails: Vec<Pos> = data.trim_end().lines().map(Pos::from).collect();

    match args.part {
        1 => snails
            .iter()
            .map(|snail| get_snail_value_after(snail, 100))
            .sum(),
        2 | 3 => {
            snails
                .iter()
                .map(|snail| (snail.y - 1, snail.x + snail.y - 1))
                .reduce(chinese_remainder_theorem)
                .unwrap()
                .0
        }
        _ => unreachable!(),
    }
}

fn get_snail_value_after(snail: &Pos, days: u64) -> u64 {
    let sum = (snail.x + snail.y) - 1;
    let x = (snail.x - 1 + days) % sum + 1;
    let y = (snail.y - 1 + sum - (days % sum)) % sum + 1;
    (x + 100 * y) as u64
}

fn bezout(a: u64, b: u64) -> (i64, i64) {
    let (mut old_r, mut r) = (a as i64, b as i64);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r > 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }
    (old_s, old_t)
}

fn chinese_remainder_theorem((rem1, mod1): (u64, u64), (rem2, mod2): (u64, u64)) -> (u64, u64) {
    let (a, b) = bezout(mod1, mod2).into();
    let (a, b) = (a as i128, b as i128);

    let m = mod1 * mod2;
    let mut value = (rem2 * mod1).rem_euclid(m) as i128 * a.rem_euclid(m as i128);
    value += (rem1 * mod2).rem_euclid(m) as i128 * b.rem_euclid(m as i128);
    value = value.rem_euclid(m as i128);

    (value as u64, mod1 * mod2)
}
