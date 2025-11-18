use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> String {
    let data = read_to_string(&args.input_file).expect("Error opening input file");

    match args.part {
        1 => get_name_saturating(data),
        2 => get_name_circular(data),
        3 => get_name_swap(data),
        _ => unreachable!(),
    }
}

fn get_name_saturating(input: String) -> String {
    let mut line_it = input.lines();
    let names: Vec<&str> = line_it.next().unwrap().split(',').collect();
    line_it.next();
    let mut index: usize = 0;
    for instruction in line_it.next().unwrap().split(',') {
        let value = instruction[1..].parse().unwrap();
        if instruction.starts_with('L') {
            index = index.saturating_sub(value);
        } else {
            index = (index + value).min(names.len() - 1);
        }
    }
    names[index].into()
}

fn get_name_circular(input: String) -> String {
    let mut line_it = input.lines();
    let names: Vec<&str> = line_it.next().unwrap().split(',').collect();
    line_it.next();
    let mut index = 0;
    for instruction in line_it.next().unwrap().split(',') {
        let sign = if instruction.starts_with('L') { -1 } else { 1 };
        let value = instruction[1..].parse::<i32>().unwrap();
        index += sign * value;
    }
    let index = index.rem_euclid(names.len() as i32) as usize;
    names[index].into()
}

fn get_name_swap(input: String) -> String {
    let mut line_it = input.lines();
    let mut names: Vec<&str> = line_it.next().unwrap().split(',').collect();
    line_it.next();
    for instruction in line_it.next().unwrap().split(',') {
        let sign = if instruction.starts_with('L') { -1 } else { 1 };
        let value = instruction[1..].parse::<i32>().unwrap();
        let index = (sign * value).rem_euclid(names.len() as i32) as usize;
        names.swap(0, index);
    }
    names[0].into()
}
