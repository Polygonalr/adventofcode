use std::{env, fs};

const TEST_FLAG: &str = "--test";
const INPUT_FILEPATH: &str = "./input.txt";
const TEST_FILEPATH: &str = "./test.txt";

fn p1(line_vec: &[String]) -> i32 {
    let mut res = 1;
    let times = line_vec[0]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let distances = line_vec[1]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    for i in 0..times.len() {
        let (time, distance) = (times[i], distances[i]);
        println!("{:?} {:?}", time, distance);
        // do a half-ass double-ended search. Starting from the front.
        let mut curr_earliest_start: i32 = 0;
        for hold_time in 1..time + 1 {
            let curr_dist = hold_time * (time - hold_time);
            if curr_dist > distance {
                curr_earliest_start = hold_time;
                break;
            }
        }
        // Starting from the end
        let mut curr_latest_start: i32 = time + 1;
        for hold_time in (curr_earliest_start..time).rev() {
            let curr_dist = hold_time * (time - hold_time);
            if curr_dist > distance {
                curr_latest_start = hold_time;
                break;
            }
        }
        // if curr_earliest_start == 0 || curr_latest_start == time + 1 {
        //     continue;
        // }
        res *= curr_latest_start - curr_earliest_start + 1;
    }
    res
}

fn p2(line_vec: &[String]) -> i64 {
    let time_str: String = line_vec[0].chars().filter(|c| !c.is_whitespace()).collect();
    println!("{}", time_str);
    let time = time_str.parse::<i64>().unwrap();
    let distance_str: String = line_vec[1].chars().filter(|c| !c.is_whitespace()).collect();
    println!("{}", distance_str);
    let distance = distance_str.parse::<i64>().unwrap();

    // do a half-ass double-ended search. Starting from the front.
    let mut curr_earliest_start: i64 = 0;
    for hold_time in 1..time + 1 {
        let curr_dist = hold_time * (time - hold_time);
        if curr_dist > distance {
            curr_earliest_start = hold_time;
            break;
        }
    }
    // Starting from the end
    let mut curr_latest_start: i64 = time + 1;
    for hold_time in (curr_earliest_start..time).rev() {
        let curr_dist = hold_time * (time - hold_time);
        if curr_dist > distance {
            curr_latest_start = hold_time;
            break;
        }
    }
    curr_latest_start - curr_earliest_start + 1
}

fn main() {
    let filepath = if env::args().any(|x| x == *TEST_FLAG) {
        TEST_FILEPATH
    } else {
        INPUT_FILEPATH
    };
    let input = fs::read_to_string(filepath).unwrap();
    let mut line_vec: Vec<String> = Vec::new();
    for line in input.lines() {
        // Process each line...
        // line_vec.push(line.to_string());

        // Use the following if only need to process digits
        let to_push = line
            .chars()
            .filter(|c| c.is_ascii_digit() || c.is_ascii_whitespace())
            .collect::<String>();
        line_vec.push(to_push);
    }
    println!("Part 1: {}\nPart 2: {}", p1(&line_vec), p2(&line_vec));
}
