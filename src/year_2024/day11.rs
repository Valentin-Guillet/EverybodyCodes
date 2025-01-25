use crate::args::RunArgs;

use std::{collections::HashMap, fs::read_to_string};

type Rules<'a> = HashMap<&'a str, Vec<&'a str>>;
type Population<'a> = HashMap<&'a str, Vec<u64>>;

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");

    let rules: Rules = data
        .lines()
        .map(|line| {
            let (termite, next_gen) = line.split_once(':').unwrap();
            (termite, next_gen.split(',').collect::<Vec<_>>())
        })
        .collect();
    let mut population: Population = rules.keys().map(|termite| (*termite, Vec::new())).collect();

    match args.part {
        1 => get_population_count(&rules, &mut population, "A", 4),
        2 => get_population_count(&rules, &mut population, "Z", 10),
        3 => {
            let pop_counts: Vec<u64> = rules
                .keys()
                .map(|termite| get_population_count(&rules, &mut population, termite, 20))
                .collect();
            pop_counts.iter().max().unwrap() - pop_counts.iter().min().unwrap()
        }
        _ => unreachable!(),
    }
}

fn get_population_count<'a>(
    rules: &Rules<'a>,
    population: &mut Population<'a>,
    termite: &'a str,
    day: usize,
) -> u64 {
    if day == 0 {
        return 1;
    }

    let pop_entry = population.get_mut(&termite).unwrap();
    if pop_entry.len() < day + 1 {
        pop_entry.extend(vec![0; day + 1 - pop_entry.len()]);
    }
    if pop_entry[day] > 0 {
        return pop_entry[day];
    }

    let mut pop_count = 0;
    for child in rules.get(&termite).unwrap() {
        pop_count += get_population_count(rules, population, child, day - 1);
    }
    population.entry(termite).and_modify(|e| e[day] = pop_count);
    pop_count
}
