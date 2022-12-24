use rayon::prelude::*;
use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const TEST_FLAG: &str = "--test";
const TEST_FILEPATH: &str = "./test.txt";
const INPUT_FILEPATH: &str = "./input.txt";
const TIME_LIMIT_P1: i32 = 24;
const TIME_LIMIT_P2: i32 = 32;

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    ore_cost: i32,  // in ores
    clay_cost: i32, // in ores
    obs_ore_cost: i32,
    obs_clay_cost: i32,
    geode_ore_cost: i32,
    geode_obs_cost: i32,
}

#[derive(Debug, Clone, Copy)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

#[derive(Debug, Clone, Copy)]
struct Robots {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

// store what robots we skipped the previous tick
// this is to prevent building a robot that we could have built earlier
#[derive(Debug, Clone, Copy)]
struct TickState<'a> {
    blueprint: &'a Blueprint,
    resources: &'a Resources,
}

impl TickState<'_> {
    fn can_build_ore_robot(&self) -> bool {
        self.resources.ore >= self.blueprint.ore_cost
    }
    fn can_build_clay_robot(&self) -> bool {
        self.resources.ore >= self.blueprint.clay_cost
    }
    fn can_build_obsidian_robot(&self) -> bool {
        self.resources.ore >= self.blueprint.obs_ore_cost
            && self.resources.clay >= self.blueprint.obs_clay_cost
    }
    fn can_build_geode_robot(&self) -> bool {
        self.resources.obsidian >= self.blueprint.geode_obs_cost
            && self.resources.ore >= self.blueprint.geode_ore_cost
    }
}

fn pass_tick(resources: &Resources, robots: &Robots) -> Resources {
    Resources {
        ore: resources.ore + robots.ore,
        clay: resources.clay + robots.clay,
        obsidian: resources.obsidian + robots.obsidian,
        geode: resources.geode + robots.geode,
    }
}

fn build_ore_robot(ore_cost: i32, resources: &Resources, robots: &Robots) -> (Resources, Robots) {
    let mut new_resources = *resources;
    new_resources.ore -= ore_cost;
    let mut new_robots = *robots;
    new_robots.ore += 1;
    (new_resources, new_robots)
}

fn build_clay_robot(ore_cost: i32, resources: &Resources, robots: &Robots) -> (Resources, Robots) {
    let mut new_resources = *resources;
    new_resources.ore -= ore_cost;
    let mut new_robots = *robots;
    new_robots.clay += 1;
    (new_resources, new_robots)
}

fn build_obsidian_robot(
    ore_cost: i32,
    clay_cost: i32,
    resources: &Resources,
    robots: &Robots,
) -> (Resources, Robots) {
    let mut new_resources = *resources;
    new_resources.ore -= ore_cost;
    new_resources.clay -= clay_cost;
    let mut new_robots = *robots;
    new_robots.obsidian += 1;
    (new_resources, new_robots)
}

fn build_geode_robot(
    ore_cost: i32,
    obsidian_cost: i32,
    resources: &Resources,
    robots: &Robots,
) -> (Resources, Robots) {
    let mut new_resources = *resources;
    new_resources.ore -= ore_cost;
    new_resources.obsidian -= obsidian_cost;
    let mut new_robots = *robots;
    new_robots.geode += 1;
    (new_resources, new_robots)
}

fn p1_solve(
    blueprint: &Blueprint,
    resources: &Resources,
    robots: &Robots,
    time: i32,
    time_limit: i32,
    skipped_robots: [bool; 4],
) -> i32 {
    let new_resources = pass_tick(resources, robots);
    let tick_state = TickState {
        blueprint,
        resources,
    };
    let current_time = time + 1;
    // exit condition
    if current_time == time_limit {
        return new_resources.geode;
    }

    // case 1: skip building robots
    // we need to indicate the next tick that we are skipping robots that
    // we can build so that we don't waste time building them in the next tick
    let new_skipped_robots: [bool; 4] = [
        tick_state.can_build_clay_robot(),
        tick_state.can_build_ore_robot(),
        tick_state.can_build_obsidian_robot(),
        tick_state.can_build_geode_robot(),
    ];
    let mut best_geode_count = p1_solve(
        blueprint,
        &new_resources,
        robots,
        current_time,
        time_limit,
        new_skipped_robots,
    );
    // case 2: build clay robot
    // there is no point building more clay robots than how much we can spend per minute
    // there is also no point building the robot in the final tick we can build the robot
    if tick_state.can_build_clay_robot()
        && !skipped_robots[0]
        && robots.clay < blueprint.obs_clay_cost
        && time != time_limit - 1
    {
        let (temp_resources, temp_robots) =
            build_clay_robot(blueprint.clay_cost, &new_resources, robots);
        best_geode_count = max(
            best_geode_count,
            p1_solve(
                blueprint,
                &temp_resources,
                &temp_robots,
                current_time,
                time_limit,
                [false; 4],
            ),
        );
    }
    // case 3: build ore robot
    // similarly, there is no more building more ore robots than how much we can spend per minute
    // also similarly, there is no point building the robot in the final tick we can build the robot
    if tick_state.can_build_ore_robot()
        && !skipped_robots[1]
        && robots.ore
            < max(
                max(blueprint.clay_cost, blueprint.ore_cost),
                max(blueprint.geode_ore_cost, blueprint.obs_ore_cost),
            )
        && time != time_limit - 1
    {
        let (temp_resources, temp_robots) =
            build_ore_robot(blueprint.ore_cost, &new_resources, robots);
        best_geode_count = max(
            best_geode_count,
            p1_solve(
                blueprint,
                &temp_resources,
                &temp_robots,
                current_time,
                time_limit,
                [false; 4],
            ),
        );
    }
    // case 4: build obsidian robot
    // likewise as above
    if tick_state.can_build_obsidian_robot()
        && !skipped_robots[2]
        && robots.obsidian < blueprint.geode_obs_cost
        && time != time_limit - 1
    {
        let (temp_resources, temp_robots) = build_obsidian_robot(
            blueprint.obs_ore_cost,
            blueprint.obs_clay_cost,
            &new_resources,
            robots,
        );
        best_geode_count = max(
            best_geode_count,
            p1_solve(
                blueprint,
                &temp_resources,
                &temp_robots,
                current_time,
                time_limit,
                [false; 4],
            ),
        );
    }
    // case 5: build geode robot
    // unlike the other robots, we can still build the geode robot in the final tick
    if tick_state.can_build_geode_robot() && !skipped_robots[3] {
        let (temp_resources, temp_robots) = build_geode_robot(
            blueprint.geode_ore_cost,
            blueprint.geode_obs_cost,
            &new_resources,
            robots,
        );
        best_geode_count = max(
            best_geode_count,
            p1_solve(
                blueprint,
                &temp_resources,
                &temp_robots,
                current_time,
                time_limit,
                [false; 4],
            ),
        );
    }
    best_geode_count
}

fn p1(blueprints: &[Blueprint]) -> i32 {
    let initial_resource_state = Resources {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    let initial_robot_state = Robots {
        ore: 1,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    let highest_geodes = blueprints
        .par_iter()
        .map(|b| {
            p1_solve(
                b,
                &initial_resource_state,
                &initial_robot_state,
                0,
                TIME_LIMIT_P1,
                [false; 4],
            )
        })
        .collect::<Vec<_>>();
    highest_geodes
        .iter()
        .enumerate()
        .map(|(i, &v)| (i + 1) as i32 * v)
        .sum()
}

fn p2(blueprints: &[Blueprint]) -> i32 {
    let initial_resource_state = Resources {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    let initial_robot_state = Robots {
        ore: 1,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    let new_blueprints = &blueprints[..3];
    let highest_geodes = new_blueprints
        .par_iter()
        .map(|b| {
            p1_solve(
                b,
                &initial_resource_state,
                &initial_robot_state,
                0,
                TIME_LIMIT_P2,
                [false; 4],
            )
        })
        .collect::<Vec<_>>();
    highest_geodes.iter().product()
}

fn main() {
    let mut filepath = INPUT_FILEPATH;
    if env::args().any(|x| x == *TEST_FLAG) {
        filepath = TEST_FILEPATH;
    }
    // let mut str_buf = "".to_owned();
    let mut blueprints: Vec<Blueprint> = vec![];
    if let Ok(lines) = read_lines(filepath) {
        for line in lines.flatten() {
            let digits = line
                .chars()
                .filter(|c| c.is_ascii_digit() || c.is_ascii_whitespace())
                .collect::<String>()
                .split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            if let [_, ore_cost, clay_cost, obs_ore_cost, obs_clay_cost, geode_ore_cost, geode_obs_cost] =
                digits[..]
            {
                blueprints.push(Blueprint {
                    ore_cost,
                    clay_cost,
                    obs_ore_cost,
                    obs_clay_cost,
                    geode_ore_cost,
                    geode_obs_cost,
                })
            }
        }
    }
    println!("Part 1: {}", p1(&blueprints));
    println!("Part 2: {}", p2(&blueprints));
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
