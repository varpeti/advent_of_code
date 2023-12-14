use std::{collections::HashSet, usize};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
enum Pipe {
    PI, // | ║
    P_, // - ═
    PL, // L ╚
    PJ, // J ╝
    P7, // 7 ╗
    PF, // F ╔
    PG, // .
    PS, // S 0
}

impl Pipe {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Pipe::PI,
            '-' => Pipe::P_,
            'L' => Pipe::PL,
            'J' => Pipe::PJ,
            '7' => Pipe::P7,
            'F' => Pipe::PF,
            '.' => Pipe::PG,
            'S' => Pipe::PS,
            err => panic!("Invalid pipe: ({})", err),
        }
    }

    /// Just for fun!
    fn draw(&self) -> char {
        match self {
            Pipe::PI => '║',
            Pipe::P_ => '═',
            Pipe::PL => '╚',
            Pipe::PJ => '╝',
            Pipe::P7 => '╗',
            Pipe::PF => '╔',
            Pipe::PG => '.',
            Pipe::PS => '0',
        }
    }

    fn next(&self, from_to: &mut Direction, pos: &mut Pos) -> bool {
        match (self, from_to.clone()) {
            (Pipe::PI, Direction::Up) => {
                pos.y += 1;
                *from_to = Direction::Up
            }
            (Pipe::PI, Direction::Down) => {
                pos.y -= 1;
                *from_to = Direction::Down
            }
            (Pipe::P_, Direction::Right) => {
                pos.x -= 1;
                *from_to = Direction::Right
            }
            (Pipe::P_, Direction::Left) => {
                pos.x += 1;
                *from_to = Direction::Left
            }
            (Pipe::PL, Direction::Up) => {
                pos.x += 1;
                *from_to = Direction::Left
            }
            (Pipe::PL, Direction::Right) => {
                pos.y -= 1;
                *from_to = Direction::Down
            }
            (Pipe::PJ, Direction::Up) => {
                pos.x -= 1;
                *from_to = Direction::Right
            }
            (Pipe::PJ, Direction::Left) => {
                pos.y -= 1;
                *from_to = Direction::Down
            }
            (Pipe::P7, Direction::Down) => {
                pos.x -= 1;
                *from_to = Direction::Right
            }
            (Pipe::P7, Direction::Left) => {
                pos.y += 1;
                *from_to = Direction::Up
            }
            (Pipe::PF, Direction::Right) => {
                pos.y += 1;
                *from_to = Direction::Up
            }
            (Pipe::PF, Direction::Down) => {
                pos.x += 1;
                *from_to = Direction::Left
            }
            (Pipe::PS, _) => {}
            (pipe, direction) => {
                println!(
                    "The Pipe ({}) cannot be entered from Direction ({:?})! Current Pos ({:?}).",
                    pipe.draw(),
                    direction,
                    pos,
                );
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

/// Just for fun!
fn draw_map(map: &[Vec<Pipe>], poses: &[Pos]) {
    for (y, line) in map.iter().enumerate() {
        for (x, pipe) in line.iter().enumerate() {
            match poses.iter().contains(&Pos { x, y }) {
                true => print!("\x1b[91m{}\x1b[0m", pipe.draw()),
                false => print!("{}", pipe.draw()),
            }
        }
        println!();
    }
}

pub fn day10_a(input: &str) -> String {
    let mut cur_pos = Pos { x: 0, y: 0 };
    let mut cur_pipe = &Pipe::PS;
    let mut cur_dir = Direction::Up;
    let map: Vec<Vec<Pipe>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let pipe = Pipe::from_char(c);
                    if pipe == Pipe::PS {
                        cur_pos = Pos { x, y };
                    }
                    pipe
                })
                .collect()
        })
        .collect();
    let mut count = 1;
    for relative_start in [
        (1, 0, Direction::Up),
        (0, 1, Direction::Left),
        (-1, 0, Direction::Down),
        (0, -1, Direction::Right),
    ] {
        let y = relative_start.0 + cur_pos.y as i32;
        let x = relative_start.1 + cur_pos.x as i32;
        if y >= 0 && y < map.len() as i32 && x >= 0 && x < map[0].len() as i32 {
            cur_pos = Pos {
                x: x as usize,
                y: y as usize,
            };
            cur_dir = relative_start.2;
            cur_pipe = &map[cur_pos.y][cur_pos.x];
            break;
        }
    }
    while *cur_pipe != Pipe::PS {
        if !cur_pipe.next(&mut cur_dir, &mut cur_pos) {
            panic!();
        }
        cur_pipe = &map[cur_pos.y][cur_pos.x];
        count += 1;
    }
    format!("{}", count / 2)
}

pub fn day10_b(input: &str) -> String {
    let mut cur_pos = Pos { x: 0, y: 0 };
    let mut cur_pipe = &Pipe::PS;
    let mut cur_dir = Direction::Up;
    let map: Vec<Vec<Pipe>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let pipe = Pipe::from_char(c);
                    if pipe == Pipe::PS {
                        cur_pos = Pos { x, y };
                    }
                    pipe
                })
                .collect()
        })
        .collect();
    for relative_start in [
        (1, 0, Direction::Up),
        (0, 1, Direction::Left),
        (-1, 0, Direction::Down),
        (0, -1, Direction::Right),
    ] {
        let y = relative_start.0 + cur_pos.y as i32;
        let x = relative_start.1 + cur_pos.x as i32;
        if y > 0 && y < map.len() as i32 && x > 0 && x < map[0].len() as i32 {
            cur_pos = Pos {
                x: x as usize,
                y: y as usize,
            };
            cur_dir = relative_start.2;
            cur_pipe = &map[cur_pos.y][cur_pos.x];
            break;
        }
    }
    let mut the_loop = Vec::<Pos>::new();
    while *cur_pipe != Pipe::PS {
        the_loop.push(cur_pos.clone());
        if !cur_pipe.next(&mut cur_dir, &mut cur_pos) {
            panic!();
        }
        cur_pipe = &map[cur_pos.y][cur_pos.x];
    }
    the_loop.push(cur_pos.clone());
    draw_map(&map, &the_loop);
    let area = sholace_formula(&the_loop);

    format!("{}", area)
}

fn sholace_formula(the_loop: &[Pos]) -> usize {
    let mut trailing: i32 = 0;
    for (a, b) in the_loop.iter().tuple_windows() {
        trailing += (a.y as i32 + b.y as i32) * (a.x as i32 - b.x as i32);
    }
    let first = the_loop.first().expect("first");
    let last = the_loop.last().expect("last");
    trailing += (last.y as i32 + first.y as i32) * (last.x as i32 - first.x as i32);
    let area = trailing / 2;
    // Pick's Theorem
    (area - (the_loop.len() as i32 / 2) + 1) as usize
}
