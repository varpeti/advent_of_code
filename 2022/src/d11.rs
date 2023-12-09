use std::{cell::RefCell, rc::Rc};

macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

#[derive(Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Pow,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u32,
    ok: usize,
    nok: usize,
}

fn parse_monkeys(input: &str) -> Vec<Rc<RefCell<Monkey>>> {
    let mut lines = input.lines();
    let mut monkeys = Vec::new();
    loop {
        // One Monkey
        if lines.next().is_none() {
            // Monkey id:
            break;
        }
        let items = p!(lines, " ")
            .skip(2)
            .map(|item| p!(item.split(','), u64))
            .collect::<Vec<_>>();
        let operation = match p!(lines, " ").skip(4).collect::<Vec<_>>() {
            add if add[0] == "+" => Operation::Add(add[1].parse::<u64>().expect("add")),
            mul if mul[0] == "*" && mul[1] != "old" => {
                Operation::Mul(mul[1].parse::<u64>().expect("mul"))
            }
            pow if pow[0] == "*" && pow[1] == "old" => Operation::Pow,
            err => panic!("Invalid operation: {:?}", err),
        };
        let test = p!(p!(lines, " ").skip(3), u32);
        let ok = p!(p!(lines, " ").skip(5), usize);
        let nok = p!(p!(lines, " ").skip(5), usize);
        let _ = lines.next(); // empty line

        // println!("{:?} {:?} {} {} {}", items, operation, test, ok, nok);
        monkeys.push(Rc::new(RefCell::new(Monkey {
            items,
            operation,
            test,
            ok,
            nok,
        })));
    }
    monkeys
}

fn solve(monkeys: Vec<Rc<RefCell<Monkey>>>, worry: &dyn Fn(u64) -> u64, rounds: usize) -> String {
    let mut monkeys_throw_count = vec![0; monkeys.len()];
    for _round in 0..rounds {
        for (id, monkey) in monkeys.iter().enumerate() {
            for item in monkey.borrow().items.iter() {
                let new_value = worry(match monkey.borrow().operation {
                    Operation::Add(num) => item + num,
                    Operation::Mul(num) => item * num,
                    Operation::Pow => item * item,
                });
                if new_value % monkey.borrow().test as u64 == 0 {
                    monkeys[monkey.borrow().ok]
                        .borrow_mut()
                        .items
                        .push(new_value);
                } else {
                    monkeys[monkey.borrow().nok]
                        .borrow_mut()
                        .items
                        .push(new_value);
                }
                monkeys_throw_count[id] += 1;
            }
            monkey.borrow_mut().items.clear();
        }
        if _round % 1000 == 0 || _round == 19 {
            println!("{:?}", monkeys_throw_count);
        }
    }
    for monkey in monkeys.iter() {
        println!("{:?}", monkey.borrow().items);
    }
    monkeys_throw_count.sort_unstable();
    format!(
        "{:?}",
        monkeys_throw_count
            .iter()
            .skip(monkeys_throw_count.len() - 2)
            .product::<u128>()
    )
}

pub fn day11_a(input: &str) -> String {
    let monkeys = parse_monkeys(input);
    println!("{:?}", monkeys);
    solve(
        monkeys,
        &|value: u64| (value as f32 / 3.).floor() as u64,
        20,
    )
}

pub fn day11_b(input: &str) -> String {
    let monkeys = parse_monkeys(input);
    let common_divisior = monkeys.iter().map(|m| m.borrow().test).product::<u32>() as u64;
    solve(monkeys, &|value: u64| value % common_divisior, 10000)
}
