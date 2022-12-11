use std::collections::HashSet;

type Trees = Vec<Vec<u8>>;

fn parse_trees(input: &String) -> Trees {
    let mut trees = Trees::new();
    for input_row in input.lines() {
        let mut tree_row = Vec::new();
        for c in input_row.chars() {
            let tree_height = (c as u8) - ('0' as u8);
            tree_row.push(tree_height);
        }
        trees.push(tree_row);
    }
    trees
}

#[derive(Debug)]
enum Direction {
    Top,
    Left,
    Bottom,
    Right,
}

fn get_visible_coords(to: Direction, trees: &Trees) -> HashSet<(usize, usize)> {
    let xx = trees[0].len();
    let yy = trees.len();
    let mut coords = HashSet::new();
    match to {
        Direction::Top => {
            for x in 0..xx {
                let mut min_height = trees[0][x];
                coords.insert((0, x));
                for y in 1..yy {
                    if trees[y][x] <= min_height {
                        continue;
                    }
                    coords.insert((y, x));
                    min_height = trees[y][x];
                }
            }
        }
        Direction::Left => {
            for y in 0..yy {
                let mut min_height = trees[y][xx - 1];
                coords.insert((y, xx - 1));
                for x in 2..xx {
                    if trees[y][xx - x] <= min_height {
                        continue;
                    }
                    coords.insert((y, xx - x));
                    min_height = trees[y][xx - x];
                }
            }
        }
        Direction::Bottom => {
            for x in 0..xx {
                let mut min_height = trees[yy - 1][x];
                coords.insert((yy - 1, x));
                for y in 2..yy {
                    if trees[yy - y][x] <= min_height {
                        continue;
                    }
                    coords.insert((yy - y, x));
                    min_height = trees[yy - y][x];
                }
            }
        }
        Direction::Right => {
            for y in 0..yy {
                let mut min_height = trees[y][0];
                coords.insert((y, 0));
                for x in 1..xx {
                    if trees[y][x] <= min_height {
                        continue;
                    }
                    coords.insert((y, x));
                    min_height = trees[y][x];
                }
            }
        }
    };
    coords
}

pub fn day8_a(input: &String) -> String {
    let trees = parse_trees(input);
    let mut coords = HashSet::new();
    for to in [
        Direction::Top,
        Direction::Left,
        Direction::Bottom,
        Direction::Right,
    ] {
        let new_coords = get_visible_coords(to, &trees);
        coords = coords
            .union(&new_coords)
            .map(|&coord| coord)
            .collect::<HashSet<_>>();
    }
    // dgb
    for y in 0..trees.len() {
        for x in 0..trees[0].len() {
            if coords.contains(&(y, x)) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    format!("{}", coords.len())
}

pub fn day8_b(input: &String) -> String {
    drop(input);
    format!("b")
}
