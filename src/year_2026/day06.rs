use crate::args::RunArgs;

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let recipes: HashMap<&str, Vec<&str>> =
        HashMap::from_iter(data.trim_end().lines().map(|line| {
            let (item, ingredients) = line.split_once(':').unwrap();
            (item, ingredients.split(',').collect())
        }));

    match args.part {
        1 => count_ingredients(&recipes),
        2 => {
            let rev_recipes = reverse_recipes(&recipes);
            count_ingredients(&rev_recipes)
        }
        3 => count_nulls_ingredients(&recipes),
        _ => unreachable!(),
    }
}

fn count_ingredients(recipes: &HashMap<&str, Vec<&str>>) -> u32 {
    let mut items = vec!["NULL"];
    let mut seen = HashSet::new();
    while let Some(item) = items.pop() {
        if seen.contains(item) {
            continue;
        }
        seen.insert(item);
        if let Some(ingredients) = recipes.get(item) {
            items.extend(
                ingredients
                    .iter()
                    .filter(|&ingredient| !seen.contains(ingredient)),
            );
        }
    }
    seen.len() as u32 - 1
}

fn reverse_recipes<'a>(recipes: &HashMap<&'a str, Vec<&'a str>>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut reversed: HashMap<&str, Vec<&str>> = HashMap::new();
    for (item, ingredients) in recipes {
        for ingredient in ingredients {
            reversed
                .entry(ingredient)
                .or_insert_with(Vec::new)
                .push(item);
        }
    }
    reversed
}

fn mark_ingredients<'a>(
    recipes: &'a HashMap<&str, Vec<&str>>,
    flags: &mut HashMap<&'a str, u32>,
    item: &'a str,
    flag: u32,
) {
    let mut items = vec![item];
    let mut seen = HashSet::new();
    while let Some(item) = items.pop() {
        if seen.contains(item) {
            continue;
        }
        seen.insert(item);
        flags.entry(item).and_modify(|f| *f |= flag).or_insert(flag);
        if let Some(ingredients) = recipes.get(item) {
            items.extend(
                ingredients
                    .iter()
                    .filter(|&ingredient| !seen.contains(ingredient)),
            );
        }
    }
}

fn count_nulls_ingredients(recipes: &HashMap<&str, Vec<&str>>) -> u32 {
    let rev_recipes = reverse_recipes(recipes);
    let mut flags: HashMap<&str, u32> = HashMap::new();
    for version in 1..=3 {
        let null_str = format!("NULLv{}", version);
        let null_item = rev_recipes.keys().find(|&&item| item == null_str).unwrap();
        mark_ingredients(&rev_recipes, &mut flags, &null_item, 1 << (version - 1));
    }
    flags
        .iter()
        .filter(|&(_, flag)| flag.count_ones() > 1)
        .count() as u32
}
