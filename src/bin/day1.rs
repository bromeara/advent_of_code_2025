use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = run(file_path) {
        println!("Application error: {e}");
        process::exit(1);
    }
    let d1 = 299;
    let d2 = 100;
    let d = d1/d2;
    println!("modtest: {d}")
    // println!("Instructions:\n{raw_contents}");
    // dbg!(args);
}

fn parse_args(args: &[String]) ->  Result<&str, &'static str> {
    if args.len() < 2 {
        return Err("Not enough arguments.");
    }
    let file_path = &args[1];
    println!("Loading: {file_path}");
    Ok(file_path)
}

fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let mut pos = 50;
    let mut count = 0;
    for instruction in parse_input(&contents){
        let (new_pos, passes) = rotate_lock(pos, instruction);
        count += passes;
        pos = new_pos;
    }
    println!("Count: {count}");

    Ok(())
}

fn rotate_lock(pos: i32, rotate: i32) -> (i32, i32) {
    let dial_size = 100;
    let mut count = (rotate / dial_size).abs();
    let mut new_pos = pos + (rotate % 100);
    if new_pos < 0{
        if pos != 0{
            count += 1;
        }
        new_pos+=dial_size;
    }
    else if new_pos > 99 {
        new_pos %= dial_size;
        if new_pos != 0{
            count += 1;
        }
    }
    if new_pos == 0 && pos!=0{
        count+=1;
    }
    println!("Rotate {pos} by {rotate} to {new_pos}: passes {count} times.");
    (new_pos, count)
}

fn get_direction_and_val(s: &str) -> (Option<char>, i32) {
    let mut chars = s.chars();
    let dir = chars.next();
    let val = chars.as_str().parse::<i32>().unwrap();
    (dir, val)
}
fn parse_input(contents: &str) -> Vec<i32> {
    let mut results = Vec::new();
    for line in contents.lines() {
        let (dir, mut v) = get_direction_and_val(line);
        match dir {
            Some(direction) => {
                if direction == 'L'{
                    v *= -1;
                }
                else if direction != 'R' {
                    println!("Invalid direction found {direction}");
                }
                }
            None => {
                println!("Encountered empty line")
            }
        }
        results.push(v);
        // println!("Instruction: ({line}, {v})");
    }
    results
}