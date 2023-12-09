pub fn day1_a(input: &str) -> String {
    let max_calories = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|calories| calories.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .max()
        .expect("max error");
    format!("{}", max_calories)
}

pub fn day1_b(input: &str) -> String {
    let calories_per_elf = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|calories| calories.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    let mut max_calories = [0; 3];
    for calories in calories_per_elf {
        for j in 0..3 {
            if max_calories[j] < calories {
                for i in j..2 {
                    max_calories[2 - i] = max_calories[1 - i]
                }
                max_calories[j] = calories;
                break;
            }
        }
    }

    format!("{}", max_calories.iter().sum::<u32>())
}
