use crate::args::RunArgs;

use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum BondAllowance {
    Strong,
    Weak,
    Replace,
}

struct Node<'a> {
    plug: &'a str,
    left_socket: &'a str,
    right_socket: &'a str,
    left_node: Option<usize>,
    right_node: Option<usize>,
}

fn read_from<'a>(input: &mut impl Iterator<Item = &'a str>, keyword: &str) -> &'a str {
    let word = input.next().unwrap();
    &word[keyword.len() + 1..]
}

impl<'a> Node<'a> {
    fn from(desc: &'a str) -> Self {
        let mut input = desc.split(", ");
        let _ = read_from(&mut input, "id");
        let plug = read_from(&mut input, "plug");
        let left_socket = read_from(&mut input, "leftSocket");
        let right_socket = read_from(&mut input, "rightSocket");
        Self {
            plug,
            left_socket,
            right_socket,
            left_node: None,
            right_node: None,
        }
    }
}

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let mut nodes: Vec<Node> = data.trim_end().lines().map(Node::from).collect();

    let bond_allowance = match args.part {
        1 => BondAllowance::Strong,
        2 => BondAllowance::Weak,
        3 => BondAllowance::Replace,
        _ => unreachable!(),
    };
    for mut node_id in 1..nodes.len() {
        while let Err(new_node_id) = attach(&mut nodes, 0, node_id, bond_allowance) {
            node_id = new_node_id;
        }
    }

    compute_checksum(&nodes)
}

fn plugs_match(socket: &str, plug: &str, bond_allowance: BondAllowance) -> bool {
    if matches!(bond_allowance, BondAllowance::Strong) {
        return socket == plug;
    }
    socket
        .split(' ')
        .zip(plug.split(' '))
        .any(|(elt1, elt2)| elt1 == elt2)
}

fn attach(
    nodes: &mut [Node],
    root_id: usize,
    mut child_id: usize,
    bond_allowance: BondAllowance,
) -> Result<(), usize> {
    if let Some(left_id) = nodes[root_id].left_node {
        if matches!(bond_allowance, BondAllowance::Replace)
            && nodes[root_id].left_socket == nodes[child_id].plug
            && nodes[left_id].plug != nodes[child_id].plug
        {
            // Found a strong bond that was taken by a weak bond: REPLACE
            nodes[root_id].left_node = Some(child_id);
            child_id = left_id;
        } else {
            // We recursively try to attach the child to the left branch
            // If we were able to attach, we stop
            // Else, we update the id of the branch to attach
            match attach(nodes, left_id, child_id, bond_allowance) {
                Ok(()) => return Ok(()),
                Err(new_child_id) => child_id = new_child_id,
            }
        }
    } else if plugs_match(
        nodes[root_id].left_socket,
        nodes[child_id].plug,
        bond_allowance,
    ) {
        nodes[root_id].left_node = Some(child_id);
        return Ok(());
    };

    // Same for right branch
    if let Some(right_id) = nodes[root_id].right_node {
        if matches!(bond_allowance, BondAllowance::Replace)
            && nodes[root_id].right_socket == nodes[child_id].plug
            && nodes[right_id].plug != nodes[child_id].plug
        {
            nodes[root_id].right_node = Some(child_id);
            child_id = right_id;
        } else {
            match attach(nodes, right_id, child_id, bond_allowance) {
                Ok(()) => return Ok(()),
                Err(new_child_id) => child_id = new_child_id,
            }
        }
    } else if plugs_match(
        nodes[root_id].right_socket,
        nodes[child_id].plug,
        bond_allowance,
    ) {
        nodes[root_id].right_node = Some(child_id);
        return Ok(());
    };

    // Couldn't attach node, or replaced one and must attach another one
    Err(child_id)
}

fn read_ids(nodes: &[Node], node_id: usize, return_ids: &mut Vec<usize>) {
    if let Some(left_id) = nodes[node_id].left_node {
        read_ids(nodes, left_id, return_ids);
    }
    return_ids.push(node_id);
    if let Some(right_id) = nodes[node_id].right_node {
        read_ids(nodes, right_id, return_ids);
    }
}

fn compute_checksum(nodes: &[Node]) -> u32 {
    let mut node_ids = Vec::new();
    read_ids(nodes, 0, &mut node_ids);
    node_ids
        .iter()
        .enumerate()
        .fold(0, |acc, (index, node_id)| acc + (index + 1) * (node_id + 1)) as u32
}
