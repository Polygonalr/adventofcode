use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, VecDeque};
use std::cmp::max;

const MAX_TIME: usize = 30;
const FIRST_VALVE: &str = "AA";

fn build_maps(line_vec: Vec<String>) -> (HashMap<String, i32>, HashMap<String, Vec<String>>) {
    let mut flow_rates: HashMap<String, i32> = HashMap::new();
    let mut tunnels: HashMap<String, Vec<String>> = HashMap::new();
    for line in line_vec {
        let l: Vec<&str> = line.split("; ").collect();
        match l[..] {
            [valve_data, lead_to_data] => {
                if let [_, valve_name, _, _, flow_rate_str] = valve_data.split(' ').collect::<Vec<_>>()[0..5] {
                    let flow_rate_num: String = flow_rate_str.chars().filter(|c| c.is_digit(10)).collect();
                    flow_rates.insert(valve_name.to_owned(), flow_rate_num.parse::<i32>().unwrap());
                    let mut connected_valves: Vec<String> = vec![];
                    let other_valves = &(lead_to_data.split(' ').collect::<Vec<_>>()[4..]);
                    for &other_valve in other_valves {
                        connected_valves.push(other_valve.chars().filter(|c| *c != ',').collect::<String>());
                    }
                    tunnels.insert(valve_name.to_owned(), connected_valves);
                } else {
                    panic!("Invalid data detected.");
                }
            },
            _ => panic!("Invalid data detected.")
        }
    }
    (flow_rates, tunnels)
}

fn build_graph(flow_rates: &HashMap<String, i32>, tunnels: &HashMap<String, Vec<String>>) -> (HashMap<(String, String), usize>, HashMap<String, bool>) {
    let mut valve_graph: HashMap<(String, String), usize> = HashMap::new();
    let mut initial_valve_states: HashMap<String, bool> = HashMap::new();
    let mut useful_valves: Vec<String> = flow_rates.iter()
            .filter(|(_, flow)| **flow != 0)
            .map(|(name, _)| name.to_owned())
            .collect::<Vec<_>>();
    useful_valves.push(FIRST_VALVE.to_owned());
    for (i, from) in useful_valves.iter().enumerate() {
        for to in useful_valves.iter().skip(i + 1) {
            let mut visited: HashMap<String, bool> = HashMap::new();
            for (valve, _) in flow_rates {
                visited.insert(valve.to_owned(), false);
            }
            let mut queue: VecDeque<(String, usize)> = VecDeque::new();
            queue.push_back((from.to_owned(), 0));
            *visited.get_mut(from).unwrap() = true;
            let mut found = false;
            // BFS
            while let Some((node, weight)) = queue.pop_front() {
                for connected in tunnels[&node].iter() {
                    if *connected == *to {
                        found = true;
                        valve_graph.insert((from.to_owned(), to.to_owned()), weight + 1);
                        break;
                    } 
                    if !visited[connected] {
                        queue.push_back((connected.to_owned(), weight + 1));
                        *visited.get_mut(connected).unwrap() = true;
                    }
                }
            }
            if !found {
                panic!("BFS has issues.");
            }
        }
    }
    for valve in useful_valves {
        if valve != "AA".to_owned() {
            initial_valve_states.insert(valve, false);
        }
    }
    (valve_graph, initial_valve_states)
}

fn get_pressure_per_tick(flow_rates: &HashMap<String, i32>, valve_states: &HashMap<String, bool>) -> usize {
    // first, filter an iterator of String containing opened valves
    // then, get their flow rates and accumulate them with reduction
    valve_states.into_iter().filter(|(_, o)| **o)
            .map(|(v, _)| flow_rates.get(v).unwrap())
            .fold(0, |acc, f| acc + f) as usize
}

fn p1(
    flow_rates: &HashMap<String, i32>,
    valve_graph: &HashMap<(String, String), usize>,
    valve_states: &HashMap<String, bool>,
    curr_valve: &str,
    curr_pressure: usize,
    time_spent: usize,
) -> usize {
    println!("At {}, pressure: {}", curr_valve, curr_pressure);
    let mut travelled = false;
    let mut max_pressure: usize = 0;
    for (valve, opened) in valve_states {
        if valve == curr_valve {
            continue;
        }
        // println!("Trying to travel to {} from {}", valve, curr_valve);
        // get travel time to the other valve
        let &travel_time = valve_graph.get(&(curr_valve.to_owned(), valve.to_owned()))
            .or(valve_graph.get(&(valve.to_owned(), curr_valve.to_owned())))
            .unwrap();
        if !opened && time_spent + travel_time < MAX_TIME {
            // travel to the other valve
            travelled = true;
            
            let new_pressure = curr_pressure + ((travel_time + 1) * get_pressure_per_tick(flow_rates, valve_states));
            println!("{}", get_pressure_per_tick(flow_rates, valve_states));
            let mut new_valve_states = valve_states.clone();
            *new_valve_states.get_mut(curr_valve).unwrap() = true;
            let new_path_pressure = p1(flow_rates, valve_graph, valve_states, &valve, new_pressure, time_spent + travel_time + 1);
            max_pressure = max(max_pressure, new_path_pressure);
        }
    }
    if !travelled {
        // no more time to travel to other nodes, remain where I am until 30 mins is up
        if time_spent == MAX_TIME {
            return curr_pressure;
        }
        max_pressure = curr_pressure + get_pressure_per_tick(flow_rates, valve_states);
        // open the valve and calculate the accumulated pressure for the remaining time
        let mut new_valve_states = valve_states.clone();
        *new_valve_states.get_mut(curr_valve).unwrap() = true;
        max_pressure += (MAX_TIME - time_spent - 1) * get_pressure_per_tick(flow_rates, &new_valve_states);
    }
    max_pressure
}

fn p2(line_vec: Vec<String>) -> i32 {
    0
}

fn main() {
    let filepath = "./test.txt";
    // let mut str_buf = "".to_owned();
    let mut line_vec: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines.flatten() {
            // Process each line...
            line_vec.push(line);
        }
    }
    let (flow_rates, tunnels): (HashMap<String, i32>, HashMap<String, Vec<String>>) = build_maps(line_vec);
    for (valve, flow_rate) in &flow_rates {
        println!("{}: {}", valve, flow_rate);
    }
    for (valve, connected_valves) in &tunnels {
        print!("{}: ", valve);
        for connected in connected_valves {
            print!("{} ", connected);
        }
        println!();
    }
    let (valve_graph, initial_valve_states) = build_graph(&flow_rates, &tunnels);
    for ((from, to), weight) in valve_graph.iter() {
        println!("{} <-> {}: {}", from, to, weight);
    }
    let mut part1_max_ans = 0;
    for (starting_valve, _) in &initial_valve_states {
        println!("Choosing {} as starting valve", starting_valve);
        let travel_time = valve_graph.get(&(starting_valve.to_owned(), "AA".to_owned())).unwrap();
        let pressure = p1(&flow_rates, &valve_graph, &initial_valve_states, &starting_valve, 0, travel_time + 1);
        part1_max_ans = max(part1_max_ans, pressure);
    }
    println!("Part 1: {}\n", part1_max_ans);
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
