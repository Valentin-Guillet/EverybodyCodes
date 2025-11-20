use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u32 {
    let soldiers = read_to_string(&args.input_file).expect("Error opening input file");
    let soldiers = soldiers.trim_end().chars().collect::<Vec<_>>();

    match args.part {
        1 => count_sword_mentors(&soldiers),
        2 => count_mentors(&soldiers),
        3 => count_mentors_in_window(&soldiers, 1000, 1000),
        _ => unreachable!(),
    }
}

fn count_sword_mentors(soldiers: &[char]) -> u32 {
    let mut count = 0;
    let mut nb_mentors = 0;
    for soldier in soldiers {
        match soldier {
            'A' => nb_mentors += 1,
            'a' => count += nb_mentors,
            _ => (),
        }
    }
    count
}

fn count_mentors(soldiers: &[char]) -> u32 {
    let mut count = 0;
    let mut nb_mentors = [0, 0, 0];
    for soldier in soldiers {
        match soldier {
            'A' => nb_mentors[0] += 1,
            'a' => count += nb_mentors[0],
            'B' => nb_mentors[1] += 1,
            'b' => count += nb_mentors[1],
            'C' => nb_mentors[2] += 1,
            'c' => count += nb_mentors[2],
            _ => (),
        }
    }
    count
}

fn count_mentors_in_window(soldiers: &[char], window: usize, repeat: u32) -> u32 {
    // Compute nb mentors with repeat for all the inner segments
    let mut inner_mentor_nb = [0, 0, 0];
    let full_window: Vec<usize> = (0..=window)
        .chain(soldiers.len() - window..soldiers.len())
        .collect();
    inner_mentor_nb[0] = full_window.iter().filter(|&&i| soldiers[i] == 'A').count() as u32;
    inner_mentor_nb[1] = full_window.iter().filter(|&&i| soldiers[i] == 'B').count() as u32;
    inner_mentor_nb[2] = full_window.iter().filter(|&&i| soldiers[i] == 'C').count() as u32;

    let mut inner_count = 0;
    for (i, soldier) in soldiers.iter().enumerate() {
        match soldier {
            'a' => inner_count += inner_mentor_nb[0],
            'b' => inner_count += inner_mentor_nb[1],
            'c' => inner_count += inner_mentor_nb[2],
            _ => (),
        }
        let left_window = (i + soldiers.len() - window) % soldiers.len();
        let right_window = (i + 1 + window) % soldiers.len();
        match soldiers[left_window] {
            'A' => inner_mentor_nb[0] -= 1,
            'B' => inner_mentor_nb[1] -= 1,
            'C' => inner_mentor_nb[2] -= 1,
            _ => (),
        }
        match soldiers[right_window] {
            'A' => inner_mentor_nb[0] += 1,
            'B' => inner_mentor_nb[1] += 1,
            'C' => inner_mentor_nb[2] += 1,
            _ => (),
        }
    }

    // Remove mentors on each extremities
    let mut outer_count = 0;
    let mut left_outer_mentor_nb = [0, 0, 0];
    for i in (0..window).rev() {
        match soldiers[soldiers.len() - window + i] {
            'A' => left_outer_mentor_nb[0] += 1,
            'B' => left_outer_mentor_nb[1] += 1,
            'C' => left_outer_mentor_nb[2] += 1,
            _ => (),
        }
        match soldiers[i] {
            'a' => outer_count += left_outer_mentor_nb[0],
            'b' => outer_count += left_outer_mentor_nb[1],
            'c' => outer_count += left_outer_mentor_nb[2],
            _ => (),
        }
    }

    let mut right_outer_mentor_nb = [0, 0, 0];
    for i in 0..window {
        match soldiers[i] {
            'A' => right_outer_mentor_nb[0] += 1,
            'B' => right_outer_mentor_nb[1] += 1,
            'C' => right_outer_mentor_nb[2] += 1,
            _ => (),
        }
        match soldiers[soldiers.len() - window + i] {
            'a' => outer_count += right_outer_mentor_nb[0],
            'b' => outer_count += right_outer_mentor_nb[1],
            'c' => outer_count += right_outer_mentor_nb[2],
            _ => (),
        }
    }
    repeat * inner_count - outer_count
}
