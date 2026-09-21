use crate::args::RunArgs;

use std::collections::HashSet;
use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let sequences: Vec<Vec<u32>> = data
        .trim_end()
        .lines()
        .map(|sequence| sequence.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    let compute_fn = match args.part {
        1 => |seq| compute_final_point(seq, true),
        2 => |seq| compute_final_point(seq, false),
        3 => |seq| compute_final_point_wo_crossings(seq),
        _ => unreachable!(),
    };
    sequences.iter().map(|x| compute_fn(x)).sum()
}

fn compute_final_point(sequence: &[u32], allow_doubles: bool) -> u32 {
    let mut seen: HashSet<u32> = HashSet::new();
    let mut value = 0;
    for &delta in sequence {
        if delta < value && !seen.contains(&(value - delta)) {
            value -= delta;
        } else {
            value += delta;
            while !allow_doubles && seen.contains(&value) {
                value += 1;
            }
        }
        seen.insert(value);
    }
    value
}

fn jump_backward(value: u32, delta: u32, jumps: &[(u32, u32)]) -> Option<u32> {
    if delta >= value {
        return None;
    }

    let target = value - delta;
    jumps
        .iter()
        .all(|&(src, dst)| src > value || target > dst || (dst > value) == (target > src))
        .then_some(target)
}

fn jump_forward(value: u32, delta: u32, seen: &HashSet<u32>, jumps: &[(u32, u32)]) -> Option<u32> {
    let mut intervals: Vec<(u32, u32)> = vec![(value + delta, u32::MAX)];

    for &(src, dst) in jumps {
        let mut index = 0;
        while index < intervals.len() {
            let (lower, upper) = &mut intervals[index];
            if dst < value || src > *upper || (value < src && dst < *lower) {
                index += 1;
                continue;
            }

            if (src < value && dst <= *lower) || (value < src && src <= *lower && dst >= *upper) {
                intervals.swap_remove(index);
                continue;
            }

            if src < value && dst <= *upper {
                *upper = dst - 1;
            } else if value < src && src <= *lower && dst < *upper {
                *lower = dst + 1;
            } else if *lower < src {
                let prev_upper = *upper;
                *upper = src - 1;
                if dst < prev_upper {
                    intervals.push((dst + 1, prev_upper));
                }
            }
            index += 1;
        }
    }
    intervals.sort();
    intervals
        .iter()
        .find_map(|&(lower, upper)| (lower..=upper).find(|pos| !seen.contains(&pos)))
}

fn compute_final_point_wo_crossings(sequence: &[u32]) -> u32 {
    let mut seen: HashSet<u32> = HashSet::new();
    let mut jumps: [Vec<(u32, u32)>; 2] = [Vec::new(), Vec::new()];
    let mut jump_index = 0;
    let mut value = 0;
    for &delta in sequence {
        if let Some(next_value) = jump_backward(value, delta, &jumps[jump_index])
            && !seen.contains(&next_value)
        {
            jumps[jump_index].push((next_value, value));
            seen.insert(next_value);
            value = next_value;
            jump_index = 1 - jump_index;
        } else if let Some(next_value) = jump_forward(value, delta, &seen, &jumps[jump_index]) {
            jumps[jump_index].push((value, next_value));
            seen.insert(next_value);
            value = next_value;
            jump_index = 1 - jump_index;
        }
    }
    value
}
