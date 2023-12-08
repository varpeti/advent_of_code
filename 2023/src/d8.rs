use std::collections::HashMap;

type Node = [u8; 3];

#[derive(Debug, Clone)]
struct Neigh {
    left: Node,
    right: Node,
}

type Nodes = HashMap<Node, Neigh>;

fn parse_line(line: &str) -> (Node, Neigh) {
    let mut line = line.split(" = (");
    let name: Node = line
        .next()
        .expect("name")
        .as_bytes()
        .try_into()
        .expect("name len");
    let mut neighs = line.next().expect("neighs").split(", ");
    let left: Node = neighs
        .next()
        .expect("left")
        .as_bytes()
        .try_into()
        .expect("left len");
    let right: Node = neighs
        .next()
        .expect("right")
        .replace(')', "")
        .as_bytes()
        .try_into()
        .expect("right");
    (name, Neigh { left, right })
}

pub fn day8_a(input: &String) -> String {
    let mut input = input.split("\n\n");
    let instructions = input
        .next()
        .expect("instructions")
        .chars()
        .collect::<Vec<char>>();
    let nodes = input
        .next()
        .expect("map")
        .lines()
        .map(parse_line)
        .collect::<Nodes>();
    let mut current_node: &Node = &['A' as u8; 3];
    let goal: Node = ['Z' as u8; 3];
    let mut count: u64 = 0;
    while *current_node != goal {
        for instruction in instructions.iter() {
            current_node = match instruction {
                'R' => &nodes.get(current_node).expect("rneigh").right,
                'L' => &nodes.get(current_node).expect("lneigh").left,
                err => panic!("Invalid instruction: ({})", err),
            };
            count += 1;
        }
    }
    format!("{}", count)
}

fn all_z(nodes: &Vec<&Node>) -> bool {
    for node in nodes.iter() {
        if node[2] != 'Z' as u8 {
            return false;
        }
    }
    true
}

pub fn day8_b(input: &String) -> String {
    let mut input = input.split("\n\n");
    let instructions = input
        .next()
        .expect("instructions")
        .chars()
        .collect::<Vec<char>>();
    let nodes = input
        .next()
        .expect("map")
        .lines()
        .map(parse_line)
        .collect::<Nodes>();
    let mut current_nodes = nodes
        .iter()
        .filter_map(|(node, _)| match node[2] == 'A' as u8 {
            true => Some(node),
            false => None,
        })
        .collect::<Vec<_>>();
    // I think its needs some loop detection... and calculate the max-step from each loop's cycle+offset
    let mut count: u64 = 0;
    while !all_z(&current_nodes) {
        for instruction in instructions.iter() {
            for current_node in current_nodes.iter_mut() {
                // println!(
                //     "{:?}, {}, {:?}",
                //     current_node,
                //     instruction,
                //     nodes.get(current_node)
                // );
                *current_node = match instruction {
                    'R' => &nodes.get(current_node.as_ref()).expect("rneigh").right,
                    'L' => &nodes.get(current_node.as_ref()).expect("lneigh").left,
                    err => panic!("Invalid instruction: ({})", err),
                };
            }
            //println!("");
            count += 1;
        }
    }
    format!("{}", count)
}
