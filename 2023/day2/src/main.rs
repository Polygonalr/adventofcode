use itertools::Itertools;
use std::{env, fs};

const TEST_FLAG: &str = "--test";
const INPUT_FILEPATH: &str = "./input.txt";
const TEST_FILEPATH: &str = "./test.txt";

fn p1(line_vec: &[String]) -> i32 {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;
    let mut res = 0;
    'line_iter: for line in line_vec {
        let t = line.split(": ").collect::<Vec<_>>();
        let (_, gid) = t[0].splitn(2, ' ').collect_tuple().unwrap();
        let sets = t[1];
        for set in sets.split("; ") {
            for typ in set.split(", ") {
                let (num, color) = typ.splitn(2, ' ').collect_tuple().unwrap();
                let num = num.parse::<i32>().unwrap();
                match color {
                    "red" => {
                        if num > MAX_RED {
                            continue 'line_iter;
                        }
                    }
                    "green" => {
                        if num > MAX_GREEN {
                            continue 'line_iter;
                        }
                    }
                    "blue" => {
                        if num > MAX_BLUE {
                            continue 'line_iter;
                        }
                    }
                    _ => {
                        panic!("Unexpected");
                    }
                }
            }
        }
        res += gid.parse::<i32>().unwrap();
    }
    res
}

fn p2(line_vec: &[String]) -> i32 {
    let mut res = 0;
    for line in line_vec {
        let t = line.split(": ").collect::<Vec<_>>();
        let (mut r, mut g, mut b) = (0, 0, 0);
        let sets = t[1];
        for set in sets.split("; ") {
            for typ in set.split(", ") {
                let (num, color) = typ.splitn(2, ' ').collect_tuple().unwrap();
                let num = num.parse::<i32>().unwrap();
                match color {
                    "red" => {
                        r = std::cmp::max(r, num);
                    }
                    "green" => {
                        g = std::cmp::max(g, num);
                    }
                    "blue" => {
                        b = std::cmp::max(b, num);
                    }
                    _ => {
                        panic!("Unexpected");
                    }
                }
            }
        }
        res += r * g * b;
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
        line_vec.push(line.to_string());

        // Use the following if only need to process digits
        // let to_push = line
        //     .chars()
        //     .filter(|c| c.is_ascii_digit() || c.is_ascii_whitespace())
        //     .collect::<String>();
        // line_vec.push(to_push);
    }
    println!("Part 1: {}\nPart 2: {}", p1(&line_vec), p2(&line_vec));
}
