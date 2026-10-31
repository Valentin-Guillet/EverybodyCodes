use crate::args::RunArgs;

use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Clone, Copy)]
struct Comp {
    inf: u8,
    sup: u8,
}

impl Comp {
    fn new(input: &str) -> Self {
        let input = input.chars().collect::<Vec<_>>();
        let inf_index = if input[1] == '<' { 0 } else { 2 };
        Self {
            inf: input[inf_index].to_digit(10).unwrap() as u8,
            sup: input[2 - inf_index].to_digit(10).unwrap() as u8,
        }
    }
}

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let comparisons = data
        .trim_end()
        .lines()
        .map(|line| line.split(' ').map(|s| Comp::new(s)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    comparisons.iter().map(|line| get_order(line)).sum()
}

fn compute_inferior_sets_rec(
    inferiors: &[HashSet<u8>],
    inferior_sets: &mut Vec<HashSet<u8>>,
    n: usize,
) {
    if !inferior_sets[n].is_empty() {
        return;
    }

    let mut inferior_set: HashSet<u8> = HashSet::new();
    for &inf in &inferiors[n] {
        compute_inferior_sets_rec(inferiors, inferior_sets, inf as usize - 1);
        inferior_set.extend(&inferior_sets[inf as usize - 1]);
        inferior_set.insert(inf - 1);
    }
    inferior_sets[n] = inferior_set;
}

fn get_order(comparisons: &[Comp]) -> u64 {
    let n = comparisons
        .iter()
        .flat_map(|comp| [comp.inf, comp.sup])
        .max()
        .unwrap() as usize;
    let mut inferiors: Vec<HashSet<u8>> = vec![HashSet::new(); n];
    for &Comp { inf, sup } in comparisons {
        inferiors[sup as usize - 1].insert(inf);
    }

    let mut inferior_sets: Vec<HashSet<u8>> = vec![HashSet::new(); n];
    for i in 0..n {
        compute_inferior_sets_rec(&inferiors, &mut inferior_sets, i);
    }

    let mut order = vec![0; n];
    for i in 0..n {
        order[n - 1 - inferior_sets[i].len()] = i as u64;
    }
    order.into_iter().fold(0, |acc, elt| 10 * acc + elt + 1)
}
