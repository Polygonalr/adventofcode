use std::{collections::HashMap, env, fs};

const TEST_FLAG: &str = "--test";
const INPUT_FILEPATH: &str = "./input.txt";
const TEST_FILEPATH: &str = "./test.txt";

#[derive(Debug)]
struct Path {
    left: String,
    right: String,
}

fn p1(line_vec: &[String]) -> i32 {
    let mut m: HashMap<String, Path> = HashMap::new();
    for line in line_vec.iter().skip(2) {
        let (path_name, path_str) = line.split_once(" = ").unwrap();
        let (l_s, r_s) = path_str.split_once(", ").unwrap();
        let (mut l_s, mut r_s) = (l_s.chars(), r_s.chars());
        l_s.next();
        r_s.next_back();
        let (l, s) = (l_s.collect::<String>(), r_s.collect::<String>());
        m.insert(path_name.to_string(), Path { left: l, right: s });
    }
    let nav = line_vec[0].chars().collect::<Vec<_>>();
    let mut nav_i = 0;
    let mut count = 0;
    let mut curr_node = "AAA".to_string();
    while true {
        count += 1;
        let paths = m.get(&curr_node).unwrap();
        curr_node = if nav[nav_i] == 'L' {
            paths.left.clone()
        } else {
            paths.right.clone()
        };
        if curr_node == "ZZZ" {
            return count;
        }
        nav_i = (nav_i + 1) % nav.len();
    }
    0
}

/// This is so silly, there's nothing in the problem statement that hints at
/// using LCM as the solution, and I have to go to Reddit for this idea
fn p2(line_vec: &[String]) -> i64 {
    let mut m: HashMap<String, Path> = HashMap::new();
    for line in line_vec.iter().skip(2) {
        let (path_name, path_str) = line.split_once(" = ").unwrap();
        let (l_s, r_s) = path_str.split_once(", ").unwrap();
        let (mut l_s, mut r_s) = (l_s.chars(), r_s.chars());
        l_s.next();
        r_s.next_back();
        let (l, s) = (l_s.collect::<String>(), r_s.collect::<String>());
        m.insert(path_name.to_string(), Path { left: l, right: s });
    }
    let nav = line_vec[0].chars().collect::<Vec<_>>();
    let curr_nodes = m
        .keys()
        .map(|x| x.clone())
        .filter(|x| x.chars().next_back().unwrap() == 'A')
        .collect::<Vec<_>>();
    let mut sol = 1;
    'out: for c in curr_nodes {
        println!("Going next");
        let mut nav_i = 0;
        let mut count = 0;
        let mut curr_node = c.clone();
        while true {
            let paths = m.get(&curr_node).unwrap();
            curr_node = if nav[nav_i] == 'L' {
                paths.left.clone()
            } else {
                paths.right.clone()
            };
            count += 1;
            if curr_node.chars().next_back().unwrap() == 'Z' {
                sol = num::integer::lcm(sol, count);
                continue 'out;
            }
            nav_i = (nav_i + 1) % nav.len();
        }
    }
    sol
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
    // println!("Part 1: {}\nPart 2: {}", p1(&line_vec), p2(&line_vec));
    println!("Part 2: {}", p2(&line_vec));
}
