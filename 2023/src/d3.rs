use std::usize;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Part {
    len: usize,
    ty: PartType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PartType {
    Number(u32),
    Symbol,
}

const N: usize = 141;
const MAX_LEN: i32 = 3;

pub fn day3_a(input: &String) -> String {
    let mut parts = [[None; N]; N];
    let parts_re = Regex::new(r"(\d+|[^.\d])").expect("parts_re");
    for (y, line) in input.lines().enumerate() {
        for c in parts_re.captures_iter(line) {
            if let Some(m) = c.get(0) {
                let x = m.start();
                let len = m.end() - x;
                let part_str = m.as_str();
                println!("A {} {} {} {}", x, y, len, part_str);
                parts[y][x] = match part_str.parse::<u32>() {
                    Ok(num) => Some(Part {
                        len,
                        ty: PartType::Number(num),
                    }),
                    Err(_) => Some(Part {
                        len,
                        ty: PartType::Symbol,
                    }),
                };
            }
        }
    }
    let mut sum = 0;
    for (y, line) in parts.iter().enumerate() {
        for (x, a_part) in line.iter().enumerate() {
            if let Some(a_part) = a_part {
                if let PartType::Number(num) = a_part.ty {
                    for iy in -1..=1 {
                        let sy = y as i32 + iy;
                        if sy < 0 || sy > (N as i32) {
                            continue;
                        }
                        for ix in -1..=(a_part.len as i32) {
                            let sx = x as i32 + ix;
                            if sx < 0 || sx > (N as i32) {
                                continue;
                            }
                            if let Some(b_part) = parts[sy as usize][sx as usize] {
                                if PartType::Symbol == b_part.ty {
                                    sum += num;
                                    println!("B {:?} {:?} {}", a_part, b_part, sum);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    format!("{}", sum)
}

pub fn day3_b(input: &String) -> String {
    let mut parts = [[None; N]; N];
    let parts_re = Regex::new(r"(\d+|[*])").expect("parts_re");
    for (y, line) in input.lines().enumerate() {
        for c in parts_re.captures_iter(line) {
            if let Some(m) = c.get(0) {
                let x = m.start();
                let len = m.end() - x;
                let part_str = m.as_str();
                println!("A {} {} {} {}", x, y, len, part_str);
                parts[y][x] = match part_str.parse::<u32>() {
                    Ok(num) => Some(Part {
                        len,
                        ty: PartType::Number(num),
                    }),
                    Err(_) => Some(Part {
                        len,
                        ty: PartType::Symbol,
                    }),
                };
            }
        }
    }
    let mut sum = 0;
    for (y, line) in parts.iter().enumerate() {
        for (x, a_part) in line.iter().enumerate() {
            if let Some(a_part) = a_part {
                if PartType::Symbol == a_part.ty {
                    let mut count = 0;
                    let mut gear_ratio = 1;
                    for iy in -1..=1 {
                        let sy = y as i32 + iy;
                        if sy < 0 || sy > (N as i32) {
                            continue;
                        }
                        for ix in -MAX_LEN..=1 {
                            let sx = x as i32 + ix;
                            if sx < 0 || sx > (N as i32) {
                                continue;
                            }
                            if let Some(b_part) = parts[sy as usize][sx as usize] {
                                if let PartType::Number(num) = b_part.ty {
                                    if ix + b_part.len as i32 > -1 {
                                        count += 1;
                                        gear_ratio *= num;
                                        println!(
                                            "B ({} {}) ({} {}), {}, {} {} {}",
                                            x, y, sx, sy, num, count, gear_ratio, sum
                                        );
                                    }
                                }
                            }
                        }
                    }
                    if count == 2 {
                        sum += gear_ratio;
                    }
                    println!("")
                }
            }
        }
    }
    format!("{}", sum)
}
