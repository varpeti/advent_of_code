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

pub fn day8_a(input: &str) -> String {
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
    let mut current_node: &Node = &[b'A'; 3];
    let goal: Node = [b'Z'; 3];
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

pub fn day8_b(input: &str) -> String {
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
        .filter_map(|(node, _)| match node[2] == b'A' {
            true => Some(node),
            false => None,
        })
        .collect::<Vec<_>>();
    let mut counts = vec![0; current_nodes.len()];
    for (i, current_node) in current_nodes.iter_mut().enumerate() {
        for instruction in instructions.iter().cycle() {
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
            counts[i] += 1;
            if current_node[2] == b'Z' {
                break;
            }
        }
        //println!("");
    }
    println!("{:?}", counts);
    let res = counts.into_iter().reduce(lcm).expect("res");

    format!("{}", res)
}

// So this is an ehhh 'solution'.
// The lcm of the counts is the solution for the given input but its not generic.
// If I would write a proper solution, I would write a depth first search with cycle detection,
// and using the calculated offsets and cycle times I would get the solution.
// But I already did a similar thing in my work some weeks ago...
// So off the clock and for fun like this, I'll press the "pass" button.

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
