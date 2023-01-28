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
    fn get_dist(&self, oth: &Self) -> i32 {
        (self.x - oth.x).abs() + (self.y - oth.y).abs()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Pair {
    s: Pos,
    b: Pos,
    r: i32,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    Possible,
    Nothing,
    Bacon,
    Sensor,
}

impl Status {
    /*
     * self:      Possible ---------------> Nothing --------------> Bacon/Sensor
     * new_value:          any but Possible         Bacon or Sensor
     */
    fn update(&mut self, new_value: Self) {
        match (&self, new_value) {
            (Status::Possible, v) => *self = v,
            (_, v) if v == Status::Bacon || v == Status::Sensor => *self = v,
            (_, _) => {}
        }
    }
}

impl Pair {
    fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        let s = Pos::new(a, b);
        let b = Pos::new(c, d);
        let r = s.get_dist(&b);
        Self { s, b, r }
    }

    fn is_in(&self, p: Pos) -> Status {
        if self.s == p {
            return Status::Sensor;
        }
        if self.b == p {
            return Status::Bacon;
        }
        if p.get_dist(&self.s) <= self.r {
            return Status::Nothing;
        }
        return Status::Possible;
    }

    fn get_max(&self) -> Pos {
        Pos::new(self.s.x + self.r, self.s.y + self.r)
    }

    fn get_min(&self) -> Pos {
        Pos::new(self.s.x - self.r, self.s.y - self.r)
    }
}

fn set_bounds(p: &Pair, min_p: &mut Pos, max_p: &mut Pos) {
    let min_o = p.get_min();
    if min_p.x >= min_o.x {
        min_p.x = min_o.x;
    }
    if min_p.y >= min_o.y {
        min_p.y = min_o.y;
    }
    let max_o = p.get_max();
    if max_p.x <= max_o.x {
        max_p.x = max_o.x;
    }
    if max_p.y <= max_o.y {
        max_p.y = max_o.y;
    }
}

fn parse_input(input: &String) -> (Vec<Pair>, Pos, Pos) {
    let mut min_p = Pos::new(i32::MAX, i32::MAX);
    let mut max_p = Pos::new(i32::MIN, i32::MIN);
    let pair_reg = Regex::new(
        r"Sensor at x=([-]?\d*), y=([-]?\d*): closest beacon is at x=([-]?\d*), y=([-]?\d*)",
    )
    .expect("bad regex");
    (
        input
            .lines()
            .map(|line| {
                let (a, b, c, d) = regex_captures!(pair_reg, line, i32, i32, i32, i32);
                let pair = Pair::new(a, b, c, d);
                set_bounds(&pair, &mut min_p, &mut max_p);
                pair
            })
            .collect::<Vec<Pair>>(),
        min_p,
        max_p,
    )
}

pub fn day15_a(input: &String) -> String {
    let (pairs, min_p, max_p) = parse_input(input);
    let y = 2000000;
    let mut count = 0;
    // Brute force the y line...
    for x in min_p.x..max_p.x {
        let mut status = Status::Possible;
        for pair in pairs.iter() {
            status.update(pair.is_in(Pos::new(x, y)));
            if status == Status::Bacon || status == Status::Sensor {
                break;
            }
        }
        // println!("{}: {:?}", x, status);
        if status == Status::Nothing {
            count += 1;
        }
    }
    format!("{}", count)
}

pub fn day15_b(input: &String) -> String {
    drop(input);
    format!("b")
}
