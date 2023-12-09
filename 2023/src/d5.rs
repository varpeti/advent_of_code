use std::collections::BTreeSet;

use itertools::Itertools;

// The order of the filed is matters! Cos the Ordering!
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Mapping {
    src: u64,
    len: u64,
    dst: u64,
}

impl Mapping {
    fn new_point(src: u64) -> Self {
        Mapping {
            src,
            len: u64::MAX,
            dst: u64::MAX,
        }
    }
    fn new_range(src: u64, len: u64) -> Self {
        Mapping {
            src,
            len,
            dst: u64::MAX,
        }
    }
}

fn parse_line(line: &str) -> Mapping {
    let mut line = line.split(' ');
    Mapping {
        dst: line.next().expect("dst").parse().expect("pdst"),
        src: line.next().expect("src").parse().expect("psrc"),
        len: line.next().expect("len").parse().expect("plen"),
    }
}

fn parse_block(block: &str) -> BTreeSet<Mapping> {
    block
        .trim_end()
        .split('\n')
        .skip(1)
        .map(parse_line)
        .collect()
}

pub fn day5_a(input: &str) -> String {
    let mut input = input.split("\n\n");
    let seeds = input
        .next()
        .expect("seeds")
        .split(' ')
        .skip(1)
        .map(|num| num.parse::<u64>().expect("num"));
    let block_of_mappings: Vec<BTreeSet<Mapping>> = input.map(parse_block).collect();
    //println!("block of mappings: {:#?}", block_of_mappings);
    let mut lowest = u64::MAX;
    for seed in seeds.into_iter() {
        let mut num = seed;
        for block in block_of_mappings.iter() {
            print!("{}", num);
            // Get all element from the first to src: seed, its O(log(n)) cos binary search
            let half_block = block.range(..=Mapping::new_point(num));
            // Get the last element its O(1)
            if let Some(mapping) = half_block.last() {
                if num >= mapping.src && num < mapping.src + mapping.len {
                    print!("\x1b[92m->\x1b[0m");
                    num = mapping.dst + (num - mapping.src);
                } else {
                    print!("\x1b[93m--\x1b[0m");
                }
            } else {
                print!("\x1b[91m--\x1b[0m");
            }
        }
        println!("{}", num);
        if lowest > num {
            lowest = num
        }
    }
    format!("{}", lowest)
}

pub fn day5_b(input: &str) -> String {
    let mut input = input.split("\n\n");
    let mut seeds = Vec::<Mapping>::new();
    for mut chunk in &input
        .next()
        .expect("seeds")
        .split(' ')
        .skip(1)
        .map(|num| num.parse::<u64>().expect("num"))
        .chunks(2)
    {
        let src = chunk.next().expect("src");
        let len = chunk.next().expect("len");
        seeds.push(Mapping::new_range(src, len));
    }
    let block_of_mappings: Vec<BTreeSet<Mapping>> = input.map(parse_block).collect();
    println!("block of mappings: {:#?}", block_of_mappings);
    let mut lowest = u64::MAX;
    for seed in seeds.into_iter() {
        println!("---");
        let mut ranges = vec![seed.to_owned()];
        for (bid, block) in block_of_mappings.iter().enumerate() {
            println!("{}:", bid);
            let mut new_ranges = vec![];
            for mut range in ranges.into_iter() {
                println!("{:?}", range);
                while range.len > 0 {
                    let mut used = false;
                    for mapping in block.iter() {
                        if range.src < mapping.src {
                            continue;
                        }
                        //          mapping | ===== | ===== | =====   |
                        //            range | ===== |  ===  |   ===== |
                        // intersection_len |  5    |   3   |    3    |
                        let delta = range.src - mapping.src;
                        if delta < mapping.len {
                            let intersection_len = u64::min(mapping.len - delta, range.len);
                            let new_range =
                                Mapping::new_range(mapping.dst + delta, intersection_len);
                            println!("-->{:?}", new_range);
                            new_ranges.push(new_range);
                            range.src += intersection_len;
                            range.len -= intersection_len;
                            used = true;
                            break;
                        }
                    }
                    if !used {
                        println!("---{:?}", range);
                        new_ranges.push(range);
                        break;
                    }
                }
            }
            ranges = new_ranges;
        }
        let min_ranges = ranges.iter().map(|range| range.src).min().expect("min");
        if min_ranges < lowest {
            lowest = min_ranges;
        }
    }
    format!("{}", lowest)
}
