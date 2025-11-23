use crate::args::RunArgs;

use std::fs::read_to_string;

type Nail = usize;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let nails: Vec<Nail> = data
        .trim_end()
        .split(',')
        .map(|s| s.parse::<Nail>().unwrap())
        .collect();

    match args.part {
        1 => count_center_threads(&nails),
        2 => count_knots(&nails),
        3 => count_cut_threads(&nails),
        _ => unreachable!(),
    }
}

fn count_center_threads(nails: &[Nail]) -> u32 {
    let mut count = 0;
    for i in 0..nails.len() - 1 {
        if nails[i] == ((nails[i + 1] + 15) % 32) + 1 {
            count += 1;
        }
    }
    count
}

fn do_intersect(a: Nail, b: Nail, c: Nail, d: Nail) -> bool {
    return ((a < c && c < b) && (b < d || d < a)) || ((a < d && d < b) && (b < c || c < a));
}

fn count_knots(nails: &[Nail]) -> u32 {
    let mut count = 0;
    for i in 2..nails.len() - 1 {
        for j in 0..=i - 2 {
            let (a, b) = (nails[i], nails[i + 1]);
            let (a, b) = (a.min(b), a.max(b));
            let (c, d) = (nails[j], nails[j + 1]);
            if do_intersect(a, b, c, d) {
                count += 1;
            }
        }
    }
    count
}

fn count_cut_threads(nails: &[Nail]) -> u32 {
    let mut max_cut = 0;
    for a in 0..256 {
        for b in a + 2..256 {
            let nb_cuts = (0..nails.len() - 1)
                .filter(|&i| do_intersect(a, b, nails[i], nails[i + 1]))
                .count() as u32;
            if nb_cuts > max_cut {
                max_cut = nb_cuts;
            }
        }
    }
    max_cut
}
