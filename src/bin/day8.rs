use std::env;
use std::error::Error;
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
    let mut junction_boxes = process_input(file_path);
    let mut junction_plan = JunctionPlan::build(&junction_boxes);

    let mut net: Vec<Network> = vec![];

    loop {
        let min = junction_plan.pop_minimum_distance();
        let persistent_min = min.clone();
        // dbg!(&min);
        junction_boxes[min.box1].connected = true;
        junction_boxes[min.box2].connected = true;
        let mut possible_nets: Vec<usize> = vec![];
        {
            possible_nets = net
                .iter()
                .enumerate()
                .filter(|(_, n)| min.is_in(n))
                .map(|(i, _)| i)
                .collect();
        };
        if possible_nets.len() == 0 {
            let new_network = Network::build(min);
            net.push(new_network);
        } else if possible_nets.len() > 1 {
            // println!("Merging.");
            for i in &possible_nets[1..] {
                let merged = net.remove(*i);
                net[possible_nets[0]].merge(merged);
            }
            net[possible_nets[0]].add(min);
        } else {
            net[possible_nets[0]].add(min);
        }
        if net.len() == 1 && net[0].node_list.len() == junction_boxes.len() {
            dbg!(&persistent_min);
            dbg!(&junction_boxes[persistent_min.box1].x);
            dbg!(&junction_boxes[persistent_min.box2].x);
            let wall_len =
                junction_boxes[persistent_min.box1].x * junction_boxes[persistent_min.box2].x;
            dbg!(wall_len);
            break;
        }
    }
    // for row in junction_plan.weights {
    //     println!("{:4.0?}", &row);
    // }

    // let mut max_connections: Vec<usize> = net.iter().map(|n| n.node_list.len()).collect();
    // max_connections.sort();
    // max_connections.reverse();
    // let mut magic_number = 1;
    // for i in &max_connections[0..=2] {
    //     magic_number *= *i as u64;
    // }
    // dbg!(&max_connections[0..3]);
    // dbg!(magic_number);
    // dbg!(&net);
    // dbg!(net.len());

    // let unconnected = junction_boxes.iter().filter(|j| !j.connected).count();
    // dbg!(unconnected);

    Ok(())
}

fn process_input(file_path: &str) -> Vec<JunctionBox> {
    let contents = fs::read_to_string(file_path).unwrap();
    let mut res: Vec<JunctionBox> = vec![];

    for line in contents.lines() {
        res.push(JunctionBox::build(line));
    }
    res
}

#[derive(Debug)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
    connected: bool,
}

impl JunctionBox {
    fn build(raw: &str) -> JunctionBox {
        // let t = raw.parse::<i32>();
        let terms: Vec<i64> = raw
            .split(',')
            .map(|n: &str| n.parse::<i64>().unwrap())
            .collect();
        JunctionBox {
            x: terms[0],
            y: terms[1],
            z: terms[2],
            connected: false,
        }
    }

    fn distance(&self, other: &JunctionBox) -> f64 {
        let inner: f64 = ((self.x - other.x).pow(2)
            + (self.y - other.y).pow(2)
            + (self.z - other.z).pow(2)) as f64;
        inner.sqrt()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Connection {
    weight: f64,
    box1: usize,
    box2: usize,
}

impl Connection {
    fn is_in(&self, n: &Network) -> bool {
        n.node_list
            .iter()
            .any(|v| *v == self.box1 || *v == self.box2)
    }
}

#[derive(Debug, PartialEq)]
struct Network {
    weight: f64,
    nodes: Vec<Connection>,
    node_list: Vec<usize>,
}

impl Network {
    fn build(c: Connection) -> Network {
        Network {
            weight: c.weight,
            node_list: vec![c.box1, c.box2],
            nodes: vec![c],
        }
    }
    fn add(&mut self, c: Connection) {
        if self.node_list.iter().all(|v| *v != c.box1) {
            self.node_list.push(c.box1);
        }
        if self.node_list.iter().all(|v| *v != c.box2) {
            self.node_list.push(c.box2);
        }
        self.weight += c.weight;
        self.nodes.push(c);
    }

    fn merge(&mut self, n: Network) {
        self.weight += n.weight;
        for c in n.nodes {
            self.add(c);
        }
    }
}
#[derive(Debug)]
struct JunctionPlan {
    weights: Vec<Vec<f64>>,
}

impl JunctionPlan {
    fn build(boxes: &Vec<JunctionBox>) -> JunctionPlan {
        let mut weights: Vec<Vec<f64>> = vec![vec![0.0; boxes.len()]; boxes.len() - 1];
        for (i, junction) in boxes.iter().enumerate() {
            for (j, other) in boxes[i + 1..].iter().enumerate() {
                let k = j + i + 1;
                weights[i][k] = junction.distance(other);
            }
        }
        JunctionPlan { weights: weights }
    }

    fn pop_minimum_distance(&mut self) -> Connection {
        let mut global_min = 0.0;
        let mut global_row = 0;
        let mut global_col = 0;

        for (i, row) in self.weights.iter().enumerate() {
            let res = row
                .iter()
                .enumerate()
                .filter(|(_, v)| **v != 0.0)
                .min_by(|(_, a), (_, b)| a.total_cmp(b));
            match res {
                Some((j, v)) => {
                    if *v < global_min || global_min == 0.0 {
                        global_min = *v;
                        global_col = j;
                        global_row = i;
                    }
                }
                None => {}
            }
        }
        self.weights[global_row][global_col] = 0.;
        Connection {
            weight: global_min,
            box1: global_row,
            box2: global_col,
        }
    }
}
