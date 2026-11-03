use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let votes: Vec<&str> = data.trim_end().lines().collect();

    votes
        .iter()
        .filter_map(|vote_repr| get_vote(vote_repr, args.part == 3))
        .sum()
}

fn get_minimum_representation_repr(
    values: &mut Vec<u64>,
    target: u64,
    base: &[u64],
    index: usize,
    max_values: &[u64],
) {
    if index == 0 {
        if target <= 9 {
            values.push(target);
        }
        return;
    }

    let mut min_factor = 0;
    while min_factor * base[index % base.len()].pow(index as u32) + max_values[index - 1] < target {
        min_factor += 1;
    }

    for factor in min_factor..=9 {
        let Some(remainder) =
            target.checked_sub(factor * base[index % base.len()].pow(index as u32))
        else {
            return;
        };

        get_minimum_representation_repr(values, remainder, base, index - 1, max_values);
        if !values.is_empty() {
            values.push(factor);
            return;
        }
    }
}

fn get_minimum_representation(vote: u64, base: &[u64]) -> Vec<u64> {
    let mut index = 0;
    let mut min_index = 0;
    let mut max_values = Vec::new();
    let mut max_value_cum = 0;
    for b in base {
        max_value_cum += 9 * b.pow(index as u32);
        if min_index == 0 && max_value_cum >= vote {
            min_index = index;
        }
        max_values.push(max_value_cum);
        index += 1;
    }

    let mut values = Vec::new();

    while values.is_empty() {
        get_minimum_representation_repr(&mut values, vote, base, min_index, &max_values);
        min_index += 1;
    }
    values
}

fn get_vote(vote_repr: &str, filter: bool) -> Option<u64> {
    let mut vote_it = vote_repr.chars();
    let mut base: Vec<u64> = vote_it
        .by_ref()
        .skip_while(|&c| c != '-')
        .skip(1)
        .take_while(|&c| c != ' ')
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    let mut value: Vec<u64> = vote_it.map(|c| c.to_digit(10).unwrap() as u64).collect();

    base.reverse();
    value.reverse();

    base = base.into_iter().cycle().take(value.len()).collect();

    let mut vote = 0;
    for i in 0..value.len() {
        vote += value[i] * base[i].pow(i as u32);
    }

    if filter && get_minimum_representation(vote, &base) != value {
        None
    } else {
        Some(vote)
    }
}
