use std::env;
use std::fs;

use import::*;
do_imports!();
do_function_call_array!();

fn answer(day: usize) {
    let lines = fs::read_to_string(format!("inputs/i{}", day)).expect("input read error");
    println!("answer:");
    if let Some(calls) = CALL.get(day - 1) {
        println!("\ta: {}", calls.0(&lines));
        println!("\tb: {}", calls.1(&lines));
    }
}

fn test(day: usize) {
    let all_lines = fs::read_to_string(format!("tests/t{}", day)).expect("test read error");
    let mut lines = all_lines.lines();
    let answer_a = lines.next().expect("answer_a error");
    let answer_b = lines.next().expect("answer_b error");
    let input = lines.collect::<Vec<_>>().join("\n");
    println!("test:");
    if let Some(calls) = CALL.get(day - 1) {
        let result_a = calls.0(&input);
        println!("\ta: {} == {}", result_a, answer_a);
        //assert_eq!(result_a, answer_a);
        let result_b = calls.1(&input);
        println!("\tb: {} == {}", result_b, answer_b);
        //assert_eq!(result_b, answer_b);
    }
}

fn main() {
    let mut args = env::args();
    let day = args
        .nth(1)
        .expect("usage: advent_of_code [day_number] <test|answer>")
        .parse::<usize>()
        .expect("parse error: day should be a number");

    match args.next() {
        Some(run_type) => match run_type.as_str() {
            "test" => test(day),
            "answer" => answer(day),
            err => panic!("Invalid argument! ({}) Use 'test' or 'answer'.", err),
        },
        None => {
            answer(day);
            test(day);
        }
    }
}
