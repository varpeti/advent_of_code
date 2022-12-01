use std::fs::{self, DirEntry};

mod day1;
use crate::day1::*;

const CALL: [(&dyn Fn(&String) -> String, &dyn Fn(&String) -> String); 1] = [(&day1_a, &day1_b)];

fn get_day(file: &DirEntry, delimeter: &str) -> Result<usize, String> {
    Ok(file
        .file_name()
        .to_str()
        .ok_or("file_name error")?
        .split(delimeter)
        .skip(1)
        .next()
        .ok_or("day split error")?
        .parse::<usize>()
        .map_err(|_| "parse error")?
        - 1)
}

fn main() {
    for file in fs::read_dir("inputs").expect("read_dir error") {
        let file = file.expect("file error");
        let lines = fs::read_to_string(file.path()).expect("read error");
        if let Ok(day) = get_day(&file, "day") {
            if let Some(calls) = CALL.get(day) {
                println!("{}", calls.0(&lines));
                println!("{}", calls.1(&lines));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_tests() {
        for file in fs::read_dir("tests").expect("read_dir error") {
            let file = file.expect("file error");
            let all_lines = fs::read_to_string(file.path()).expect("read error");
            let mut lines = all_lines.lines();
            let answer_a = lines.next().expect("answer_a error");
            let answer_b = lines.next().expect("answer_b errror");
            let input = lines.collect::<Vec<_>>().join("\n");
            match get_day(&file, "test") {
                Ok(day) => {
                    if let Some(calls) = CALL.get(day) {
                        assert_eq!(calls.0(&input), answer_a);
                        assert_eq!(calls.1(&input), answer_b);
                    }
                }
                Err(msg) => panic!("{}", msg),
            }
        }
    }
}
