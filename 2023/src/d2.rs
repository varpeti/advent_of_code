use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
};

use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
enum Color {
    Blue,
    Red,
    Green,
}

impl Color {
    fn from_str(s: &str) -> Self {
        match s {
            "blue" => Color::Blue,
            "red" => Color::Red,
            "green" => Color::Green,
            err => panic!("color {}", err),
        }
    }
}

// Wow its a mess, I think it is pretty unreadable... TODO refactor...
//                                  this probably never happen ^

pub fn day2_a(input: &str) -> String {
    format!(
        "{}",
        input
            .lines()
            .filter_map(|game| {
                let mut s = game.split(": ");
                let gid = s
                    .next()
                    .expect("g")
                    .split(' ')
                    .last()
                    .expect("gid")
                    .parse::<u32>()
                    .expect("pgid");
                let draws = s
                    .next()
                    .expect("draws")
                    .split("; ")
                    .map(|dices| {
                        let dices = dices
                            .split(", ")
                            .map(|dice| {
                                let mut dice = dice.split(' ');
                                let num = dice.next().expect("num").parse::<u32>().expect("pnum");
                                let color = Color::from_str(dice.next().expect("color"));
                                match color {
                                    Color::Blue => num <= 14,
                                    Color::Red => num <= 12,
                                    Color::Green => num <= 13,
                                }
                            })
                            .counts();
                        !dices.contains_key(&false)
                    })
                    .counts();
                match draws.contains_key(&false) {
                    true => None,
                    false => Some(gid),
                }
            })
            .sum::<u32>()
    )
}

pub fn day2_b(input: &str) -> String {
    format!(
        "{}",
        input
            .lines()
            .map(|game| {
                let draws = game
                    .split(": ")
                    .last()
                    .expect("draws")
                    .split("; ")
                    .map(|dices| {
                        let dices = dices
                            .split(", ")
                            .map(|dice| {
                                let mut dice = dice.split(' ');
                                let num = dice.next().expect("num").parse::<u32>().expect("pnum");
                                let color = Color::from_str(dice.next().expect("color"));
                                (color, num)
                            })
                            .fold(HashMap::<Color, u32>::new(), |acc, dice| {
                                let mut acc = acc;
                                match acc.entry(dice.0) {
                                    Entry::Occupied(oc) => {
                                        let a = oc.into_mut();
                                        if *a < dice.1 {
                                            *a = dice.1;
                                        }
                                    }
                                    Entry::Vacant(va) => {
                                        va.insert(dice.1);
                                    }
                                };
                                acc
                            });
                        dices
                    })
                    .fold(HashMap::<Color, u32>::new(), |acc, draws| {
                        let mut acc = acc;
                        for draw in draws.into_iter() {
                            match acc.entry(draw.0) {
                                Entry::Occupied(oc) => {
                                    let a = oc.into_mut();
                                    if *a < draw.1 {
                                        *a = draw.1;
                                    }
                                }
                                Entry::Vacant(va) => {
                                    va.insert(draw.1);
                                }
                            }
                        }
                        acc
                    });

                draws.into_values().product::<u32>()
            })
            .sum::<u32>()
    )
}
