use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let star_pos: Vec<(usize, usize)> = data
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '*')
                .map(|(col, _)| (row, col))
                .collect::<Vec<_>>()
        })
        .collect();

    match args.part {
        1 | 2 => get_constellation_size(&star_pos),
        3 => get_small_constellation_size(&star_pos),
        _ => unreachable!(),
    }
}

fn compute_dist(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1)
}

fn get_constellation_size(star_pos: &[(usize, usize)]) -> u32 {
    let mut total_dist = 0;
    let mut in_constellation: Vec<usize> = vec![0];
    let mut out_constellation: Vec<usize> = (1..star_pos.len()).collect();

    while !out_constellation.is_empty() {
        let mut closest_star_ind = 0;
        let mut smallest_dist = usize::MAX;

        for (out_ind, &star_out) in out_constellation.iter().enumerate() {
            for &star_in in &in_constellation {
                let dist = compute_dist(star_pos[star_in], star_pos[star_out]);
                if dist < smallest_dist {
                    closest_star_ind = out_ind;
                    smallest_dist = dist;
                }
            }
        }

        total_dist += smallest_dist;
        in_constellation.push(out_constellation[closest_star_ind]);
        out_constellation.swap_remove(closest_star_ind);
    }

    (star_pos.len() + total_dist) as u32
}

fn get_small_constellation_size(star_pos: &[(usize, usize)]) -> u32 {
    let mut brilliant_constellation_sizes: Vec<u32> = Vec::new();

    let mut avail_stars: Vec<usize> = (0..star_pos.len()).collect();
    while let Some(first_star) = avail_stars.pop() {
        let mut frontier = vec![first_star];
        let mut stars_in_constellation: Vec<(usize, usize)> = Vec::new();
        while let Some(star) = frontier.pop() {
            stars_in_constellation.push(star_pos[star]);
            let close_star_indices: Vec<usize> = (0..avail_stars.len())
                .filter(|&ind| compute_dist(star_pos[star], star_pos[avail_stars[ind]]) < 6)
                .collect();
            frontier.extend(close_star_indices.iter().map(|&ind| avail_stars[ind]));
            for &star_index in close_star_indices.iter().rev() {
                avail_stars.swap_remove(star_index);
            }
        }

        brilliant_constellation_sizes.push(get_constellation_size(&stars_in_constellation));
    }

    brilliant_constellation_sizes.sort();
    brilliant_constellation_sizes.iter().rev().take(3).product()
}
