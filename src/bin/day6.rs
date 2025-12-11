use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::str::Chars;

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

fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let equations = process_input_part2(file_path);
    let mut total = 0;
    for e in equations {
        total += e.result();
    }
    println!("Total value: {total}");
    Ok(())
}

fn process_input_part2(file_path: &str) -> Vec<Equation> {
    let mut result: Vec<Equation> = vec![];
    let contents = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = contents.split_inclusive('\n').collect();
    let mut as_chars: Vec<_> = lines.iter().map(|v| v.chars()).collect();

    // dbg!(&as_chars);
    let mut raw_columns: Vec<Vec<String>> = vec![];
    let mut problem: Vec<String> = vec![];
    for i in 0..lines[0].len() {
        let mut placeholder: Vec<char> = vec![];
        let mut preserve = false;
        for v in &mut as_chars {
            let char: char = v.next().unwrap();
            if char.is_alphanumeric() {
                preserve = true;
            }
            placeholder.push(char);
        }
        if preserve {
            problem.push(placeholder.into_iter().collect());
        } else {
            if !problem.is_empty() {
                raw_columns.push(problem.clone());
                problem = vec![];
            }
        }
    }
    for v in &raw_columns {
        result.push(Equation::build(v));
    }
    // dbg!(&result);
    result
}

fn process_input_part1(file_path: &str) -> Vec<Equation> {
    let contents = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let first_term_raw: Vec<u64> = process_line_to_u64(&lines[0]);
    let sec_term_raw = process_line_to_u64(&lines[1]);
    let third_term_raw = process_line_to_u64(&lines[2]);
    let fourth_term_raw = process_line_to_u64(&lines[3]);
    let operator: Vec<&str> = lines[4].split(' ').filter(|&c| c != "").collect();
    let mut equations: Vec<Equation> = vec![];

    if first_term_raw.len() != sec_term_raw.len() || third_term_raw.len() != operator.len() {
        println!("Error: Vector length mismatch.");
    } else {
        let length = first_term_raw.len();
        for i in 0..length {
            equations.push(Equation {
                terms: vec![
                    first_term_raw[i],
                    sec_term_raw[i],
                    third_term_raw[i],
                    fourth_term_raw[i],
                ],
                operator: operator[i].chars().next().unwrap(),
            });
        }
    }
    equations
}

fn process_line_to_u64(line: &str) -> Vec<u64> {
    line.split(' ')
        .filter(|&c| c != "")
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

#[derive(Debug)]
struct Equation {
    terms: Vec<u64>,
    operator: char,
}

impl Equation {
    fn build(raw: &Vec<String>) -> Equation {
        let mut terms: Vec<u64> = vec![];
        let mut first_term = raw[0].chars();
        let operator = first_term.next_back().unwrap();
        let t = first_term.as_str();
        terms.push(t.trim().parse::<u64>().unwrap());
        for r in &raw[1..] {
            terms.push(r.trim().parse::<u64>().unwrap());
        }
        Equation {
            terms: terms,
            operator: operator,
        }
    }
    fn result(&self) -> u64 {
        let mut result = 0;
        match self.operator {
            '*' => {
                result = 1;
                for i in &self.terms {
                    result *= *i;
                }
            }
            '+' => {
                for i in &self.terms {
                    result += *i;
                }
            }

            _ => {
                println!("Invalid operator");
            }
        }
        // dbg!(&self);
        // dbg!(result);
        result
    }
}
