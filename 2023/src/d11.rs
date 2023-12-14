use std::usize;

#[derive(Debug, Clone, PartialEq)]
struct Galaxy {
    y: usize,
    x: usize,
}

const N: usize = 140;

fn _draw_galaxy(galaxies: &[Galaxy], h: usize, w: usize) {
    for y in 0..h {
        for x in 0..w {
            match galaxies.contains(&Galaxy { x, y }) {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
    println!()
}

fn manhattan_dst(a: &Galaxy, b: &Galaxy) -> usize {
    (usize::max(a.x, b.x) - usize::min(a.x, b.x)) + (usize::max(a.y, b.y) - usize::min(a.y, b.y))
}

pub fn day11_a(input: &str) -> String {
    // Parse
    let mut galaxies = Vec::<Galaxy>::new();
    let mut empty_row = [true; N];
    let mut empty_col = [true; N];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Galaxy {
                    y: y.to_owned(),
                    x: x.to_owned(),
                });
                empty_row[y] = false;
                empty_col[x] = false;
            }
        }
    }
    let mut h = galaxies.iter().map(|g| g.y).max().expect("h") + 1;
    let mut w = galaxies.iter().map(|g| g.x).max().expect("w") + 1;
    //draw_galaxy(&galaxies, h, w);

    // Expand
    let galaxies_old = galaxies.clone();
    for (y, is_empty) in empty_row.iter().enumerate().take(h) {
        if !is_empty {
            continue;
        }
        h += 1;
        for (i, old_galaxy) in galaxies_old.iter().enumerate() {
            if old_galaxy.y > y {
                galaxies[i].y += 1;
            }
        }
    }
    for (x, is_empty) in empty_col.iter().enumerate().take(w) {
        if !is_empty {
            continue;
        }
        w += 1;
        for (i, old_galaxy) in galaxies_old.iter().enumerate() {
            if old_galaxy.x > x {
                galaxies[i].x += 1;
            }
        }
    }
    //draw_galaxy(&galaxies, h, w);
    // Just brute force, but I considered: Quadtree, or just chunks
    let mut sum = 0;
    for (i, a) in galaxies.iter().enumerate() {
        for b in galaxies.iter().skip(i + 1) {
            if a != b {
                sum += manhattan_dst(a, b);
            }
        }
    }
    format!("{}", sum)
}

/// Same as the day11_a, but insted of 1 its 1000000-1
pub fn day11_b(input: &str) -> String {
    // Parse
    let mut galaxies = Vec::<Galaxy>::new();
    let mut empty_row = [true; N];
    let mut empty_col = [true; N];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Galaxy {
                    y: y.to_owned(),
                    x: x.to_owned(),
                });
                empty_row[y] = false;
                empty_col[x] = false;
            }
        }
    }
    let mut h = galaxies.iter().map(|g| g.y).max().expect("h") + 1;
    let mut w = galaxies.iter().map(|g| g.x).max().expect("w") + 1;
    //draw_galaxy(&galaxies, h, w);

    // Expand
    let galaxies_old = galaxies.clone();
    for (y, is_empty) in empty_row.iter().enumerate().take(h) {
        if !is_empty {
            continue;
        }
        h += 1;
        for (i, old_galaxy) in galaxies_old.iter().enumerate() {
            if old_galaxy.y > y {
                galaxies[i].y += 1000000 - 1;
            }
        }
    }
    for (x, is_empty) in empty_col.iter().enumerate().take(w) {
        if !is_empty {
            continue;
        }
        w += 1;
        for (i, old_galaxy) in galaxies_old.iter().enumerate() {
            if old_galaxy.x > x {
                galaxies[i].x += 1000000 - 1;
            }
        }
    }
    //draw_galaxy(&galaxies, h, w);
    // Just brute force, but I considered: Quadtree, or just chunks
    let mut sum = 0;
    for (i, a) in galaxies.iter().enumerate() {
        for b in galaxies.iter().skip(i + 1) {
            if a != b {
                sum += manhattan_dst(a, b);
            }
        }
    }
    format!("{}", sum)
}
