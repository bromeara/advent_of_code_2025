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

fn process_input(file_path: &str) -> Vec<BatteryBank> {
    let mut r_vector: Vec<BatteryBank> = Vec::new();
    let contents = fs::read_to_string(file_path).unwrap();
    for raw in contents.lines() {
        r_vector.push(BatteryBank::build(raw).unwrap());
    }
    r_vector
}

fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let battery_banks = process_input(file_path);
    let mut total_joltage: u64 = 0;
    for bank in battery_banks {
        let j = bank.max_joltage(12);
        // println!("Max Joltage: {j}");
        total_joltage += u64::from(j);
    }
    println!("Total Joltage: {total_joltage}");
    Ok(())
}
