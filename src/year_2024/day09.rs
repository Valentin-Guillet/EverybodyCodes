use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let brightness_list: Vec<u32> = data.lines().map(|n| n.trim().parse().unwrap()).collect();

    let mut stamps: Vec<u32> = vec![1, 3, 5, 10];
    if args.part > 1 {
        stamps.extend_from_slice(&[15, 16, 20, 24, 25, 30]);
    }
    if args.part == 3 {
        stamps.extend_from_slice(&[37, 38, 49, 50, 74, 75, 100, 101]);
    }

    let count_fn = match args.part {
        1 => count_min_beetles_greedy,
        2 => count_min_beetles_dyn,
        3 => count_min_beetles_split,
        _ => unreachable!(),
    };

    let mut cache = vec![0; *brightness_list.iter().max().unwrap() as usize];
    brightness_list
        .iter()
        .map(|&b| count_fn(&mut cache, &stamps, b))
        .sum()
}

fn count_min_beetles_greedy(_: &mut [u32], stamps: &[u32], mut brightness: u32) -> u32 {
    let mut nb_beetles = 0;
    for i in (0..4).rev() {
        nb_beetles += brightness / stamps[i];
        brightness %= stamps[i];
    }
    nb_beetles
}

fn count_min_beetles_dyn(cache: &mut [u32], stamps: &[u32], brightness: u32) -> u32 {
    if brightness == 0 {
        return 0;
    }
    let cache_index = (brightness - 1) as usize;
    if cache[cache_index] != 0 {
        return cache[cache_index];
    }
    let min_count_stamps = stamps
        .iter()
        .rev()
        .filter(|&&stamp| stamp <= brightness)
        .map(|&stamp| count_min_beetles_dyn(cache, stamps, brightness - stamp))
        .min()
        .unwrap()
        + 1;
    cache[cache_index] = min_count_stamps;
    min_count_stamps
}

fn count_min_beetles_split(cache: &mut [u32], stamps: &[u32], brightness: u32) -> u32 {
    (0..50)
        .map(|diff| {
            count_min_beetles_dyn(cache, stamps, (brightness / 2) + diff)
                + count_min_beetles_dyn(cache, stamps, (brightness / 2) - diff)
        })
        .min()
        .unwrap()
}
