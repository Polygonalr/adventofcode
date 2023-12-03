use regex::Regex;
use std::{env, fs};

const TEST_FLAG: &str = "--test";
const INPUT_FILEPATH: &str = "./input.txt";
const TEST_FILEPATH: &str = "./test.txt";

fn p1(line_vec: &[String]) -> i64 {
    line_vec
        .iter()
        .map(|x| {
            let c: Vec<char> = x.chars().collect();
            let mut owned: String = "".to_owned();
            owned.push(c[0]);
            owned.push(c[c.len() - 1]);
            owned.parse::<i64>().unwrap()
        })
        .sum()
}

fn p2(line_vec: &[String]) -> i64 {
    let rgx =
        Regex::new(r"one|two|three|four|five|six|seven|eight|nine|zero|[0-9]").expect("invalid");
    let rgx_reversed =
        Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|orez|[0-9]").expect("invalid");
    return line_vec
        .iter()
        .map(|x| {
            let (s1, s2) = (x.clone(), x.clone());
            let match1 = s1.matches(&rgx).next().unwrap();
            let s2c = s2.chars().rev().collect::<String>();
            let match2 = s2c.matches(&rgx_reversed).next().unwrap();
            let match1_val = match match1 {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                "zero" => 0,
                _ => match1.parse::<i64>().unwrap(),
            };
            let match2_val = match match2 {
                "eno" => 1,
                "owt" => 2,
                "eerht" => 3,
                "ruof" => 4,
                "evif" => 5,
                "xis" => 6,
                "neves" => 7,
                "thgie" => 8,
                "enin" => 9,
                "orez" => 0,
                _ => match2.parse::<i64>().unwrap(),
            };
            match1_val * 10 + match2_val
        })
        .sum();
}

fn main() {
    let filepath = if env::args().any(|x| x == *TEST_FLAG) {
        TEST_FILEPATH
    } else {
        INPUT_FILEPATH
    };
    let input = fs::read_to_string(filepath).unwrap();
    let mut line_vec1: Vec<String> = Vec::new();
    let mut line_vec2: Vec<String> = Vec::new();
    for line in input.lines() {
        // Process each line...
        line_vec2.push(line.to_string());

        // Use the following if only need to process digits
        let to_push = line
            .chars()
            .filter(|c| c.is_ascii_digit() || c.is_ascii_whitespace())
            .collect::<String>();
        line_vec1.push(to_push);
    }
    println!("Part 1: {}\nPart 2: {}", p1(&line_vec1), p2(&line_vec2));
}
