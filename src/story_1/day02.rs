use crate::args::RunArgs;

use std::fs::read_to_string;
use std::mem::swap;

struct Node {
    id: usize,
    rank: usize,
    symbol: char,
    left: Option<usize>,
    right: Option<usize>,
}

type Tree = Vec<Node>;

fn add_node(tree: &mut Tree, root_index: usize) {
    let node_index = tree.len() - 1;
    let node_rank = tree[node_index].rank;
    let root = &mut tree[root_index];
    if root.rank > node_rank {
        match root.left {
            Some(child_id) => add_node(tree, child_id),
            None => root.left = Some(node_index),
        }
    } else {
        match root.right {
            Some(child_id) => add_node(tree, child_id),
            None => root.right = Some(node_index),
        }
    }
}

fn find(tree: &Tree, id: usize, node_index: usize) -> Option<usize> {
    let node = &tree[node_index];
    if node.id == id {
        return Some(node_index);
    }

    node.left
        .and_then(|left_id| find(tree, id, left_id))
        .or_else(|| node.right.and_then(|right_id| find(tree, id, right_id)))
}

pub fn run(args: &RunArgs) -> String {
    let data = read_to_string(&args.input_file).expect("Error opening input file");

    let tree = build_tree(&data, args.part == 3);
    let left_msg = read_largest_level(&tree, 0);
    let right_msg = read_largest_level(&tree, 1);
    left_msg + &right_msg
}

fn parse_node_values(node_str: &str) -> (usize, char) {
    let ind1 = node_str.find('[').unwrap();
    let ind2 = node_str.find(',').unwrap();

    (
        node_str[ind1 + 1..ind2].parse().unwrap(),
        node_str.chars().nth(ind2 + 1).unwrap(),
    )
}

fn build_tree(input: &str, swap_full: bool) -> Tree {
    let mut tree = Tree::new();

    for instruction in input.trim().lines() {
        let words: Vec<&str> = instruction.split(' ').collect();

        match words[0] {
            "ADD" => {
                // Trick: we use even id for left tree and odd for right
                // so that it's easier to swap later
                let id = 2 * words[1][3..].parse::<usize>().unwrap();
                for (i, word) in words[2..=3].iter().enumerate() {
                    let (rank, symbol) = parse_node_values(word);
                    tree.push(Node {
                        id: id + i,
                        rank,
                        symbol,
                        left: None,
                        right: None,
                    });
                    // skip root of left and right trees
                    if id == 2 {
                        continue;
                    }
                    add_node(&mut tree, i);
                }
            }
            "SWAP" => {
                let id: usize = words[1].parse().unwrap();
                let node_id1 = find(&tree, 2 * id, 0)
                    .or_else(|| find(&tree, 2 * id, 1))
                    .unwrap();
                let node_id2 = find(&tree, 2 * id + 1, 0)
                    .or_else(|| find(&tree, 2 * id + 1, 1))
                    .unwrap();
                if swap_full {
                    tree.swap(node_id1, node_id2);
                } else {
                    let [node1, node2] = tree.get_disjoint_mut([node_id1, node_id2]).unwrap();
                    swap(&mut node1.rank, &mut node2.rank);
                    swap(&mut node1.symbol, &mut node2.symbol);
                }
            }
            _ => unreachable!(),
        }
    }
    tree
}

fn read_levels(tree: &Tree, node_index: usize, level_msgs: &mut Vec<String>, level: usize) {
    if level >= level_msgs.len() {
        level_msgs.push(String::new());
    }

    let node = &tree[node_index];
    level_msgs[level].push(node.symbol);
    if let Some(left_index) = node.left {
        read_levels(tree, left_index, level_msgs, level + 1);
    }
    if let Some(right_index) = node.right {
        read_levels(tree, right_index, level_msgs, level + 1);
    }
}

fn read_largest_level(tree: &Tree, root_index: usize) -> String {
    let mut level_msgs = Vec::new();
    read_levels(tree, root_index, &mut level_msgs, 0);
    level_msgs
        .into_iter()
        .rev()
        .max_by(|m1, m2| m1.len().cmp(&m2.len()))
        .unwrap()
}
