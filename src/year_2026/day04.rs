use crate::args::RunArgs;

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

struct Result {
    id: u16,
    quest: u8,
    part: u8,
    time: u32,
}

impl Result {
    fn new(input: &str) -> Self {
        let mut parts = input.split(' ');
        let id = parts.next().unwrap()[3..].parse::<u16>().unwrap();
        let quest = parts.next().unwrap()[6..].parse::<u8>().unwrap();
        let part = parts.next().unwrap()[5..].parse::<u8>().unwrap();
        let time = parts.next().unwrap()[5..].parse::<u32>().unwrap();
        Self {
            id,
            quest,
            part,
            time,
        }
    }
}

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let results: Vec<Result> = data.trim_end().lines().map(Result::new).collect();

    match args.part {
        1 | 2 => get_checksum(&results),
        3 => get_complete_checksum(&results),
        _ => unreachable!(),
    }
}

fn get_checksum(results: &[Result]) -> u32 {
    let mut frequencies: HashMap<u16, (u8, u32)> = HashMap::new();

    for result in results {
        frequencies
            .entry(result.id)
            .and_modify(|(nb_parts, time)| {
                *nb_parts += 1;
                *time += result.time;
            })
            .or_insert((1, result.time));
    }

    let mut times = frequencies.values().cloned().collect::<Vec<_>>();
    times.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    times
        .into_iter()
        .enumerate()
        .map(|(i, (_, time))| (i as u32 + 1) * time)
        .sum::<u32>()
}

fn get_complete_checksum(results: &[Result]) -> u32 {
    let mut max_id = 0;
    let mut max_quest = 0;
    for result in results {
        if result.id as usize > max_id {
            max_id = result.id as usize;
        }
        if result.quest as usize > max_quest {
            max_quest = result.quest as usize;
        }
    }

    let mut grouped_results: Vec<[Vec<&Result>; 3]> = Vec::new();
    for _ in 0..max_quest {
        grouped_results.push([Vec::new(), Vec::new(), Vec::new()]);
    }
    for result in results {
        grouped_results[result.quest as usize - 1][result.part as usize - 1].push(result);
    }

    let mut scores = vec![0; max_id];
    for quest in 0..max_quest {
        let mut quest_times = vec![0; max_id];
        for part in 0..3 {
            let mut sorted_indices: HashSet<usize> = HashSet::new();
            for result in &grouped_results[quest][part] {
                quest_times[result.id as usize - 1] += result.time;
                sorted_indices.insert(result.id as usize - 1);
            }
            let mut part_score = 50 * (part + 1);
            let mut sorted_indices: Vec<usize> = sorted_indices.into_iter().collect();
            sorted_indices.sort_unstable_by_key(|&i| quest_times[i]);
            let mut ind = 0;
            while ind < sorted_indices.len() {
                let mut end_ind = ind + 1;
                while end_ind < sorted_indices.len()
                    && quest_times[sorted_indices[ind]] == quest_times[sorted_indices[end_ind]]
                {
                    end_ind += 1;
                }
                for i in ind..end_ind {
                    scores[sorted_indices[i]] += part_score;
                }
                part_score = part_score.saturating_sub(end_ind - ind);
                ind = end_ind;
            }
        }
    }
    scores.sort_unstable_by(|a, b| b.cmp(a));

    let mut checksum = 0;
    let mut index = 0;
    while index < scores.len() {
        let mut equal_index = index + 1;
        while equal_index < scores.len() && scores[index] == scores[equal_index] {
            equal_index += 1;
        }
        for i in index..equal_index {
            checksum += ((index + 1) * scores[i]) as u32;
        }
        index = equal_index;
    }
    checksum
}
