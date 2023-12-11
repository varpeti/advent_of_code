use itertools::Itertools;

type History = Vec<i32>;

fn parse_lines(line: &str) -> History {
    line.split_ascii_whitespace()
        .map(|num| num.parse::<i32>().expect("num parse"))
        .collect::<Vec<i32>>()
}

fn get_delta(cur: &History) -> (History, bool) {
    let mut all_zero = true;
    (
        cur.iter()
            .tuple_windows()
            .map(|(a, b)| {
                let delta = b - a;
                all_zero &= delta == 0;
                delta
            })
            .collect(),
        all_zero,
    )
}

pub fn day9_a(input: &str) -> String {
    let mut sum = 0;
    for history in input.lines().map(parse_lines) {
        let mut last_of_deltas = vec![history[history.len() - 1]];
        let mut cur_delta = history;
        loop {
            let (next_delta, end) = get_delta(&cur_delta);
            if end {
                break;
            }
            last_of_deltas.push(next_delta[next_delta.len() - 1]);
            cur_delta = next_delta;
        }
        let mut value = 0;
        for cur_delta in last_of_deltas.iter().rev() {
            value += cur_delta;
        }
        sum += value;
    }
    format!("{}", sum)
}

pub fn day9_b(input: &str) -> String {
    let mut sum = 0;
    for history in input.lines().map(parse_lines) {
        let mut first_of_deltas = vec![history[0]];
        let mut cur_delta = history;
        loop {
            let (next_delta, end) = get_delta(&cur_delta);
            if end {
                break;
            }
            first_of_deltas.push(next_delta[0]);
            cur_delta = next_delta;
        }
        let mut value = 0;
        for cur_delta in first_of_deltas.iter().rev() {
            value = cur_delta - value;
        }
        sum += value;
    }
    format!("{}", sum)
}
