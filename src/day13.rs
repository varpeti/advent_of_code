use std::{cmp::Ordering, iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq)]
enum Data {
    Array(Vec<Data>),
    Number(u32),
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // println!("compare: {self:?} vs {other:?}");
        match (self, other) {
            (Data::Number(a), Data::Number(b)) => {
                // println!(" nn: {:?}", a.cmp(b));
                Some(a.cmp(b))
            }
            (Data::Array(a), Data::Array(b)) => {
                // println!(" aa: {a:?} vs {b:?}");
                for i in 0.. {
                    let oai = a.get(i);
                    let obi = b.get(i);
                    if matches!((oai, obi), (None, None)) {
                        // println!("  {i}: {:?}", Ordering::Equal);
                        return Some(Ordering::Equal);
                    }
                    if let Some(ai) = oai {
                        if let Some(bi) = obi {
                            let ord = ai.partial_cmp(bi);
                            // println!("  {i}: {:?}", &ord);
                            match ord {
                                Some(Ordering::Equal) => continue,
                                _ => return ord,
                            }
                        } else {
                            // println!("  {i}: b {:?}", Ordering::Greater);
                            return Some(Ordering::Greater);
                        }
                    } else {
                        // println!("  {i}: a {:?}", Ordering::Less);
                        return Some(Ordering::Less);
                    }
                }
                // println!("None - should never reach!");
                None // Should never reach
            }
            (Data::Array(_), Data::Number(b)) => {
                self.partial_cmp(&Data::Array(vec![Data::Number(*b)]))
            }
            (Data::Number(a), Data::Array(_)) => {
                Data::Array(vec![Data::Number(*a)]).partial_cmp(other)
            }
        }
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_signal(chars: &mut Peekable<Chars>) -> Data {
    let mut elems = vec![];
    loop {
        let c = chars.next();
        if matches!(c, None) {
            break;
        };
        let c = c.unwrap();
        match c {
            '[' => elems.push(parse_signal(chars)),
            ']' => break,
            n if n.is_numeric() => {
                let mut num = vec![c];
                loop {
                    let c = chars.peek();
                    if matches!(c, None) || !c.unwrap().is_numeric() {
                        break;
                    }
                    let c = chars.next().unwrap();
                    num.push(c);
                }
                elems.push(Data::Number(
                    num.iter().collect::<String>().parse::<u32>().unwrap(),
                ));
            }
            _ignore => (),
        };
    }
    Data::Array(elems)
}

fn parse_input(input: &String) -> Vec<(Data, Data)> {
    let mut lines = input.lines();
    let mut signals = vec![];
    loop {
        let a = lines.next();
        if matches!(a, None) {
            break;
        }
        let a = parse_signal(&mut a.unwrap().chars().peekable());
        let b = parse_signal(&mut lines.next().unwrap().chars().peekable());
        let _empty = lines.next();
        // println!("{a:?}\n{b:?}\n");
        signals.push((a, b));
    }
    signals
}

pub fn day13_a(input: &String) -> String {
    let data = parse_input(input);
    let mut righ_order = 0;
    for (i, (a, b)) in data.iter().enumerate() {
        if a < b {
            righ_order += i + 1;
        }
    }
    format!("{righ_order}")
}

pub fn day13_b(input: &String) -> String {
    let mut signals = input
        .lines()
        .filter(|&line| line != "")
        .map(|line| parse_signal(&mut line.chars().peekable()))
        .collect::<Vec<_>>();
    let driver_signals = [
        parse_signal(&mut "[[2]]".chars().peekable()),
        parse_signal(&mut "[[6]]".chars().peekable()),
    ];
    signals.push(parse_signal(&mut "[[2]]".chars().peekable()));
    signals.push(parse_signal(&mut "[[6]]".chars().peekable()));
    signals.sort_unstable();
    let mut decoder_key = 1;
    for (i, signal) in signals.iter().enumerate() {
        // println!("{signal:?}");
        if driver_signals.contains(signal) {
            decoder_key *= i + 1;
        }
    }
    format!("{decoder_key}")
}

#[test]
fn data_ord() {
    let a = parse_signal(&mut "[[1],[2,3,5]]".chars().peekable());
    let b = parse_signal(&mut "[1,1,5,1,2]".chars().peekable());
    assert!(b < a);
    assert!(a > b);
    assert!(a == a);
    assert!(a != b);
}
