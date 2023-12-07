// x*(t-x) > d
// t > 0
// 0 < d < t
// ------------------
// (t - sqrt(t*t-4d))/2 < x < (t + sqrt(t*t-4d))

pub fn day6_a(input: &String) -> String {
    let mut input = input.split("\n");
    let times = input
        .next()
        .expect("time")
        .split_ascii_whitespace()
        .skip(1)
        .map(|num| num.parse::<u32>().expect("num"));
    let distances = input
        .next()
        .expect("distance")
        .split_ascii_whitespace()
        .skip(1)
        .map(|num| num.parse::<u32>().expect("num"));
    let mut res = 1;
    for (time, distance) in times.zip(distances) {
        let r = f32::sqrt((time * time - 4 * distance) as f32);
        let mut low = ((time as f32 - r) / 2.).ceil() as u32;
        let mut hig = ((time as f32 + r) / 2.).floor() as u32;
        if low * (time - low) == distance {
            low += 1;
        }
        if hig * (time - hig) == distance {
            hig -= 1;
        }
        let num_of_ways = hig - low + 1;
        println!(
            "t{}, d{}, r{}, low{}, hig{}, #{}",
            time, distance, r, low, hig, num_of_ways
        );
        res *= num_of_ways;
    }
    format!("{}", res)
}

pub fn day6_b(input: &String) -> String {
    let mut input = input.split("\n");
    let time = input
        .next()
        .expect("time")
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u128>()
        .expect("ptime");
    let distance = input
        .next()
        .expect("distance")
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u128>()
        .expect("pdistance");
    let r = f64::sqrt((time * time - 4 * distance) as f64);
    let mut low = ((time as f64 - r) / 2.).ceil() as u128;
    let mut hig = ((time as f64 + r) / 2.).floor() as u128;
    if low * (time - low) == distance {
        low += 1;
    }
    if hig * (time - hig) == distance {
        hig -= 1;
    }
    let num_of_ways = hig - low + 1;
    println!(
        "t{}, d{}, r{}, low{}, hig{}, #{}",
        time, distance, r, low, hig, num_of_ways
    );
    format!("{}", num_of_ways)
}
