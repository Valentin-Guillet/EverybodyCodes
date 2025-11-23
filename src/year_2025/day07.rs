use crate::args::RunArgs;

use std::{collections::HashMap, fs::read_to_string};

type Rules = HashMap<char, Vec<char>>;
type Memory = HashMap<(char, usize), u32>;

pub fn run(args: &RunArgs) -> String {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let (names, rules) = read_data(&data);

    match args.part {
        1 => find_valid_name(&names, &rules),
        2 => find_valid_names_sum(&names, &rules).to_string(),
        3 => count_valid_names(&names, &rules).to_string(),
        _ => unreachable!(),
    }
}

fn read_data(input: &str) -> (Vec<Vec<char>>, Rules) {
    let mut lines = input.lines();
    let names: Vec<Vec<char>> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|name| name.chars().collect())
        .collect();
    _ = lines.next();

    let mut rules = Rules::new();
    for line in lines {
        let ante = line.chars().next().unwrap();
        let successors = line[4..]
            .split(',')
            .map(|s| s.chars().next().unwrap())
            .collect();
        rules.insert(ante, successors);
    }

    (names, rules)
}

fn is_name_valid(name: &[char], rules: &Rules) -> bool {
    for i in 0..name.len() - 1 {
        if rules
            .get(&name[i])
            .is_some_and(|post| !post.contains(&name[i + 1]))
        {
            return false;
        }
    }
    true
}

fn find_valid_name(names: &[Vec<char>], rules: &Rules) -> String {
    String::from_iter(
        names
            .iter()
            .filter(|name| is_name_valid(name, rules))
            .next()
            .unwrap(),
    )
}

fn find_valid_names_sum(names: &[Vec<char>], rules: &Rules) -> usize {
    let mut sum = 0;
    for (i, name) in names.iter().enumerate() {
        if is_name_valid(name, rules) {
            sum += i + 1;
        }
    }
    sum
}

fn _count_valid_names(seed: (char, usize), rules: &Rules, memory: &mut Memory) -> u32 {
    if let Some(&count) = memory.get(&seed) {
        return count;
    }

    if seed.1 > 11 {
        return 0;
    }

    let mut count = if seed.1 >= 7 { 1 } else { 0 };
    let Some(after) = rules.get(&seed.0) else {
        return count;
    };

    for &c in after {
        count += _count_valid_names((c, seed.1 + 1), rules, memory);
    }
    memory.insert(seed, count);
    count
}

fn count_valid_names(names: &[Vec<char>], rules: &Rules) -> u32 {
    let mut sum = 0;
    let mut memory = Memory::new();
    for name in names {
        if !is_name_valid(name, rules) {
            continue;
        }

        // Skip curr name is another name is its prefix
        let mut has_prefix = false;
        for other in names {
            if other.len() >= name.len() {
                continue;
            }

            if (0..other.len()).all(|i| other[i] == name[i]) {
                has_prefix = true;
                break;
            }
        }
        if has_prefix {
            continue;
        }

        let seed = (name[name.len() - 1], name.len());
        sum += _count_valid_names(seed, rules, &mut memory);
    }
    sum
}
