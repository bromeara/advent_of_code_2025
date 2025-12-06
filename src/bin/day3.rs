use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let _ = run(file_path);
}

fn parse_args(args: &[String]) -> Result<&str, &'static str> {
    if args.len() < 2 {
        return Err("Not enough arguments.");
    }
    let file_path = &args[1];
    println!("Loading: {file_path}");
    Ok(file_path)
}

fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let battery_banks = process_input(file_path);
    let mut total_joltage: i64 = 0;
    for bank in battery_banks {
        let j = bank.max_joltage();
        // println!("Max Joltage: {j}");
        total_joltage += i64::from(j);
    }
    println!("Total Joltage: {total_joltage}");
    Ok(())
}

#[derive(Debug)]
struct BatteryBank {
    batteries: Vec<u32>,
}

impl BatteryBank {
    fn build(raw: &str) -> Result<BatteryBank, &'static str> {
        let mut r_vector: Vec<u32> = Vec::new();
        for c in raw.chars() {
            let res = c.to_digit(10);
            match res {
                Some(num) => {
                    r_vector.push(num);
                }
                None => {
                    println!("No digit given");
                }
            }
        }
        Ok(BatteryBank {
            batteries: r_vector,
        })
    }

    fn max_joltage(&self) -> u32 {
        let first_digit = BatteryBank::max_digit(&self.batteries[..self.batteries.len() - 1], 0);
        let second_digit =
            BatteryBank::max_digit(&self.batteries[first_digit.0 + 1..], first_digit.0);
        first_digit.1 * 10 + second_digit.1
    }

    fn max_digit(batteries: &[u32], start_index: usize) -> (usize, u32) {
        let mut current_max = (start_index, batteries[0]);
        // println!("Starting max: ({}, {})", current_max.0, current_max.1);
        for (i, j) in batteries[1..].iter().enumerate() {
            let index = i + 1 + start_index;
            if *j > current_max.1 {
                current_max = (index, *j);
                // println!("New max: ({}, {})", current_max.0, current_max.1);
            }
            if *j == 9 {
                // println!("Found a 9");
                break;
            }
        }
        current_max
    }
}

fn process_input(file_path: &str) -> Vec<BatteryBank> {
    let mut r_vector: Vec<BatteryBank> = Vec::new();
    let contents = fs::read_to_string(file_path).unwrap();
    for raw in contents.lines() {
        r_vector.push(BatteryBank::build(raw).unwrap());
    }
    r_vector
}
