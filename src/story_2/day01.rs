use crate::args::RunArgs;

use std::fs::read_to_string;

type Machine = Vec<Vec<bool>>;

pub fn run(args: &RunArgs) -> String {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let (machine, token_instructions) = parse_game(&data);

    match args.part {
        1 => token_instructions
            .iter()
            .enumerate()
            .map(|(i, instruction)| get_coins(&machine, instruction, i))
            .sum::<u32>()
            .to_string(),
        2 => token_instructions
            .iter()
            .map(|instruction| get_max_coins(&machine, instruction))
            .sum::<u32>()
            .to_string(),
        3 => solve_min_max_coins(&machine, &token_instructions),
        _ => unreachable!(),
    }
}

fn parse_game(input: &str) -> (Machine, Vec<&str>) {
    let mut machine = Machine::new();
    let mut line_it = input.lines();
    while let Some(line) = line_it.next()
        && !line.is_empty()
    {
        machine.push(line.chars().map(|c| c == '*').collect());
    }

    let tokens = line_it.collect();
    (machine, tokens)
}

fn get_coins(machine: &Machine, instruction: &str, input_slot: usize) -> u32 {
    let instruction: Vec<char> = instruction.chars().collect();
    let size = machine[0].len() - 1;

    let mut col = 2 * input_slot;
    let mut instr_index = 0;
    for row in 0..machine.len() {
        if !machine[row][col] {
            continue;
        }

        col = match (col, instruction[instr_index]) {
            (0, _) => 1,
            (col, _) if col == size => size - 1,
            (col, 'L') => col - 1,
            (col, 'R') => col + 1,
            (_, _) => unreachable!(),
        };
        instr_index += 1;
    }
    (2 * (col / 2 + 1)).saturating_sub(input_slot + 1) as u32
}

fn get_max_coins(machine: &Machine, instruction: &str) -> u32 {
    (0..(machine[0].len() + 1) / 2)
        .map(|i| get_coins(machine, instruction, i))
        .max()
        .unwrap()
}

fn solve_extreme_score_rec(
    token_map: &[Vec<u32>],
    sorted_indices: &[Vec<usize>],
    curr_indices: &mut Vec<usize>,
    curr_score: u32,
    max_diff: &mut u32,
    curr_diff: u32,
    should_max: bool,
) -> u32 {
    let n = curr_indices.len();

    if token_map.len() == n {
        let theoretical_extreme_score = (0..token_map.len())
            .map(|i| token_map[i][sorted_indices[i][0]])
            .sum::<u32>();
        *max_diff = curr_score.abs_diff(theoretical_extreme_score);
        return curr_score;
    }

    let token_extreme_value = token_map[n][sorted_indices[n][0]];

    let mut score = if should_max { 0 } else { u32::MAX };
    for new_index in 0..token_map[0].len() - 1 {
        // Already a token with this input slot
        if curr_indices
            .iter()
            .enumerate()
            .any(|(i, &ind)| sorted_indices[i][ind] == sorted_indices[n][new_index])
        {
            continue;
        }

        let token_value = token_map[n][sorted_indices[n][new_index]] as u32;
        let new_value = curr_score + token_value;
        let new_diff = curr_diff + token_value.abs_diff(token_extreme_value);

        if new_diff > *max_diff {
            break;
        }

        curr_indices.push(new_index);
        let extremum_score_rec = solve_extreme_score_rec(
            token_map,
            sorted_indices,
            curr_indices,
            new_value,
            max_diff,
            new_diff,
            should_max,
        );
        curr_indices.pop();

        if should_max {
            score = score.max(extremum_score_rec);
        } else {
            score = score.min(extremum_score_rec);
        }
    }
    score
}

fn solve_extreme_score(
    token_map: &[Vec<u32>],
    sorted_indices: &[Vec<usize>],
    should_max: bool,
) -> u32 {
    let mut max_diff = u32::MAX;
    solve_extreme_score_rec(
        token_map,
        sorted_indices,
        &mut Vec::with_capacity(token_map[0].len()),
        0,
        &mut max_diff,
        0,
        should_max,
    )
}

fn solve_min_max_coins(machine: &Machine, instructions: &[&str]) -> String {
    let mut token_map: Vec<Vec<u32>> = Vec::new();
    let mut sorted_indices: Vec<Vec<usize>> = Vec::new();
    let n = (machine[0].len() + 1) / 2;
    for instr in instructions {
        let token_outputs: Vec<u32> = (0..n)
            .map(|input_slot| get_coins(machine, instr, input_slot))
            .collect();

        let mut token_indices: Vec<usize> = (0..n).collect();
        token_indices.sort_unstable_by_key(|&index| token_outputs[index]);

        token_map.push(token_outputs);
        sorted_indices.push(token_indices);
    }

    let min_score = solve_extreme_score(&token_map, &sorted_indices, false);

    for token_indices in &mut sorted_indices {
        token_indices.reverse();
    }
    let max_score = solve_extreme_score(&token_map, &sorted_indices, true);

    format!("{} {}", min_score, max_score)
}
