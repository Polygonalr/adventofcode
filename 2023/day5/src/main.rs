// TLDR part 2 has a lot of interval maths

use itertools::Itertools;
use std::{collections::VecDeque, env, fs};

const TEST_FLAG: &str = "--test";
const INPUT_FILEPATH: &str = "./input.txt";
const TEST_FILEPATH: &str = "./test.txt";

fn p1(line_vec: &[String]) -> i64 {
    // process seeds to find
    let mut seeds: Vec<i64> = line_vec[0]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    seeds.sort();
    let mut seed_map: Vec<i64> = seeds.to_vec();
    let mut curr_line_idx = 3;

    while curr_line_idx < line_vec.len() {
        let mut seed_to_soil_map: Vec<(i64, i64, i64)> = vec![];
        while curr_line_idx < line_vec.len() && !line_vec[curr_line_idx].is_empty() {
            let (dst, src, rng): (i64, i64, i64) = line_vec[curr_line_idx]
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();

            seed_to_soil_map.push((src, dst, rng));
            curr_line_idx += 1;
        }
        seed_to_soil_map.sort();

        // process the ranges
        let mut seed_idx = 0;
        let mut map_idx = 0;
        while map_idx < seed_to_soil_map.len() && seed_idx < seed_map.len() {
            let mapping = seed_to_soil_map[map_idx];
            let from = mapping.0;
            let to = mapping.0 + mapping.2 - 1;
            if seed_map[seed_idx] >= from && seed_map[seed_idx] <= to {
                seed_map[seed_idx] = mapping.1 + (seed_map[seed_idx] - mapping.0);
                seed_idx += 1;
            } else if seed_map[seed_idx] < from {
                seed_idx += 1;
            } else {
                map_idx += 1;
            }
        }
        seed_map.sort();
        curr_line_idx += 2;
    }
    seed_map[0]
}

// i actually hate that this is so tedious, god damn interval arithmetics
fn p2(line_vec: &[String]) -> i64 {
    let seeds: Vec<i64> = line_vec[0]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut res: i64 = i64::MAX;

    for i in 0..seeds.len() / 2 {
        let mut range_map: VecDeque<(i64, i64)> = VecDeque::new();
        range_map.push_back((seeds[i * 2], seeds[i * 2] + seeds[i * 2 + 1] - 1));

        let mut curr_line_idx = 3;

        while curr_line_idx < line_vec.len() {
            let mut seed_to_soil_map: Vec<(i64, i64, i64)> = vec![];
            while curr_line_idx < line_vec.len() && !line_vec[curr_line_idx].is_empty() {
                let (dst, src, rng): (i64, i64, i64) = line_vec[curr_line_idx]
                    .split(' ')
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap();

                seed_to_soil_map.push((src, dst, rng));
                curr_line_idx += 1;
            }
            seed_to_soil_map.sort();

            // process the ranges
            let mut range_idx = 0;
            let mut map_idx = 0;
            while map_idx < seed_to_soil_map.len() && range_idx < range_map.len() {
                let mapping = seed_to_soil_map[map_idx];
                let (from, to) = (mapping.0, mapping.0 + mapping.2 - 1);
                let (r_from, r_to) = (range_map[range_idx].0, range_map[range_idx].1);
                if range_map[range_idx].1 < from {
                    range_idx += 1;
                    continue;
                } else if range_map[range_idx].0 > to {
                    map_idx += 1;
                    continue;
                }

                if range_map[range_idx].0 >= from {
                    if range_map[range_idx].1 <= to {
                        // map both
                        range_map[range_idx] =
                            (mapping.1 + r_from - mapping.0, mapping.1 + r_to - mapping.0);
                        range_idx += 1;
                    } else {
                        // slice the range into 2, repeat
                        range_map[range_idx] =
                            (mapping.1 + r_from - mapping.0, mapping.1 + mapping.2 - 1);
                        range_map.insert(range_idx + 1, (to + 1, r_to));
                        range_idx += 1;
                    }
                } else if range_map[range_idx].1 <= to {
                    // map both case handled, so just slice range into 2 and repeat
                    range_map[range_idx] = (mapping.1, mapping.1 + r_to - mapping.0);
                    range_map.insert(range_idx + 1, (r_from, from - 1));
                    range_idx += 1;
                } else {
                    // range provided is within map range, slice into 3 and repeat
                    range_map[range_idx] = (r_from, mapping.0 - 1);
                    range_map.insert(range_idx + 1, (mapping.1, mapping.1 + mapping.2 - 1));
                    range_map.insert(range_idx + 2, (mapping.0 + mapping.2, r_to));
                    range_idx += 2;
                }
            }
            range_map.make_contiguous().sort();
            curr_line_idx += 2;
        }
        let curr_best = range_map.front().unwrap().0;
        res = std::cmp::min(curr_best, res);
    }
    res
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
