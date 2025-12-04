// Invalid Ids are only possible if the first half of the numbers can appear in the second half.
// so splitting the first and second ids apart and then seeing if the range of them is posible in the full sequence
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
    let mut silly_ids: Vec<i64> = Vec::new();
    for raw in raw_range_pairs {
        let parse_res = IDRange::build(raw);
        match parse_res {
            Ok(id_range) => {
                // dbg!(&raw);
                // dbg!(&id_range);
                range_pairs.push(id_range);

                let ids_res = get_sillyids(range_pairs.last().unwrap());
                match ids_res {
                    Ok(mut ids) => {
                        silly_ids.append(&mut ids);
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
            Err(e) => {
                return Err("Error getting int from str: low_value");
            }
        }
        let high_value_result = high_id.parse::<i64>();
        match high_value_result {
            Ok(i) => {
                high_value = i;
            }
            Err(e) => {
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
}

fn get_sillyids(range: &IDRange) -> Result<Vec<i64>, &'static str> {
    let mut low_id = range.low_id.clone();
    let low_initial_len = low_id.len();

    let mut high_id = range.high_id.clone();
    let high_initial_len = high_id.len();

    let mut silly_ids: Vec<i64> = Vec::new();

    if !low_initial_len.is_multiple_of(2) {
        low_id = (0..low_initial_len).map(|_| "0").collect::<String>();
        low_id.insert(0, '1');
        // dbg!(&low_id);
    };
    if !high_initial_len.is_multiple_of(2) {
        high_id = (0..high_initial_len - 1).map(|_| "9").collect::<String>();
        // dbg!(&high_id);
    };

    let low_range_front = String::from(low_id.split_at(low_id.len() / 2).0);
    let high_range_front = String::from(high_id.split_at(high_id.len() / 2).0);

    let low_front_value = low_range_front.parse::<i64>().unwrap();
    let high_front_value = high_range_front.parse::<i64>().unwrap();
    let silly_id_space = (low_front_value..(high_front_value + 1));

    for id in silly_id_space {
        let silly_id = format!("{}{}", id, id);
        let silly_id = silly_id.parse::<i64>().unwrap();
        if silly_id <= range.high_value && silly_id >= range.low_value {
            silly_ids.push(silly_id);
        }
    }
    // }
    Ok(silly_ids)
}

fn valid_pattern_factor(number: i64) -> Vec<i64> {
    let start = 2;
    let mut result: Vec<i64> = vec![1];
    for n in (start..(number / 2) + 1) {
        if number % n == 0 {
            result.push(n);
        }
    }
    result
}
