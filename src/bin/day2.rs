// Invalid Ids are only possible if the first half of the numbers can appear in the second half.
// so splitting the first and second ids apart and then seeing if the range of them is posible in the full sequence
use std::collections::HashSet;
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

    if let Err(e) = run(file_path) {
        println!("Application error: {e}");
        process::exit(1);
    }
    // println!("Instructions:\n{raw_contents}");
    // dbg!(args);
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
    let contents = fs::read_to_string(file_path)?;
    let raw_range_pairs: Vec<&str> = contents.split(',').collect();
    let mut range_pairs: Vec<IDRange> = Vec::new();
    let mut silly_ids: HashSet<i64> = HashSet::new();
    for raw in raw_range_pairs {
        let parse_res = IDRange::build(raw);
        match parse_res {
            Ok(id_range) => {
                // dbg!(&raw);
                // dbg!(&id_range);
                range_pairs.push(id_range);

                let ids_res = get_sillyids(range_pairs.last().unwrap());
                match ids_res {
                    Ok(ids) => {
                        // dbg!(&ids);
                        for id in ids {
                            silly_ids.insert(id);
                        }
                    }
                    Err(e) => {
                        println!("Error: {e}");
                    }
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
    let silly_sum: i64 = silly_ids.iter().sum();
    println!("The sum of the silly ids is: {silly_sum}");
    // dbg!(range_pairs);
    Ok(())
}
#[derive(Debug)]
struct IDRange {
    low_id: String,
    high_id: String,
    low_value: i64,
    high_value: i64,
}

impl IDRange {
    fn build(raw_range: &str) -> Result<IDRange, &'static str> {
        let ranges: Vec<&str> = raw_range.split('-').collect();
        if ranges.len() < 2 {
            return Err("Not a valid id range string: Not enough IDs");
        }
        let low_id = ranges[0].to_string();
        let high_id = ranges[1].to_string();
        let low_value: i64;
        let high_value: i64;
        let low_value_result = low_id.parse::<i64>();
        match low_value_result {
            Ok(i) => {
                low_value = i;
            }
            Err(_e) => {
                return Err("Error getting int from str: low_value");
            }
        }
        let high_value_result = high_id.parse::<i64>();
        match high_value_result {
            Ok(i) => {
                high_value = i;
            }
            Err(_e) => {
                return Err("Error getting int from str: high_value");
            }
        }

        if high_value - low_value < 0 {
            return Err("Invalid range low_value is larger than high value.");
        }
        Ok(IDRange {
            low_id,
            high_id,
            low_value,
            high_value,
        })
    }

    fn is_valid(&self, id: &str) -> bool {
        let value = id.parse::<i64>().unwrap();
        value <= self.high_value && value >= self.low_value
    }
}

fn get_sillyids(range: &IDRange) -> Result<Vec<i64>, &'static str> {
    let mut silly_ids: Vec<i64> = Vec::new();

    for i in range.low_value..range.high_value + 1 {
        let id = format!("{}", i);
        for n in valid_pattern_factor(id.len()) {
            let c = split_every(&id, n);
            let mut comparisons = c.windows(2).peekable();
            if !comparisons.peek().is_some() {
                // println!("No windows were found.");
                continue;
            }
            let is_silly = c.windows(2).all(|w| w[0] == w[1]);
            if is_silly {
                // dbg!(&c);
                if i <= range.high_value && i >= range.low_value {
                    // dbg!(&c);
                    silly_ids.push(i);
                } else {
                    // dbg!(range);
                    // dbg!(&c);
                    println!("Invalid silly id{i}");
                }
                break;
            }
        }
    }
    Ok(silly_ids)
}

fn split_every<'a>(value: &'a str, n: usize) -> Vec<&'a str> {
    let number_of_slices = value.len() / n;
    let mut r_vector: Vec<&str> = Vec::new();
    for i in 0..number_of_slices {
        let s = &value[i * n..(i + 1) * n];
        r_vector.push(s);
    }
    r_vector
}

fn valid_pattern_factor(number: usize) -> Vec<usize> {
    let start = 2;
    let mut result: Vec<usize> = vec![1];
    for n in start..(number / 2) + 1 {
        if number % n == 0 {
            result.push(n);
        }
    }
    result.reverse();
    // dbg!(&result);
    result
}
