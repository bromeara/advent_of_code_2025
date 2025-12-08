use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::process;

// -1 for every empty space add 1 to every square around a roll then count the squares with less than 4
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

fn process_input(file_path: &str) -> FloorGrid {
    let contents = fs::read_to_string(file_path).unwrap();
    FloorGrid::build(&contents)
}

fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut floor_grid = process_input(file_path);
    // dbg!(&floor_grid);

    let mut removed_rolls = floor_grid.remove_rolls();
    let mut removed_count = removed_rolls;
    while removed_rolls != 0 {
        removed_rolls = floor_grid.remove_rolls();
        removed_count += removed_rolls;
        // println!("Remove {removed_rolls} roll of paper");
        // dbg!(&floor_grid);
    }
    println!("Removed {removed_count} rolls of paper total");
    // dbg!(&floor_grid);
    Ok(())
}

struct FloorGrid {
    warehouse: Vec<Vec<i32>>,
    width: i32,
    height: i32,
}

impl FloorGrid {
    fn build(raw: &str) -> FloorGrid {
        let lines: Vec<&str> = raw.split_terminator('\n').collect();
        let width = lines[0].len();
        let height = lines.len();
        let mut grid = vec![vec![0; width]; height];

        // println!("W: {}, H: {}", &width, &height);
        // println!("First Line {}", &lines[0]);
        for (row, value) in lines.iter().enumerate() {
            for (column, ch) in value.chars().enumerate() {
                match ch {
                    '.' => grid[row][column] = -1,
                    '@' => {
                        for r in row as i32 - 1..=row as i32 + 1 {
                            if r >= 0 && r < height as i32 {
                                for c in column as i32 - 1..=column as i32 + 1 {
                                    if c >= 0 && c < width as i32 {
                                        if r == row as i32 && c == column as i32 {
                                            continue;
                                        }
                                        if grid[r as usize][c as usize] >= 0 {
                                            grid[r as usize][c as usize] += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        FloorGrid {
            warehouse: grid,
            width: width as i32,
            height: height as i32,
        }
    }

    fn recalculate(&mut self) {
        let mut new_warehouse = vec![vec![0; self.width as usize]; self.height as usize];
        for (row, value) in &mut self.warehouse.iter().enumerate() {
            for (column, roll) in &mut value.iter().enumerate() {
                match roll {
                    n if *n < 0 => {
                        new_warehouse[row][column] = -1;
                    }
                    _ => {
                        for r in row as i32 - 1..=row as i32 + 1 {
                            if r >= 0 && r < self.height as i32 {
                                for c in column as i32 - 1..=column as i32 + 1 {
                                    if c >= 0 && c < self.width as i32 {
                                        if r == row as i32 && c == column as i32 {
                                            continue;
                                        }
                                        if new_warehouse[r as usize][c as usize] >= 0 {
                                            new_warehouse[r as usize][c as usize] += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self.warehouse = new_warehouse;
    }

    fn remove_rolls(&mut self) -> i32 {
        let mut count = 0;
        for row in &mut self.warehouse {
            for value in row {
                if *value < 4 && *value >= 0 {
                    *value = -1;
                    count += 1;
                }
            }
        }
        self.recalculate();
        count
    }
}

impl fmt::Debug for FloorGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Warehouse: [\n")?;
        for line in &self.warehouse {
            let mut c_line = String::new();
            for n in line {
                match n {
                    -1 => c_line.push('.'),
                    0..4 => c_line.push('x'),
                    _ => c_line.push('@'),
                }
            }
            write!(f, "{:?}\n", c_line)?;
        }
        write!(f, "]")
    }
}
