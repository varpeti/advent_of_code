use std::{collections::HashSet, str::Lines};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn diff(&self, other: &Self) -> Self {
        Self {
            x: (self.x - other.x),
            y: (self.y - other.y),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Action {
    direction: Direction,
    step: u32,
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Coord>,
    current_action: Action,
}

impl Rope {
    fn new(num_of_knots: usize) -> Self {
        Rope {
            knots: vec![Coord::new(0, 0); num_of_knots],
            current_action: Action {
                direction: Direction::Up,
                step: 0,
            },
        }
    }

    fn parse_head_movement(movement: &str) -> Action {
        let mut movement = movement.split(" ");
        let direction = movement.next().expect("direction");
        let step = movement
            .next()
            .expect("step")
            .parse::<u32>()
            .expect("step parse");

        match direction {
            "U" => Action {
                direction: Direction::Up,
                step,
            },
            "R" => Action {
                direction: Direction::Right,
                step,
            },
            "D" => Action {
                direction: Direction::Down,
                step,
            },
            "L" => Action {
                direction: Direction::Left,
                step,
            },
            err => panic!("Invalid input: {}", err),
        }
    }

    fn update(&mut self, lines: &mut Lines) -> bool {
        if self.current_action.step == 0 {
            match lines.next() {
                Some(movement) => self.current_action = Self::parse_head_movement(movement),
                None => return false,
            }
            println!("{:?}", self.current_action);
        }
        self.head_update();
        for id in 1..self.knots.len() {
            self.tail_update(id);
        }
        println!("{:?}", self.knots);
        true
    }

    fn head_update(&mut self) {
        let head = &mut self.knots[0];
        match self.current_action.direction {
            Direction::Up => head.y += 1,
            Direction::Right => head.x += 1,
            Direction::Down => head.y -= 1,
            Direction::Left => head.x -= 1,
        }
        self.current_action.step -= 1;
    }

    fn tail_update(&mut self, id: usize) {
        let slices = self.knots.split_at_mut(id);
        let head = &slices.0[id - 1];
        let tail = &mut slices.1[0];
        let d = tail.diff(&head);
        if d.x.abs() <= 1 && d.y.abs() <= 1 {
            return; // Touching
        }
        tail.y -= d.y.signum() * 1;
        tail.x -= d.x.signum() * 1;
    }
}

pub fn day9_a(input: &String) -> String {
    let mut rope = Rope::new(2);
    let mut lines = input.lines();
    let mut visited_by_tail = HashSet::new();
    while rope.update(&mut lines) {
        visited_by_tail.insert(rope.knots[rope.knots.len() - 1].clone());
    }
    format!("{}", visited_by_tail.len())
}

pub fn day9_b(input: &String) -> String {
    let mut rope = Rope::new(10);
    let mut lines = input.lines();
    let mut visited_by_tail = HashSet::new();
    while rope.update(&mut lines) {
        visited_by_tail.insert(rope.knots[rope.knots.len() - 1].clone());
    }
    format!("{}", visited_by_tail.len())
}
