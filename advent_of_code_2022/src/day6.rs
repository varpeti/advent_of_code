use std::collections::{HashSet, VecDeque};

fn naive_solution(size_of_marker: usize, input: &String) -> Option<String> {
    let mut chars = input.chars().enumerate();
    let mut previous = VecDeque::new();
    for _ in 0..(size_of_marker - 1) {
        previous.push_back(chars.next().expect("input length").1);
    }
    for (i, c) in chars {
        previous.push_back(c);
        // it is a quiet heavy but readable clean solution
        if previous.iter().collect::<HashSet<_>>().len() == size_of_marker {
            return Some(format!("{}", i + 1));
        }
        previous.pop_front();
    }
    None
}

pub fn day6_a(input: &String) -> String {
    match naive_solution(4, input) {
        Some(index) => format!("{}", index),
        None => format!("Not found!"),
    }
}

pub fn day6_b(input: &String) -> String {
    match naive_solution(14, input) {
        Some(index) => format!("{}", index),
        None => format!("Not found!"),
    }
}
