use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::process;

use advent_of_code::util::parse_args;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let _ = run(file_path);
}

fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut manifold = process_input_part1(file_path);
    // dbg!(&manifold);
    let mut splitters = 0;
    loop {
        let res = manifold.propogate();
        match res {
            Ok(s) => {
                if s > 0 {
                    splitters += s
                }
            }
            Err(e) => {
                println!("Error: {e}");
                break;
            }
        }
    }
    let mut paths = 0;
    let final_row = manifold.grid.iter().last().unwrap();
    for cell in final_row {
        match cell {
            CellContents::Beam(n) => paths += n,
            _ => {}
        }
    }
    println!("Paths Encountered: {paths}");
    // dbg!(&manifold);
    Ok(())
}

fn process_input_part1(file_path: &str) -> TachyonManifold {
    let contents = fs::read_to_string(file_path).unwrap();
    TachyonManifold::build(&contents)
}

#[derive(Debug, PartialEq, Clone)]
enum CellContents {
    Empty,
    Start,
    Beam(i64),
    Splitter,
}

struct TachyonManifold {
    grid: Vec<Vec<CellContents>>,
    laser_position: usize,
    width: usize,
    height: usize,
}

impl TachyonManifold {
    fn build(raw: &str) -> TachyonManifold {
        let mut r_grid: Vec<Vec<CellContents>> = vec![];
        for line in raw.lines() {
            let mut layer: Vec<CellContents> = vec![];
            for c in line.chars() {
                match c {
                    '.' => layer.push(CellContents::Empty),
                    'S' => layer.push(CellContents::Start),
                    '|' => layer.push(CellContents::Beam(0)), //Shouldn't be part of input normally
                    '^' => layer.push(CellContents::Splitter),
                    _ => println!("Parsing error unknown charecter: {c}"),
                }
            }
            r_grid.push(layer);
        }
        TachyonManifold {
            width: r_grid[0].len(),
            height: r_grid.len(),
            grid: r_grid,
            laser_position: 0,
        }
    }

    fn propogate(&mut self) -> Result<i64, &'static str> {
        // println!(
        //     "Laser Position: {}, Height: {}",
        //     self.laser_position, self.height
        // );
        if self.laser_position >= self.height - 1 {
            return Err("Reached end of grid");
        }
        let mut result_row = self.grid[self.laser_position + 1].clone();
        let mut splitters = 0;
        for (i, cell) in self.grid[self.laser_position].iter().enumerate() {
            let target = &self.grid[self.laser_position + 1];

            match cell {
                CellContents::Empty => continue,
                CellContents::Start => {
                    if result_row[i] == CellContents::Splitter {
                        if i < self.width + 1 {
                            if result_row[i + 1] == CellContents::Empty {
                                result_row[i + 1] = CellContents::Beam(1);
                                splitters += 1;
                            }
                        }
                        if i > 0 {
                            if result_row[i - 1] == CellContents::Empty {
                                result_row[i - 1] = CellContents::Beam(1);
                                splitters += 1;
                            }
                        }
                    } else if result_row[i] == CellContents::Empty {
                        result_row[i] = CellContents::Beam(1);
                    }
                }
                CellContents::Beam(n) => match result_row[i] {
                    CellContents::Splitter => {
                        // println!("Beam hit splitter at ({},{})", self.laser_position, i);
                        // dbg!(&result_row);
                        if i < self.width + 1 {
                            match target[i + 1] {
                                CellContents::Empty => result_row[i + 1] = CellContents::Beam(*n),
                                CellContents::Beam(m) => {
                                    result_row[i + 1] = CellContents::Beam(n + m);
                                }
                                _ => {}
                            }
                        }
                        if i > 0 {
                            match result_row[i - 1] {
                                CellContents::Empty => result_row[i - 1] = CellContents::Beam(*n),
                                CellContents::Beam(m) => {
                                    result_row[i - 1] = CellContents::Beam(n + m);
                                }
                                _ => {}
                            }
                        }
                    }
                    CellContents::Beam(k) => {
                        result_row[i] = CellContents::Beam(n + k);
                    }
                    CellContents::Empty => result_row[i] = CellContents::Beam(*n),
                    _ => {}
                }, //Shouldn't be part of input normally
                CellContents::Splitter => continue,
            }
            // dbg!(&result_row);
        }
        // dbg!(&result_row);
        self.grid[self.laser_position + 1] = result_row;
        self.laser_position += 1;
        Ok(splitters)
    }
}

impl fmt::Debug for TachyonManifold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Manifold: [\n")?;
        for line in &self.grid {
            let mut c_line = String::new();
            for v in line {
                match v {
                    CellContents::Empty => c_line.push('.'),
                    CellContents::Start => c_line.push('S'),
                    CellContents::Beam(n) => {
                        let s = n.to_string();
                        c_line.push_str(&s);
                    } //Shouldn't be part of input normally
                    CellContents::Splitter => c_line.push('^'),
                }
            }
            write!(f, "{}\n", c_line)?;
        }
        write!(f, "]")
    }
}
