use std::collections::HashMap;

fn calculate_score(input: &String, move_map: &HashMap<&str, HashMap<&str, u32>>) -> u32 {
    input
        .split("\n")
        .map(|line| {
            if line.is_empty() {
                return 0;
            }
            let mut moves = line.split(" ");
            let oponent_move = moves.next().expect("oponent move error");
            let my_move = moves.next().expect("my move error");
            *move_map
                .get(oponent_move)
                .expect("invalid oponenet move")
                .get(my_move)
                .expect("invalid my move")
        })
        .sum::<u32>()
}

pub fn day2_a(input: &String) -> String {
    let mut move_map = HashMap::new();
    move_map.insert(
        "A",
        HashMap::from([("X", 1 + 3), ("Y", 2 + 6), ("Z", 3 + 0)]),
    );
    move_map.insert(
        "B",
        HashMap::from([("X", 1 + 0), ("Y", 2 + 3), ("Z", 3 + 6)]),
    );
    move_map.insert(
        "C",
        HashMap::from([("X", 1 + 6), ("Y", 2 + 0), ("Z", 3 + 3)]),
    );
    format!("{}", calculate_score(input, &move_map))
}

pub fn day2_b(input: &String) -> String {
    let mut move_map = HashMap::new();
    move_map.insert(
        "A",
        HashMap::from([("X", 3 + 0), ("Y", 1 + 3), ("Z", 2 + 6)]),
    );
    move_map.insert(
        "B",
        HashMap::from([("X", 1 + 0), ("Y", 2 + 3), ("Z", 3 + 6)]),
    );
    move_map.insert(
        "C",
        HashMap::from([("X", 2 + 0), ("Y", 3 + 3), ("Z", 1 + 6)]),
    );
    format!("{}", calculate_score(input, &move_map))
}
