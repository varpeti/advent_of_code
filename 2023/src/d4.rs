pub fn day4_a(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let mut game = 0;
        let mut wining = [false; 99];
        let mut s = line.split(" | ");
        for num_str in s.next().expect("winings").split(' ') {
            if let Ok(num) = num_str.parse::<u8>() {
                wining[(num - 1) as usize] = true
            }
        }
        for num_str in s.next().expect("my_nums").split(' ') {
            if let Ok(num) = num_str.parse::<u8>() {
                if wining[(num - 1) as usize] {
                    match game {
                        0 => game = 1,
                        _ => game *= 2,
                    };
                }
            }
        }
        sum += game;
    }
    format!("{}", sum)
}

#[derive(Debug)]
struct Card {
    wining: [bool; 99],
    nums: Vec<u8>,
}

pub fn day4_b(input: &str) -> String {
    let mut sum = 0;
    let mut cards = Vec::<Card>::new();
    let mut counts = Vec::<usize>::new();
    for line in input.lines() {
        let mut wining = [false; 99];
        let mut s = line.split(" | ");
        for num_str in s.next().expect("winings").split(' ') {
            if let Ok(num) = num_str.parse::<u8>() {
                wining[(num - 1) as usize] = true
            }
        }
        let card = Card {
            wining,
            nums: s
                .next()
                .expect("nums")
                .split(' ')
                .filter_map(|num_str| match num_str.parse::<u8>() {
                    Ok(num) => Some(num - 1),
                    Err(_) => None,
                })
                .collect(),
        };
        cards.push(card);
        counts.push(1);
    }
    for (i, card) in cards.iter().enumerate() {
        let mut matching = 0;
        let my_count = counts.get(i).expect("count self").to_owned();
        for num in card.nums.iter() {
            if card.wining[*num as usize] {
                matching += 1;
            }
        }
        for j in 1..(matching + 1) {
            let count = counts.get_mut(i + j).expect("count");
            *count += my_count;
        }
        sum += my_count;
    }

    format!("{}", sum)
}
