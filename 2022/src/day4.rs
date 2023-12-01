struct Range {
    s: u32,
    e: u32,
}

impl Range {
    fn new(s: u32, e: u32) -> Self {
        if s <= e {
            Range { s, e }
        } else {
            Range { s: e, e: s }
        }
    }
    fn is_fully_contain(&self, oth: &Self) -> bool {
        if self.s >= oth.s && self.e <= oth.e {
            true
        } else if self.s <= oth.s && self.e >= oth.e {
            true
        } else {
            false
        }
    }
    fn is_overlapping(&self, oth: &Self) -> bool {
        self.s <= oth.e && self.e >= oth.s
    }
}

fn elf_to_range(elf: &str) -> Range {
    let mut range_iter = elf
        .split("-")
        .map(|num| num.parse::<u32>().expect("parse err elf"));
    Range::new(
        range_iter.next().expect("range error 0"),
        range_iter.next().expect("range error 1"),
    )
}

pub fn day4_a(input: &String) -> String {
    let sum = input
        .lines()
        .map(|pair| {
            let mut pair_iter = pair.split(",").map(elf_to_range);
            let range_a = pair_iter.next().expect("pair error a");
            let range_b = pair_iter.next().expect("pair error b");
            range_a.is_fully_contain(&range_b) as u32
        })
        .sum::<u32>();
    format!("{}", sum)
}

pub fn day4_b(input: &String) -> String {
    let sum = input
        .lines()
        .map(|pair| {
            let mut pair_iter = pair.split(",").map(elf_to_range);
            let range_a = pair_iter.next().expect("pair error a");
            let range_b = pair_iter.next().expect("pair error b");
            range_a.is_overlapping(&range_b) as u32
        })
        .sum::<u32>();
    format!("{}", sum)
}
