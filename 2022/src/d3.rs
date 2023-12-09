use std::collections::HashSet;

fn get_priority(item: &char) -> u32 {
    if item.is_lowercase() {
        (*item as u8 - b'a' + 1) as u32
    } else {
        (*item as u8 - b'A' + 27) as u32
    }
}

pub fn day3_a(input: &str) -> String {
    let mut sum = 0;
    for rucksack in input.lines() {
        let half_point = rucksack.len() / 2;
        let first_compartment = rucksack.chars().take(half_point).collect::<HashSet<_>>();
        let second_compartment = rucksack.chars().skip(half_point).collect::<HashSet<_>>();
        sum += first_compartment
            .intersection(&second_compartment)
            .map(get_priority)
            .sum::<u32>();
    }
    format!("{}", sum)
}

pub fn day3_b(input: &str) -> String {
    let mut sum = 0;
    let mut rucksacks = input
        .lines()
        .map(|rucksack| rucksack.chars().collect::<HashSet<_>>());
    loop {
        // iter.next_chunk() is experimental, so I use this instead.
        let first = rucksacks.next();
        if first.is_none() {
            // guard for the unwraps, only None if no rucksack left
            break;
        }
        let mut common = first.unwrap();
        let group = [rucksacks.next().unwrap(), rucksacks.next().unwrap()];
        for i in 0..(group.len()) {
            common = common.intersection(&group[i]).copied().collect();
        }
        sum += common.iter().map(get_priority).sum::<u32>()
    }
    format!("{}", sum)
}
