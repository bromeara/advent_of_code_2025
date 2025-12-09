use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

use advent_of_code::util::parse_args;

// -1 for every empty space add 1 to every square around a roll then count the squares with less than 4
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let _ = run(file_path);
}

fn process_input(file_path: &str) -> (Vec<FreshRange>, Vec<u64>) {
    let contents = fs::read_to_string(file_path).unwrap();

    let mut ranges: Vec<FreshRange> = vec![];
    let mut stock: Vec<u64> = vec![];

    let mut finished_ranges = false;
    for line in contents.lines() {
        // println!(line);
        match line {
            "" => finished_ranges = true,
            _ => {
                if finished_ranges {
                    stock.push(line.parse::<u64>().unwrap());
                } else {
                    ranges.push(FreshRange::build(line));
                }
            }
        }
    }
    (ranges, stock)
}

fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let (mut ranges, stock) = process_input(file_path);
    let mut fresh_count = 0;
    let mut fresh_items: HashSet<u64> = HashSet::new();
    // for item in stock {
    //     if ranges.iter().any(|r| r.includes(item)) {
    //         fresh_count += 1;
    //     }
    // }

    // dbg!(&ranges);
    ranges.sort_by(|a, b| a.low.partial_cmp(&b.low).unwrap());
    // dbg!(&ranges);
    let mut preconsolidated_ranges: Vec<FreshRange> = vec![];
    preconsolidated_ranges.push(ranges[0].clone());
    for r in &ranges[1..] {
        let mut changed = false;
        for c in &mut preconsolidated_ranges {
            changed = c.consolidate(&r);
            if changed {
                break;
            }
        }
        if !changed {
            preconsolidated_ranges.push(r.clone());
        }
    }

    // let mut consolidated_ranges: Vec<FreshRange> = vec![];
    // consolidated_ranges.push(preconsolidated_ranges[0].clone());
    // for r in &preconsolidated_ranges[1..] {
    //     bar.inc(1);
    //     let mut changed = false;
    //     for c in &mut consolidated_ranges {
    //         changed = c.consolidate(&r);
    //         if changed {
    //             break;
    //         }
    //     }
    //     if !changed {
    //         consolidated_ranges.push(r.clone());
    //     }
    // }

    for r in preconsolidated_ranges {
        fresh_count += r.len();
    }
    println!("There are {} possible fresh ingredients.", fresh_count);
    Ok(())
}

#[derive(Debug, Clone)]
struct FreshRange {
    low: u64,
    high: u64,
}

impl FreshRange {
    fn build(raw: &str) -> FreshRange {
        let split: Vec<&str> = raw.split('-').collect();
        let first = split[0].parse::<u64>().unwrap();
        let second = split[1].parse::<u64>().unwrap();
        if second < first {
            return FreshRange {
                low: second,
                high: first,
            };
        }
        FreshRange {
            low: first,
            high: second,
        }
    }

    fn includes(&self, id: u64) -> bool {
        id >= self.low && id <= self.high
    }

    fn len(&self) -> u64 {
        let l = self.high - self.low + 1;
        // dbg!(&self);
        println!("len = {l}");
        l
    }

    fn consolidate(&mut self, new_range: &FreshRange) -> bool {
        if self.includes(new_range.low) {
            if self.includes(new_range.high) {
                return true;
            }
            self.high = new_range.high;
            return true;
        }
        if new_range.includes(self.low) {
            if new_range.includes(self.high) {
                self.high = new_range.high;
            }
            self.low = new_range.low;
            return true;
        }
        false
    }
}
