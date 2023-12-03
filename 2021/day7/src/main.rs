use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::min;

// Calculate the median
fn p1(line_vec: Vec<String>) -> i32 {
    let mut crabs: Vec<i32> = line_vec[0].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    crabs.sort_unstable();
    let median = (crabs[499] + crabs[500]) / 2;
    let mut answer = 0;
    for i in &crabs {
        answer += (i - median).abs();
    }
    answer
}

// Calculate the mean, and then check floor and ceil of the mean to see which is more optimal.
fn p2(line_vec: Vec<String>) -> i32 {
    let crabs: Vec<i32> = line_vec[0].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    let sum = crabs.to_vec().into_iter().sum::<i32>();
    let mean = sum as f64 / crabs.len() as f64;
    let (fmean, cmean): (i32, i32) = (mean.floor() as i32, mean.ceil() as i32);
    let (mut fmean_ans, mut cmean_ans): (i32, i32) = (0, 0);
    // check both ans
    for i in &crabs {
        let fdistance = (i - fmean).abs();
        let cdistance = (i - cmean).abs();
        fmean_ans += fdistance * (fdistance + 1) / 2;
        cmean_ans += cdistance * (cdistance + 1) / 2;
    }
    min(fmean_ans, cmean_ans)
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
