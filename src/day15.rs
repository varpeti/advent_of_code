use regex::Regex;

macro_rules! regex_captures {
    ($reg:expr, $line:expr, $($t:ty),*) => { {
        let c = $reg.captures($line).expect("captures");
        let mut i = c.iter();
        i.next(); // Skip the all match
        ($(
            i.next()
             .expect("next")
             .expect("match")
             .as_str()
             .parse::<$t>()
             .expect("parse")
        ),*)
    }};
}

#[derive(Debug, PartialEq, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Pair {
    s: Pos,
    b: Pos,
}

impl Pair {
    fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        Self {
            s: Pos::new(a, b),
            b: Pos::new(c, d),
        }
    }
}

fn parse_input(input: &String) -> u32 {
    let pair_reg = Regex::new(
        r"Sensor at x=([-]?\d*), y=([-]?\d*): closest beacon is at x=([-]?\d*), y=([-]?\d*)",
    )
    .expect("bad regex");
    for line in input.lines() {
        let (a, b, c, d) = regex_captures!(pair_reg, line, i32, i32, i32, i32);
        let pair = Pair::new(a, b, c, d);
        println!("{pair:?}");
    }
    0
}

pub fn day15_a(input: &String) -> String {
    format!("{}", parse_input(input))
}

pub fn day15_b(input: &String) -> String {
    drop(input);
    format!("b")
}
