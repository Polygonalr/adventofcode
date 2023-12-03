use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

fn p1(line_vec: Vec<String>) -> i32 {
    const NUMBER_OF_DAYS: usize = 80;
    // queue holds the number of fishes in the given stage of their birth cycle
    let mut queue: VecDeque<usize> = VecDeque::from([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let starting_fishes:Vec<&str> = line_vec[0].split(',').collect();
    for fish in starting_fishes.iter() {
        queue[fish.parse::<usize>().unwrap()] += 1;
    }
    // for every day, dequeue the no. of fishes which are due for birth,
    // add the newborn fishes to stage 8, and finally add the fishes which just
    // gave birth back to the stage 6.
    for _ in 0..NUMBER_OF_DAYS {
        let birthing = queue.pop_front().unwrap();
        queue.push_back(birthing);
        queue[6] += birthing;
    }
    queue.iter().sum::<usize>() as i32
}

// p1 except I changed everything to i64 and set the number of days to 256
fn p2(line_vec: Vec<String>) -> i64 {
    const NUMBER_OF_DAYS: usize = 256;
    let mut queue: VecDeque<i64> = VecDeque::from([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let starting_fishes:Vec<&str> = line_vec[0].split(',').collect();
    for fish in starting_fishes.iter() {
        queue[fish.parse::<usize>().unwrap()] += 1;
    }
    for _ in 0..NUMBER_OF_DAYS {
        let birthing = queue.pop_front().unwrap();
        queue.push_back(birthing);
        queue[6] += birthing;
    }
    queue.iter().sum::<i64>()
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
    let line_vec2 = line_vec.to_vec();
    println!("Part 1: {}\nPart 2: {}", p1(line_vec), p2(line_vec2));
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
