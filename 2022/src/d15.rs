#![allow(dead_code)]
use std::{cell::RefCell, rc::Rc};

use pathfinding::num_traits::PrimInt;
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
        Status::Possible
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

fn parse_input(input: &str) -> (Vec<Pair>, Pos, Pos) {
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

pub fn day15_a(input: &str) -> String {
    let (pairs, min_p, max_p) = parse_input(input);
    let y = 2000000;
    let mut count = 0;
    // Brute force the y line...
    // Should have used interception w/ line in (manhattan dist)
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

pub fn day15_b(input: &str) -> String {
    drop(input.to_owned());
    format!("{}", 'b')
}

type Neigh<N, V> = Option<Rc<RefCell<QuadTree<N, V>>>>;

struct QuadTree<N: PrimInt + Default + std::fmt::Debug, V: Clone + Default> {
    x: N,
    y: N,
    width: N,
    height: N,
    value: V,
    is_leaf: bool,
    area: N,
    min_size: N,

    pub neighs: [Neigh<N, V>; 4],
}

impl<N: PrimInt + Default + std::fmt::Debug, V: Clone + Default> QuadTree<N, V> {
    pub fn new(x: N, y: N, width: N, height: N, value: V, min_size: N) -> Self {
        Self {
            x,
            y,
            width,
            height,
            value,
            is_leaf: true,
            area: width * height,
            min_size,
            neighs: [None, None, None, None],
        }
    }

    pub fn set_value(&mut self, x: &N, y: &N, width: &N, height: &N, value: V) {
        if self.is_fully_contained(x, y, width, height) {
            if !self.is_leaf {
                self.merge();
            }
            self.value = value;
        } else if self.is_fully_outside(x, y, width, height) {
            return;
        } else {
            if self.area <= self.min_size {
                return;
            }
            if self.is_leaf {
                self.split()
            }

            self.set_child_values(x, y, width, height, value);
        }
    }

    pub fn get_value(&self, x: &N, y: &N) -> V {
        let mut value = V::default();
        self.get_value_rec(x, y, &mut value);
        value
    }

    pub fn draw(&self, f: &dyn Fn(&N, &N, &N, &N, &V)) {
        if self.is_leaf {
            f(&self.x, &self.y, &self.width, &self.height, &self.value)
        } else {
            for i in 0..self.neighs.len() {
                if let Some(neigh) = &self.neighs[i] {
                    neigh.borrow().draw(f);
                } else {
                    panic!("draw");
                }
            }
        }
    }

    fn get_value_rec(&self, x: &N, y: &N, value: &mut V) {
        if self.is_fully_outside(x, y, &N::default(), &N::default()) {
            return;
        }
        if self.is_leaf {
            *value = self.value.clone();
            return;
        }
        for i in 0..self.neighs.len() {
            if let Some(neigh) = &self.neighs[i] {
                neigh.borrow().get_value_rec(x, y, value);
            } else {
                panic!("get_value_rec")
            }
        }
    }

    //TODO draw

    fn merge(&mut self) {
        self.is_leaf = true;
        if let Some(neigh) = &self.neighs[0] {
            self.value = neigh.borrow().value.clone();
        } else {
            panic!("merge")
        }
    }

    fn split(&mut self) {
        self.is_leaf = false;
        // Having width and height
        let new_width = self.width >> 1;
        let new_height = self.height >> 1;

        self.neighs[0] = Some(Rc::new(RefCell::new(QuadTree::<N, V>::new(
            self.x,
            self.y,
            new_width,
            new_height,
            self.value.clone(),
            self.min_size,
        ))));
        self.neighs[1] = Some(Rc::new(RefCell::new(QuadTree::<N, V>::new(
            self.x + new_width,
            self.y,
            new_width,
            new_height,
            self.value.clone(),
            self.min_size,
        ))));
        self.neighs[2] = Some(Rc::new(RefCell::new(QuadTree::<N, V>::new(
            self.x,
            self.y + new_height,
            new_width,
            new_height,
            self.value.clone(),
            self.min_size,
        ))));
        self.neighs[3] = Some(Rc::new(RefCell::new(QuadTree::<N, V>::new(
            self.x + new_width,
            self.y + new_height,
            new_width,
            new_height,
            self.value.clone(),
            self.min_size,
        ))));
    }

    fn set_child_values(&mut self, x: &N, y: &N, width: &N, height: &N, value: V) {
        for i in 0..self.neighs.len() {
            if let Some(neigh) = &self.neighs[i] {
                neigh
                    .borrow_mut()
                    .set_value(x, y, width, height, value.clone());
            } else {
                panic!("set_child_values")
            }
        }
    }

    fn is_fully_contained(&self, x: &N, y: &N, width: &N, height: &N) -> bool {
        *x <= self.x
            && *x + *width >= self.x + self.width
            && *y <= self.y
            && *y + *height <= self.y + self.height
    }

    fn is_fully_outside(&self, x: &N, y: &N, width: &N, height: &N) -> bool {
        self.x + self.width <= *x
            || self.y + self.height <= *y
            || *x + *width <= self.x
            || *y + *height <= self.y
    }
}

impl<N: PrimInt + Default + std::fmt::Debug, V: Clone + Default> Clone for QuadTree<N, V> {
    fn clone(&self) -> Self {
        let mut new = QuadTree::<N, V>::new(
            self.x,
            self.y,
            self.width,
            self.height,
            self.value.clone(),
            self.min_size,
        );
        if self.is_leaf {
            return new;
        }
        for i in 0..self.neighs.len() {
            new.neighs[i] = self.neighs[i].clone();
        }
        new
    }
}

fn test_qt_draw(x: &i32, y: &i32, width: &i32, height: &i32, value: &i32) {
    println!("({} {})({} {}) : {}", x, y, width, height, value)
}

#[test]
fn test_quad_tree() {
    let mut qt = QuadTree::<i32, i32>::new(0, 0, 100, 100, 0, 1);

    println!("{:?}", qt.get_value(&15, &15));
    qt.draw(&test_qt_draw);
    println!("---");

    qt.set_value(&10, &10, &10, &10, 1);
    println!("{:?}", qt.get_value(&15, &15));
    qt.draw(&test_qt_draw);
    println!("---");

    qt.set_value(&10, &10, &10, &10, 0);
    println!("{:?}", qt.get_value(&15, &15));
    qt.draw(&test_qt_draw);
    println!("---");
}
