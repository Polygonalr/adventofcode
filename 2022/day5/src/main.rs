use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

const NUMBER_OF_STACKS: usize = 9;

fn p1(line_vec: Vec<String>) -> String {
    let mut stacks: [VecDeque<char>; NUMBER_OF_STACKS] = Default::default();
    let mut init: bool = false;
    for line in line_vec {
        if !init {
            if line.starts_with(" 1") {
                init = true;
                continue;
            }
            // split line into equal chunks
            let non_letter = [' ', '[', ']'];
            for (i, c) in line.chars().enumerate() {
                if !non_letter.contains(&c) {
                    stacks[(i - 1) / 4].push_front(c);
                }
            }
        } else if line != "" {
            if let [_a, b, _c, s, _d, d] = line.split(' ').collect::<Vec<_>>()[..] {
                let n = b.parse::<i32>().unwrap();
                let source = s.parse::<usize>().unwrap() - 1;
                let dest = d.parse::<usize>().unwrap() - 1;
                for _ in 0..n {
                    let to_push = stacks[source].pop_back().unwrap();
                    stacks[dest].push_back(to_push);
                }
            }
            else {
                panic!("Invalid instruction");
            }
        }
    }
    let mut result = "".to_owned();
    for stack in stacks.iter() {
        result.push(*stack.back().unwrap());
    }
    result
}

fn p2(line_vec: Vec<String>) -> String {
    let mut stacks: [VecDeque<char>; NUMBER_OF_STACKS] = Default::default();
    let mut init: bool = false;
    for line in line_vec {
        if !init {
            if line.starts_with(" 1") {
                init = true;
                continue;
            }
            // split line into equal chunks
            let non_letter = [' ', '[', ']'];
            for (i, c) in line.chars().enumerate() {
                if !non_letter.contains(&c) {
                    stacks[(i - 1) / 4].push_front(c);
                }
            }
        } else if line != "" {
            if let [_a, b, _c, s, _d, d] = line.split(' ').collect::<Vec<_>>()[..] {
                let n = b.parse::<i32>().unwrap();
                let source = s.parse::<usize>().unwrap() - 1;
                let dest = d.parse::<usize>().unwrap() - 1;
                let mut temp: Vec<char> = vec![];
                for _ in 0..n {
                    let to_push = stacks[source].pop_back().unwrap();
                    temp.push(to_push);
                }
                temp.reverse();
                for c in temp {
                    stacks[dest].push_back(c);
                }
            }
            else {
                panic!("Invalid instruction");
            }
        }
    }
    let mut result = "".to_owned();
    for stack in stacks.iter() {
        result.push(*stack.back().unwrap());
    }
    result
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
