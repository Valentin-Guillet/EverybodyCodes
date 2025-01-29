use crate::args::RunArgs;

use std::{collections::HashMap, fmt::Display, fs::read_to_string};

struct Machine {
    steps: Vec<u32>,
    cats: Vec<Vec<[char; 3]>>,
}

impl Machine {
    fn new(input: &str) -> Self {
        let mut input_it = input.lines();
        let steps: Vec<u32> = input_it
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut cats = vec![Vec::new(); steps.len()];
        for line in input_it.skip(1) {
            let nb_cols = (line.len() + 1) / 4;
            for strip in 0..nb_cols {
                let cat = &line[4 * strip..4 * (strip + 1) - 1];
                if cat != "   " {
                    cats[strip].push(cat.chars().collect::<Vec<char>>().try_into().unwrap());
                }
            }
        }

        Self { steps, cats }
    }

    fn get_frequency(&self) -> u32 {
        let mut frequency = 1;
        for col in &self.cats {
            frequency = lcm(frequency, col.len() as u32);
        }
        frequency
    }

    fn get_coins(&self, nb_steps: u32) -> u64 {
        let state: Vec<u32> = self
            .steps
            .iter()
            .enumerate()
            .map(|(col, step)| (nb_steps * step) % self.cats[col].len() as u32)
            .collect();
        self.get_coins_from_state(&state)
    }

    fn get_coins_from_state(&self, state: &[u32]) -> u64 {
        let mut char_freq: HashMap<char, u64> = HashMap::new();
        for (col, &index) in state.iter().enumerate() {
            let cat = self.cats[col][index as usize];
            for c in [cat[0], cat[2]] {
                char_freq.entry(c).and_modify(|v| *v += 1).or_insert(1);
            }
        }
        char_freq
            .values()
            .map(|&count| if count < 3 { 0 } else { count - 2 })
            .sum()
    }
}

type Cache = HashMap<(Vec<u32>, u32), (u64, u64)>;

pub fn run(args: &RunArgs) -> Box<dyn Display> {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let machine = Machine::new(&data);

    match args.part {
        1 => Box::new(simulate_rolls(&machine, 100)),
        2 => Box::new(compute_coins(&machine, 202420242024)),
        3 => Box::new(compute_coin_range(&machine, 256)),
        _ => unreachable!(),
    }
}

fn lcm(mut a: u32, mut b: u32) -> u32 {
    let prod = a * b;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    prod / a
}

fn simulate_rolls(machine: &Machine, nb_steps: u32) -> String {
    let mut final_cats = String::new();
    for col in 0..machine.steps.len() {
        let cat_index = (nb_steps * machine.steps[col]) as usize % machine.cats[col].len();
        final_cats.push_str(&machine.cats[col][cat_index].iter().collect::<String>());
        if col < machine.steps.len() - 1 {
            final_cats.push(' ');
        }
    }
    final_cats
}

fn compute_coins(machine: &Machine, nb_steps: u64) -> u64 {
    let frequency = machine.get_frequency();
    let coins: Vec<u64> = (0..frequency)
        .map(|step| machine.get_coins(step + 1))
        .collect();

    let frequency = frequency as u64;
    let mut total_coins = (nb_steps / frequency) * coins.iter().sum::<u64>();
    total_coins += coins
        .iter()
        .take((nb_steps % frequency) as usize)
        .sum::<u64>();
    total_coins
}

fn compute_coin_range(machine: &Machine, nb_steps: u32) -> String {
    let mut cache = Cache::new();
    let (min, max) =
        get_min_max_coins(machine, nb_steps, &vec![0; machine.steps.len()], &mut cache);
    format!("{max} {min}")
}

fn get_next_states(machine: &Machine, state: &[u32]) -> Vec<Vec<u32>> {
    (-1..=1)
        .map(|diff| {
            state
                .iter()
                .enumerate()
                .map(|(i, &ind)| {
                    if diff == -1 {
                        (ind + machine.steps[i] + machine.cats[i].len() as u32 - 1)
                            % machine.cats[i].len() as u32
                    } else {
                        (ind + machine.steps[i] + diff as u32) % machine.cats[i].len() as u32
                    }
                })
                .collect()
        })
        .collect()
}

fn get_min_max_coins(
    machine: &Machine,
    nb_steps: u32,
    state: &[u32],
    cache: &mut Cache,
) -> (u64, u64) {
    if nb_steps == 0 {
        return (0, 0);
    }

    let cache_key = (state.to_vec(), nb_steps);
    if cache.contains_key(&cache_key) {
        return cache[&cache_key];
    }

    let mut min_coin = u64::MAX;
    let mut max_coin = 0;
    for next_state in get_next_states(machine, state) {
        let nb_coins = machine.get_coins_from_state(&next_state);
        let (min, max) = get_min_max_coins(machine, nb_steps - 1, &next_state, cache);
        if min + nb_coins < min_coin {
            min_coin = min + nb_coins;
        }
        if max + nb_coins > max_coin {
            max_coin = max + nb_coins;
        }
    }
    cache.insert(cache_key, (min_coin, max_coin));
    (min_coin, max_coin)
}
