use regex::Regex;

struct Ship {
    stacks: Vec<Vec<char>>,
    row_re: Regex,
    move_re: Regex,
}

#[derive(PartialEq)]
enum ParseStatus {
    Cargo,
    Moving,
}

#[derive(PartialEq)]
enum CrateMover {
    V9000,
    V9001,
}

impl Ship {
    fn new() -> Self {
        let row_re = Regex::new(r"[ \[]([A-Z ])[ \]] ").expect("Bad Regex");
        let move_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").expect("Bad Regex");
        Self {
            stacks: Vec::new(),
            row_re,
            move_re,
        }
    }

    fn parse(&mut self, input: &String, crate_mover: &CrateMover) {
        let lines = input.lines();
        let mut state = ParseStatus::Cargo;
        for line in lines {
            if state == ParseStatus::Cargo {
                state = self.parse_cargo(line); // parsing the cargo layout
            } else {
                self.parse_moving(line, crate_mover); // parsing the moving instructions
            }
        }
    }

    fn parse_cargo(&mut self, line: &str) -> ParseStatus {
        if line == "" {
            return ParseStatus::Moving;
        }
        let row = format!("{} ", line); // Extra space for regex
        let row_capture = self.row_re.captures_iter(row.as_str());
        for (i, stuff) in row_capture.enumerate() {
            let stuff = stuff[1].chars().next().expect("not a char");
            if let None = self.stacks.get(i) {
                // initialize the stacks, TODO refactor
                self.stacks.push(Vec::new());
            }
            if stuff == ' ' {
                // space means no stuff in this stack
                continue;
            }
            self.stacks
                .get_mut(i)
                .expect(format!("{} index", i).as_str())
                .insert(0, stuff); // yeah.. it is expensive, TODO refactor
        }
        ParseStatus::Cargo
    }

    fn parse_moving(&mut self, line: &str, crate_mover: &CrateMover) {
        if let Some(line_capture) = self.move_re.captures(line) {
            self.move_stuff(
                line_capture[1].parse::<usize>().expect("p1"),
                line_capture[2].parse::<usize>().expect("p2") - 1, // -1 cos indexing starts from 1
                line_capture[3].parse::<usize>().expect("p3") - 1,
                crate_mover,
            )
        }
    }

    fn move_stuff(&mut self, num: usize, from: usize, to: usize, crate_mover: &CrateMover) {
        // println!("{:?} {:?} {} {} {}", &self.stacks[from], &self.stacks[to], num, from, to);
        // it is an ugly way to separate the slices, but optimal
        let to_arr;
        let from_arr;
        if from > to {
            let (first_half, second_half) = self.stacks.split_at_mut(from);
            to_arr = first_half.get_mut(to).expect("to index");
            from_arr = second_half.get_mut(0).expect("from index");
        } else {
            let (first_half, second_half) = self.stacks.split_at_mut(to);
            from_arr = first_half.get_mut(from).expect("to index");
            to_arr = second_half.get_mut(0).expect("from index");
        }
        // which crate_mover are we using? aka a or b task
        match crate_mover {
            CrateMover::V9000 => {
                for _ in 0..num {
                    if let Some(stuff) = from_arr.pop() {
                        to_arr.push(stuff);
                    }
                }
            }
            CrateMover::V9001 => {
                // It should be a better solution... 
                // but slices are killed my pc... so O(2num) is fine..
                let mut temp_stack = Vec::new();
                for _ in 0..num {
                    if let Some(stuff) = from_arr.pop() {
                        temp_stack.push(stuff);
                    }
                }
                for _ in 0..num {
                    if let Some(stuff) = temp_stack.pop() {
                        to_arr.push(stuff);
                    }
                }
            }
        }
    }

    fn get_top_stuff_from_stacks(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| *stack.last().expect("empty stack"))
            .collect()
    }
}

pub fn day5_a(input: &String) -> String {
    let mut ship = Ship::new();
    ship.parse(input, &CrateMover::V9000);
    format!("{}", ship.get_top_stuff_from_stacks())
}

pub fn day5_b(input: &String) -> String {
    let mut ship = Ship::new();
    ship.parse(input, &CrateMover::V9001);
    format!("{}", ship.get_top_stuff_from_stacks())
}
