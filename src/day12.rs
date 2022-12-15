use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    x: usize,
    y: usize,
    height: u8,
}

impl Node {
    fn distance(&self, other: &Node) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }
    fn new(x: usize, y: usize, height: u8) -> Self {
        Node { x, y, height }
    }
}

fn _print_map(map: &Vec<Vec<Node>>, start_node: &Node, end_node: &Node) {
    for line in map.iter() {
        for node in line.iter() {
            if node == start_node {
                print!("  S");
            } else if node == end_node {
                print!("  E");
            } else {
                print!("{:3}", node.height);
            }
        }
        println!("");
    }
    println!("");
}

fn _print_solution(map: &Vec<Vec<Node>>, solution: &(Vec<Node>, u32)) {
    let mut s = vec![vec![0; map[0].len()]; map.len()];
    for (n, node) in solution.0.iter().enumerate() {
        s[node.y][node.x] = n as u8;
    }
    for line in s.iter() {
        for n in line.iter() {
            print!("{:3}", n);
        }
        println!("");
    }
    println!("");
}

fn parse_input(input: &String) -> (Vec<Vec<Node>>, (usize, usize), (usize, usize)) {
    let mut start_node = (0, 0);
    let mut end_node = (0, 0);
    (
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start_node = (x, y);
                            Node::new(x, y, 0)
                        }
                        'E' => {
                            end_node = (x, y);
                            Node::new(x, y, 'z' as u8 - 'a' as u8)
                        }
                        c => Node::new(x, y, c as u8 - 'a' as u8),
                    })
                    .collect()
            })
            .collect(),
        start_node,
        end_node,
    )
}

fn get_neighs(map: &Vec<Vec<Node>>, node: &Node) -> Vec<(Node, u32)> {
    let mut neighs = Vec::new();
    for (x, y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if let Some(line) = map.get((node.y as i32 + y) as usize) {
            if let Some(neigh) = line.get((node.x as i32 + x) as usize) {
                if node.height + 1 >= neigh.height {
                    neighs.push((neigh.clone(), 1));
                }
            }
        }
    }
    neighs
}

pub fn day12_a(input: &String) -> String {
    let (map, start_node_coord, end_node_coord) = parse_input(input);
    let start_node = &map[start_node_coord.1][start_node_coord.0];
    let end_node = &map[end_node_coord.1][end_node_coord.0];
    // _print_map(&map, &start_node, &end_node);

    let solution = astar(
        start_node,
        |node| get_neighs(&map, node),
        |node| node.distance(end_node),
        |node| node == end_node,
    )
    .expect("No solution?");

    // _print_solution(&map, &solution);

    format!("{}", solution.1)
}

pub fn day12_b(input: &String) -> String {
    let (map, _start_node_coord, end_node_coord) = parse_input(input);
    let end_node = &map[end_node_coord.1][end_node_coord.0];
    let mut best_solution = (Vec::new(), u32::MAX);
    // Jeah... just prue bruteforce...
    for start_node in map.iter().flatten().filter(|node| node.height == 0) {
        let solution = astar(
            start_node,
            |node| get_neighs(&map, node),
            |node| node.distance(end_node),
            |node| node == end_node,
        );
        if let Some(solution) = solution {
            if best_solution.1 > solution.1 {
                best_solution = solution;
            }
        }
    }
    _print_solution(&map, &best_solution);

    format!("{}", best_solution.1)
}
