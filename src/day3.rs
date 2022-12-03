use std::collections::HashSet;

fn get_priority(item: &char) -> u32 {
    if item.is_lowercase() {(*item as u8 - 'a' as u8 + 1) as u32}
    else {(*item as u8 - 'A' as u8 + 27) as u32}
}

pub fn day3_a(input: &String) -> String {
    let mut sum = 0;
    for rucksack in input.lines() {
        let half_point = rucksack.len()/2;
        let first_compartment = rucksack.chars().take(half_point).collect::<HashSet::<_>>();
        let second_compartment = rucksack.chars().skip(half_point).collect::<HashSet<_>>();
        sum += first_compartment.intersection(&second_compartment).map(get_priority).sum::<u32>();
    }
    format!("{}", sum)
}


pub fn day3_b(input: &String) -> String {
    format!("b")
}
