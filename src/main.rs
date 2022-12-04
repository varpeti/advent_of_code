use std::env;
use std::fs;

// I'm pretty sure there is a better way doing this...
// But it was a great oportunity to use vim macros!
mod day1;
use crate::day1::*;
mod day2;
use crate::day2::*;
mod day3;
use crate::day3::*;
mod day4;
use crate::day4::*;
mod day5;
use crate::day5::*;
mod day6;
use crate::day6::*;
mod day7;
use crate::day7::*;
mod day8;
use crate::day8::*;
mod day9;
use crate::day9::*;
mod day10;
use crate::day10::*;
mod day11;
use crate::day11::*;
mod day12;
use crate::day12::*;
mod day13;
use crate::day13::*;
mod day14;
use crate::day14::*;
mod day15;
use crate::day15::*;
mod day16;
use crate::day16::*;
mod day17;
use crate::day17::*;
mod day18;
use crate::day18::*;
mod day19;
use crate::day19::*;
mod day20;
use crate::day20::*;
mod day21;
use crate::day21::*;
mod day22;
use crate::day22::*;
mod day23;
use crate::day23::*;
mod day24;
use crate::day24::*;
mod day25;
use crate::day25::*;

const CALL: [(&dyn Fn(&String) -> String, &dyn Fn(&String) -> String); 25] = [
    (&day1_a, &day1_b),
    (&day2_a, &day2_b),
    (&day3_a, &day3_b),
    (&day4_a, &day4_b),
    (&day5_a, &day5_b),
    (&day6_a, &day6_b),
    (&day7_a, &day7_b),
    (&day8_a, &day8_b),
    (&day9_a, &day9_b),
    (&day10_a, &day10_b),
    (&day11_a, &day11_b),
    (&day12_a, &day12_b),
    (&day13_a, &day13_b),
    (&day14_a, &day14_b),
    (&day15_a, &day15_b),
    (&day16_a, &day16_b),
    (&day17_a, &day17_b),
    (&day18_a, &day18_b),
    (&day19_a, &day19_b),
    (&day20_a, &day20_b),
    (&day21_a, &day21_b),
    (&day22_a, &day22_b),
    (&day23_a, &day23_b),
    (&day24_a, &day24_b),
    (&day25_a, &day25_b),
];

fn answer(day: usize) {
    let lines = fs::read_to_string(format!("inputs/day{}", day)).expect("input read error");
    println!("answer:");
    if let Some(calls) = CALL.get(day - 1) {
        println!("\ta: {}", calls.0(&lines));
        println!("\tb: {}", calls.1(&lines));
    }
}

fn test(day: usize) {
    let all_lines = fs::read_to_string(format!("tests/test{}", day)).expect("test read error");
    let mut lines = all_lines.lines();
    let answer_a = lines.next().expect("answer_a error");
    let answer_b = lines.next().expect("answer_b error");
    let input = lines.collect::<Vec<_>>().join("\n");
    println!("test:");
    if let Some(calls) = CALL.get(day - 1) {
        let result_a = calls.0(&input);
        println!("\ta: {} == {}", result_a, answer_a);
        assert_eq!(result_a, answer_a);
        let result_b = calls.1(&input);
        println!("\tb: {} == {}", result_b, answer_b);
        assert_eq!(result_b, answer_b);
    }
}

fn main() {
    let day = env::args()
        .skip(1)
        .next()
        .expect("usage: advent_of_code day_number")
        .parse::<usize>()
        .expect("parse error: day should be a number");
    answer(day);
    test(day);
}
