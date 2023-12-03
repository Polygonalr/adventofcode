use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_TIME_P1: usize = 31;
const MAX_TIME_P2: usize = 27;
const FIRST_VALVE: &str = "AA";

fn get_useful_valves(flow_rates: &HashMap<String, i32>) -> Vec<String> {
    flow_rates
        .iter()
        .filter(|(_, f)| **f != 0)
        .map(|(name, _)| name.to_owned())
        .collect::<Vec<_>>()
}

fn build_maps(line_vec: Vec<String>) -> (HashMap<String, i32>, HashMap<String, Vec<String>>) {
    let mut flow_rates: HashMap<String, i32> = HashMap::new();
    let mut tunnels: HashMap<String, Vec<String>> = HashMap::new();
    for line in line_vec {
        let l: Vec<&str> = line.split("; ").collect();
        match l[..] {
            [valve_data, lead_to_data] => {
                if let [_, valve_name, _, _, flow_rate_str] =
                    valve_data.split(' ').collect::<Vec<_>>()[0..5]
                {
                    let flow_rate_num: String = flow_rate_str
                        .chars()
                        .filter(|c| c.is_ascii_digit())
                        .collect();
                    flow_rates.insert(valve_name.to_owned(), flow_rate_num.parse::<i32>().unwrap());
                    let mut connected_valves: Vec<String> = vec![];
                    let other_valves = &(lead_to_data.split(' ').collect::<Vec<_>>()[4..]);
                    for &other_valve in other_valves {
                        connected_valves.push(
                            other_valve
                                .chars()
                                .filter(|c| *c != ',')
                                .collect::<String>(),
                        );
                    }
                    tunnels.insert(valve_name.to_owned(), connected_valves);
                } else {
                    panic!("Invalid data detected.");
                }
            }
            _ => panic!("Invalid data detected."),
        }
    }
    (flow_rates, tunnels)
}

fn build_graph(
    flow_rates: &HashMap<String, i32>,
    tunnels: &HashMap<String, Vec<String>>,
) -> HashMap<(String, String), usize> {
    let mut valve_graph: HashMap<(String, String), usize> = HashMap::new();
    let mut useful_valves: Vec<String> = get_useful_valves(flow_rates);
    useful_valves.push(FIRST_VALVE.to_owned());
    for (i, from) in useful_valves.iter().enumerate() {
        for to in useful_valves.iter().skip(i + 1) {
            let mut visited: HashMap<String, bool> = HashMap::new();
            for valve in flow_rates.keys() {
                visited.insert(valve.to_owned(), false);
            }
            let mut queue: VecDeque<(String, usize)> = VecDeque::new();
            queue.push_back((from.to_owned(), 0));
            visited.entry(from.to_owned()).and_modify(|v| *v = true);
            let mut found = false;
            // BFS
            while let Some((node, weight)) = queue.pop_front() {
                for connected in tunnels[&node].iter() {
                    if *connected == *to {
                        found = true;
                        valve_graph.insert((from.to_owned(), to.to_owned()), weight + 1);
                        // println!("{} <-> {}: {}", from, to, weight + 1);
                        break;
                    }
                    if !visited[connected] {
                        queue.push_back((connected.to_owned(), weight + 1));
                        visited
                            .entry(connected.to_owned())
                            .and_modify(|v| *v = true);
                    }
                }
                if found {
                    break;
                }
            }
            if !found {
                panic!("BFS has issues.");
            }
        }
    }
    valve_graph
}

fn get_memo_key(opened_valves: &[String]) -> String {
    let mut key = "".to_owned();
    let mut sorted = opened_valves.to_vec();
    sorted.sort();
    for valve in sorted {
        key += &valve;
    }
    key
}

fn p1(
    flow_rates: &HashMap<String, i32>,
    valve_graph: &HashMap<(String, String), usize>,
    useful_valves: &Vec<String>,
    opened_valves: &[String],
    curr_valve: &str,
    curr_pressure: usize,
    time_spent: usize,
    max_time: usize,
) -> usize {
    // add the current valve's pressure first
    let new_pressure = curr_pressure + (flow_rates[curr_valve] as usize * (max_time - time_spent));
    let mut travelled = false;
    let mut max_pressure: usize = 0;
    for valve in useful_valves {
        if valve == curr_valve {
            continue;
        }
        // println!("Trying to travel to {} from {}", valve, curr_valve);
        // get travel time to the other valve
        let &travel_time = valve_graph
            .get(&(curr_valve.to_owned(), valve.to_owned()))
            .or_else(|| valve_graph.get(&(valve.to_owned(), curr_valve.to_owned())))
            .unwrap();
        if !opened_valves.contains(valve) && time_spent + travel_time + 1 < max_time {
            // travel to the other valve
            travelled = true;
            // println!("{}", get_pressure_per_tick(flow_rates, opened_valves));
            let mut new_opened_valves = opened_valves.to_vec();
            new_opened_valves.push(curr_valve.to_owned());
            let new_path_pressure = p1(
                flow_rates,
                valve_graph,
                useful_valves,
                &new_opened_valves,
                valve,
                new_pressure,
                time_spent + travel_time + 1,
                max_time,
            );
            max_pressure = max(max_pressure, new_path_pressure);
        }
    }
    if !travelled {
        // no more time to travel to other nodes, remain where I am until 30 mins is up
        // open the valve and calculate the accumulated pressure for the remaining time
        max_pressure = new_pressure;
    }
    max_pressure
}

/**
 * Same algo as part 1, except we calculate the elephant's path with the leftover
 * unopened valves and sum the pressures up.
 */
fn p2(
    flow_rates: &HashMap<String, i32>,
    valve_graph: &HashMap<(String, String), usize>,
    useful_valves: &Vec<String>,
    opened_valves: &[String],
    curr_valve: &str,
    curr_pressure: usize,
    time_spent: usize,
    memo: &mut HashMap<String, usize>,
    max_time: usize,
) -> usize {
    // add the current valve's pressure first
    let new_pressure = curr_pressure + (flow_rates[curr_valve] as usize * (max_time - time_spent));
    let mut new_opened_valves = opened_valves.to_vec();
    new_opened_valves.push(curr_valve.to_owned());

    let mut travelled = false;
    let mut max_pressure: usize = 0;
    for valve in useful_valves {
        if valve == curr_valve {
            continue;
        }
        // println!("Trying to travel to {} from {}", valve, curr_valve);
        // get travel time to the other valve
        let &travel_time = valve_graph
            .get(&(curr_valve.to_owned(), valve.to_owned()))
            .or_else(|| valve_graph.get(&(valve.to_owned(), curr_valve.to_owned())))
            .unwrap();
        if !opened_valves.contains(valve) && time_spent + travel_time + 1 < max_time {
            // travel to the other valve
            travelled = true;
            // println!("{}", get_pressure_per_tick(flow_rates, opened_valves));
            let new_path_pressure = p2(
                flow_rates,
                valve_graph,
                useful_valves,
                &new_opened_valves,
                valve,
                new_pressure,
                time_spent + travel_time + 1,
                memo,
                max_time,
            );
            max_pressure = max(max_pressure, new_path_pressure);
        }
    }
    if !travelled {
        // no more time to travel to other nodes, remain where I am until 26 mins is up
        // open the valve and calculate the accumulated pressure for the remaining time
        max_pressure = new_pressure;
        // then calculate the best path for the elephant for the remaining unopened valves
        // First, check memo
        let memo_key = get_memo_key(&new_opened_valves);
        if !memo.contains_key(&memo_key) {
            let mut max_elephant_pressure = 0;
            for starting_valve in useful_valves {
                if new_opened_valves.contains(starting_valve) {
                    continue;
                }
                let travel_time = valve_graph
                    .get(&(starting_valve.to_owned(), "AA".to_owned()))
                    .unwrap();
                let total_pressure = p1(
                    flow_rates,
                    valve_graph,
                    useful_valves,
                    &new_opened_valves,
                    starting_valve,
                    0,
                    travel_time + 2,
                    MAX_TIME_P2,
                );
                max_elephant_pressure = max(max_elephant_pressure, total_pressure);
            }
            memo.insert(memo_key.to_owned(), max_elephant_pressure);
        }
        max_pressure += memo.get(&memo_key).unwrap();
    }
    max_pressure
}

fn main() {
    let filepath = "./input.txt";
    // let mut str_buf = "".to_owned();
    let mut line_vec: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines.flatten() {
            // Process each line...
            line_vec.push(line);
        }
    }
    let (flow_rates, tunnels): (HashMap<String, i32>, HashMap<String, Vec<String>>) =
        build_maps(line_vec);
    let valve_graph = build_graph(&flow_rates, &tunnels);

    // ----- part 1 -----
    let mut part1_max_ans = 0;
    let useful_valves = get_useful_valves(&flow_rates);
    for starting_valve in &useful_valves {
        // println!("Choosing {} as starting valve", starting_valve);
        let travel_time = valve_graph
            .get(&(starting_valve.to_owned(), "AA".to_owned()))
            .unwrap();
        // open valve
        let total_pressure = p1(
            &flow_rates,
            &valve_graph,
            &get_useful_valves(&flow_rates),
            &[],
            starting_valve,
            0,
            travel_time + 2,
            MAX_TIME_P1,
        );
        // println!("Starting at: {}, Pressure: {}", starting_valve, total_pressure);
        part1_max_ans = max(part1_max_ans, total_pressure);
    }
    println!("Part 1: {}", part1_max_ans);

    // ----- part 2 -----
    let mut part2_max_ans = 0;
    let mut part2_memo: HashMap<String, usize> = HashMap::new();
    for starting_valve in &useful_valves {
        let travel_time = valve_graph
            .get(&(starting_valve.to_owned(), "AA".to_owned()))
            .unwrap();
        let total_pressure = p2(
            &flow_rates,
            &valve_graph,
            &get_useful_valves(&flow_rates),
            &[],
            starting_valve,
            0,
            travel_time + 2,
            &mut part2_memo,
            MAX_TIME_P2,
        );
        part2_max_ans = max(part2_max_ans, total_pressure);
    }
    println!("Part 2: {}", part2_max_ans);
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
