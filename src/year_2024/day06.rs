use crate::args::RunArgs;

use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

type Tree<'a> = HashMap<&'a str, Vec<&'a str>>;

pub fn run(args: &RunArgs) -> String {
    let data = read_to_string(&args.input_file).expect("Error opening input file");

    let mut tree = Tree::new();
    let mut parents: HashMap<&str, &str> = HashMap::new();
    for line in data.lines() {
        let (br_name, children) = line.split_once(':').unwrap();
        let children: Vec<&str> = children
            .split(',')
            .filter(|child| *child != "ANT" && *child != "BUG")
            .collect();
        for child in &children {
            parents.insert(child, br_name);
        }
        tree.insert(br_name, children);
    }

    let unique_fruit_path = get_unique_fruit_path(&tree, &parents);

    match args.part {
        1 => unique_fruit_path.join(""),
        2 | 3 => unique_fruit_path
            .iter()
            .map(|br| br.get(..1).unwrap())
            .collect::<Vec<&str>>()
            .join(""),
        _ => unreachable!(),
    }
}

fn get_unique_fruit_path<'a>(tree: &Tree<'a>, parents: &HashMap<&'a str, &'a str>) -> Vec<&'a str> {
    let mut fruit_depths: HashMap<u16, Vec<&str>> = HashMap::new();
    let mut stack: VecDeque<(&str, u16)> = VecDeque::from([("RR", 0)]);
    while !stack.is_empty() {
        let node = stack.pop_front().unwrap();
        let branches = match tree.get(node.0) {
            None => continue,
            Some(br) => br,
        };
        for branch in branches {
            if *branch == "@" {
                fruit_depths.entry(node.1).or_default().push(node.0);
            } else {
                stack.push_back((branch, node.1 + 1));
            }
        }
    }
    let mut unique_fruit = fruit_depths
        .iter()
        .filter(|(_, nodes)| nodes.len() == 1)
        .map(|(_, nodes)| nodes[0])
        .next()
        .unwrap();
    let mut path = Vec::new();
    while unique_fruit != "RR" {
        path.push(unique_fruit);
        unique_fruit = parents.get(&unique_fruit).unwrap();
    }
    path.push("RR");
    path.reverse();
    path.push("@");
    path
}
