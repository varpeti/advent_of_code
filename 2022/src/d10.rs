use std::str::Lines;

const CYCLE_PERIOD: i32 = 40;

pub fn day10_a(input: &str) -> String {
    let mut sum: i128 = 0;
    let mut x = 1;
    let mut prev_x = 1;
    let mut cycle = 1;
    let mut last_cycle = -20;
    for line in input.lines() {
        match line {
            _ if line.starts_with("addx") => {
                prev_x = x;
                x += line
                    .split(' ')
                    .last()
                    .expect("add")
                    .parse::<i32>()
                    .expect("parse add");
                cycle += 2;
            }
            _ if line.starts_with("noop") => {
                cycle += 1;
            }
            err => panic!("Invaild input! {}", err),
        }
        if cycle == last_cycle + CYCLE_PERIOD {
            last_cycle += CYCLE_PERIOD;
            sum += (x * last_cycle) as i128;
            continue;
        }
        if cycle > last_cycle + CYCLE_PERIOD {
            last_cycle += CYCLE_PERIOD;
            sum += (prev_x * last_cycle) as i128;
        }
    }
    format!("{}", sum)
}

#[derive(Debug)]
struct Task {
    remaining_cycle: usize,
    dx: i32,
}

fn get_next_task(lines: &mut Lines) -> Option<Task> {
    match lines.next() {
        Some(add) if add.starts_with("addx") => {
            let dx = add
                .split(' ')
                .last()
                .expect("add")
                .parse::<i32>()
                .expect("parse add");
            Some(Task {
                remaining_cycle: 2,
                dx,
            })
        }
        Some(nop) if nop.starts_with("noop") => Some(Task {
            remaining_cycle: 1,
            dx: 0,
        }),
        Some(err) => panic!("Invalid input! {}", err),
        None => None,
    }
}

pub fn day10_b(input: &str) -> String {
    let mut reg_x = 1;
    let mut lines = input.lines();
    let mut current_task = get_next_task(&mut lines);
    for _y in 0..6 {
        for x in 0..40 {
            if (reg_x - 1..reg_x + 2).contains(&x) {
                print!("#");
            } else {
                print!(".");
            }
            if let Some(task) = &mut current_task {
                task.remaining_cycle -= 1;
                if task.remaining_cycle == 0 {
                    reg_x += task.dx;
                    current_task = get_next_task(&mut lines);
                }
            }
        }
        println!();
    }
    "PRINTED_ANSWER".to_string()
}
