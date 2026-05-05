use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let balloons: Vec<char> = data.lines().next().unwrap().chars().collect();

    match args.part {
        1 => count_linear_fluffbolts(&balloons),
        2 => count_circular_fluffbolts(&balloons, 100),
        3 => count_circular_fluffbolts(&balloons, 100000),
        _ => unreachable!(),
    }
}

fn count_linear_fluffbolts(balloons: &[char]) -> u32 {
    let mut nb_fluffbolts = 0;
    let mut bolt_it = "RGB".chars().cycle();
    let mut index = 0;
    while index < balloons.len() {
        let bolt = bolt_it.next().unwrap();
        while index < balloons.len() && balloons[index] == bolt {
            index += 1;
        }
        index += 1;
        nb_fluffbolts += 1;
    }
    nb_fluffbolts
}

fn count_circular_fluffbolts(balloons: &[char], nb_repeats: usize) -> u32 {
    let mut nb_fluffbolts = 0;
    let mut popped: Vec<usize> = Vec::new();
    let mut popped_index = 0;

    let mut bolt_it = "RGB".chars().cycle();
    let mut index = 0;
    let mut double_opposite_index = nb_repeats * balloons.len();
    while index < nb_repeats * balloons.len() {
        // Balloon already popped with an opposite dart, skip
        while popped_index < popped.len() && popped[popped_index] == index {
            popped_index += 1;
            index += 1;
        }
        let bolt = bolt_it.next().unwrap();

        // Pop on the opposite side
        if double_opposite_index % 2 == 0 && balloons[index % balloons.len()] == bolt {
            popped.push(double_opposite_index / 2);
            double_opposite_index += 2;
        } else {
            double_opposite_index += 1;
        }
        index += 1;
        nb_fluffbolts += 1;
    }
    nb_fluffbolts
}
