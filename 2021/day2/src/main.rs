use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn p1(line_vec: Vec<String>) -> i32 {
    let (mut x, mut y) = (0, 0);
    for step in line_vec.iter() {
        let mut p = step.split(' ');
        match p.next().unwrap() {
            "forward" => {
                x += p.next().unwrap().parse::<i32>().unwrap();
            },
            "down" => {
                y += p.next().unwrap().parse::<i32>().unwrap();
            },
            "up" => {
                y -= p.next().unwrap().parse::<i32>().unwrap();
            },
            _ => {}
        }
    }
    x * y
}

fn p2(line_vec: Vec<String>) -> i32 {
    let (mut x, mut y, mut aim) = (0, 0, 0);
    for step in line_vec.iter() {
        let mut p = step.split(' ');
        match p.next().unwrap() {
            "forward" => {
                let f = p.next().unwrap().parse::<i32>().unwrap();
                x += f;
                y += aim * f;
            },
            "down" => {
                aim += p.next().unwrap().parse::<i32>().unwrap();
            },
            "up" => {
                aim -= p.next().unwrap().parse::<i32>().unwrap();
            },
            _ => {}
        }
    }
    x * y
}

fn main() {
    let filepath = "./input.txt";
    // let mut str_buf = "".to_owned();
    let mut line_vec: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines.flatten() {
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
