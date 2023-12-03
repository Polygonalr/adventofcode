use std::{env, fs};

const TEST_FLAG: &str = "--test";
const INPUT_FILEPATH: &str = "./input.txt";
const TEST_FILEPATH: &str = "./test.txt";

const ROCK_TYPES: &'static [&'static (i32, i32)] = [
    [
        (0,0),
        (1,0),
        (2,0),
        (3,0),
    ],
    [
        (1,0),
        (0,1),
        (1,1),
        (2,1),
        (1,2),
    ],
    [
        (0,0),
        (1,0),
        (2,0),
        (2,1),
        (2,2),
    ],
    [
        (0,0),
        (0,1),
        (0,2),
        (0,3),
    ],
    [
        (0,0),
        (0,1),
        (1,0),
        (1,1),
    ]
];

fn p1(line_vec: &[String]) -> i32 {
    0
}

fn p2(line_vec: &[String]) -> i32 {
    0
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
