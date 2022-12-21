use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rayon::prelude::*;

const TEST_FLAG: &str = "--test";
const TEST_FILEPATH: &str = "./test.txt";
const INPUT_FILEPATH: &str = "./input.txt";
const TIME_LIMIT: i32 = 24;
struct Blueprint {
    ore_cost: i32, // in ores
    clay_cost: i32, // in ores
    obs_ore_cost: i32,
    obs_clay_cost: i32,
    geode_ore_cost: i32,
    geode_obs_cost: i32
}

struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32
}

struct Robots {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32
}

fn p1(blueprints: &Vec<Blueprint>) -> i32 {
    0
}

fn p2(blueprints: &Vec<Blueprint>) -> i32 {
    0
}

fn main() {
    let mut filepath = INPUT_FILEPATH;
    if env::args().collect::<Vec<_>>().contains(&TEST_FLAG.to_string()) {
        filepath = TEST_FILEPATH;
    }
    // let mut str_buf = "".to_owned();
    let mut blueprints: Vec<Blueprint> = vec![];
    if let Ok(lines) = read_lines(filepath) {
        for line in lines.flatten() {
            let digits = line.chars()
                    .filter(|c| c.is_ascii_digit() || c.is_ascii_whitespace())
                    .collect::<String>()
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
            if let [ore_cost, clay_cost, obs_ore_cost, obs_clay_cost, geode_ore_cost, geode_obs_cost] = digits[..] {
                blueprints.push(Blueprint {
                    ore_cost,
                    clay_cost,
                    obs_ore_cost,
                    obs_clay_cost,
                    geode_ore_cost,
                    geode_obs_cost
                })
            }
        }
    }
    println!("Part 1: {}\nPart 2: {}", p1(&blueprints), p2(&blueprints));
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
