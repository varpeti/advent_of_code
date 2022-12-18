use std::collections::HashMap;

#[derive(Debug)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn new(x: u32, y: u32) -> Self {
        Coord { y, x }
    }
}

#[derive(Debug)]
enum UnitType {
    Stone,
    Sand,
    Source,
}

type Units = HashMap<u32, HashMap<u32, UnitType>>;

trait UnitsMethods {
    fn ensure_y(&mut self, y: &u32);
    fn insert_xy(&mut self, x: u32, y: u32, unit_type: UnitType);
    fn get_xy(&self, x: u32, y: u32) -> Option<&UnitType>;
    fn get_xy_mut(&mut self, x: &u32, y: &u32) -> Option<&mut UnitType>;
}

impl UnitsMethods for Units {
    fn ensure_y(&mut self, y: &u32) {
        if !self.contains_key(&y) {
            self.insert(*y, HashMap::new());
        }
    }

    fn insert_xy(&mut self, x: u32, y: u32, unit_type: UnitType) {
        let x_units = self.get_mut(&y).expect("y");
        x_units.insert(x, unit_type);
        // println!("({x},{y})");
    }

    fn get_xy(&self, x: u32, y: u32) -> Option<&UnitType> {
        match self.get(&y) {
            Some(x_units) => x_units.get(&x),
            None => None,
        }
    }

    fn get_xy_mut(&mut self, x: &u32, y: &u32) -> Option<&mut UnitType> {
        match self.get_mut(y) {
            Some(x_units) => x_units.get_mut(x),
            None => None,
        }
    }
}

fn _visualize(units: &Units, minc: &Coord, maxc: &Coord) {
    for y in minc.y..maxc.y + 1 {
        for x in minc.x..maxc.x + 1 {
            print!(
                "{}",
                match units.get_xy(x, y) {
                    Some(UnitType::Stone) => "#",
                    Some(UnitType::Sand) => "o",
                    Some(UnitType::Source) => "+",
                    None => ".",
                }
            );
        }
        println!("");
    }
}

fn parse_input(input: &String) -> (Units, Coord, Coord) {
    let mut minc = Coord::new(500, 0);
    let mut maxc = Coord::new(500, 0);
    let mut units = Units::new();
    for line in input.lines() {
        let mut stone_line = line.split(" -> ").map(|stone| {
            let mut coord = stone.split(",");
            let x = coord.next().expect("x").parse::<u32>().expect("x parse");
            let y = coord.next().expect("y").parse::<u32>().expect("y parse");
            Coord::new(x, y)
        });
        let mut last = stone_line.next().expect("last");
        for next in stone_line {
            // println!("{last:?} -> {next:?}");
            for y in last.y.min(next.y)..last.y.max(next.y) + 1 {
                units.ensure_y(&y);
                for x in last.x.min(next.x)..last.x.max(next.x) + 1 {
                    units.insert_xy(x, y, UnitType::Stone);

                    //For bounds
                    minc.x = minc.x.min(x);
                    minc.y = minc.y.min(y);
                    maxc.x = maxc.x.max(x);
                    maxc.y = maxc.y.max(y);
                }
            }
            last = next
        }
    }
    (units, minc, maxc)
}

fn simulate_sand_a(units: &mut Units, max_y: u32) -> bool {
    let mut sand = (500, 0);
    loop {
        let mut ok = false;
        for moves in [(0, 1), (-1, 1), (1, 1)] {
            match units.get_xy(
                (sand.0 as i32 + moves.0) as u32,
                (sand.1 as i32 + moves.1) as u32,
            ) {
                Some(UnitType::Sand) | Some(UnitType::Stone) => (),
                Some(UnitType::Source) | None => {
                    sand.0 = sand.0 + moves.0;
                    sand.1 = sand.1 + moves.1;
                    if sand.1 as u32 >= max_y {
                        return false;
                    }
                    ok = true;
                    break;
                }
            }
        }
        if ok {
            continue;
        }
        units.ensure_y(&(sand.1 as u32));
        units.insert_xy(sand.0 as u32, sand.1 as u32, UnitType::Sand);
        return true;
    }
}

fn run_simulation_a(units: &mut Units, _minc: &Coord, maxc: &Coord) -> u32 {
    let mut counter = 0;
    loop {
        // _visualize(units, _minc, maxc);
        if !simulate_sand_a(units, maxc.y) {
            break;
        }
        counter += 1;
    }
    counter
}

pub fn day14_a(input: &String) -> String {
    let (mut units, minc, maxc) = parse_input(input);
    units.ensure_y(&0);
    units.insert_xy(500, 0, UnitType::Source);
    // println!("{units:?}");
    format!("{}", run_simulation_a(&mut units, &minc, &maxc))
}

fn simulate_sand_b(units: &mut Units, max_y: u32, minc: &mut Coord, maxc: &mut Coord) -> bool {
    let mut sand = (500, 0);
    loop {
        let mut ok = false;
        for moves in [(0, 1), (-1, 1), (1, 1)] {
            let x = (sand.0 as i32 + moves.0) as u32;
            let y = (sand.1 as i32 + moves.1) as u32;
            match units.get_xy(x, y) {
                Some(UnitType::Sand) | Some(UnitType::Stone) => (),
                Some(UnitType::Source) | None => {
                    if y >= max_y {
                        continue;
                    }
                    sand.0 = x;
                    sand.1 = y;
                    ok = true;
                    break;
                }
            }
        }
        if ok {
            continue;
        }
        if sand == (500, 0) {
            return false;
        }
        units.ensure_y(&(sand.1 as u32));
        units.insert_xy(sand.0 as u32, sand.1 as u32, UnitType::Sand);

        //For bounds
        minc.x = minc.x.min(sand.0);
        minc.y = minc.y.min(sand.1);
        maxc.x = maxc.x.max(sand.0);
        maxc.y = maxc.y.max(sand.1);
        return true;
    }
}

fn run_simulation_b(units: &mut Units, max_y: u32, minc: &mut Coord, maxc: &mut Coord) -> u32 {
    let mut counter = 1;
    loop {
        // _visualize(units, minc, maxc);
        if !simulate_sand_b(units, max_y, minc, maxc) {
            break;
        }
        counter += 1;
    }
    _visualize(&units, &minc, &maxc);
    counter
}

pub fn day14_b(input: &String) -> String {
    let (mut units, mut minc, mut maxc) = parse_input(input);
    units.ensure_y(&0);
    units.insert_xy(500, 0, UnitType::Source);
    format!(
        "{}",
        run_simulation_b(&mut units, maxc.y + 2, &mut minc, &mut maxc)
    )
}
