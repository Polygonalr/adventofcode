use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Easy O(n) solution.
fn part1() {
    let filepath = "./input.txt";
    let mut prev_num = 0;
    let mut counter = 0;
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(num) = line.unwrap().parse::<i32>() {
                if num > prev_num {
                    counter += 1;
                }
                prev_num = num;
            }
        }
    }
    println!("{}", counter - 1);
}

// O(n) sliding window solution.
fn part2() {
    let filepath = "./input.txt";
    let mut num_buffer: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(num) = line.unwrap().parse::<i32>() {
                num_buffer.push(num);
            }
        }
    }
    let mut prev_sum = num_buffer[0] + num_buffer[1] + num_buffer[2];
    let mut curr_sum = prev_sum;
    let mut counter = 0;
    for i in 3..num_buffer.len() {
        curr_sum = curr_sum - num_buffer[i - 3] + num_buffer[i];
        if curr_sum > prev_sum {
            counter += 1;
        }
        prev_sum = curr_sum;
    }
    println!("{}", counter);
}

fn main() {
    part1();
    part2();
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
