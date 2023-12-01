pub fn day1_a(input: &String) -> String {
    format!(
        "{}",
        input
            .lines()
            .map(|line| {
                let mut nums = line.chars().filter(|c| c.is_numeric());
                let first = match nums.next() {
                    Some(c) => c.to_digit(10).expect("fd"),
                    None => 0,
                };
                let last = match nums.last() {
                    Some(c) => c.to_digit(10).expect("ld"),
                    None => first.to_owned(),
                };
                first * 10 + last
            })
            .sum::<u32>()
    )
}

const NUMS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

pub fn day1_b(input: &String) -> String {
    format!(
        "{}",
        input
            .lines()
            .map(|line| {
                let mut first = (0, usize::MAX);
                let mut last = (0, 0);
                for (i, num) in NUMS.iter().enumerate() {
                    if let Some(index) = line.find(num) {
                        if index < first.1 {
                            first.1 = index;
                            first.0 = i % 9 + 1;
                        }
                    }
                    if let Some(index) = line.rfind(num) {
                        if index >= last.1 {
                            last.1 = index;
                            last.0 = i % 9 + 1;
                        }
                    }
                }
                //println!("{} {} {}", line, first.0, last.0);
                first.0 * 10 + last.0
            })
            .sum::<usize>()
    )
}
