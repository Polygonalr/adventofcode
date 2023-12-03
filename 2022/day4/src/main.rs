use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn p1(line_vec: Vec<String>) -> i32 {
    let mut count = 0;
    for line in line_vec {
        let mut s = line.split(',');
        let mut first = s.next().unwrap().split('-');
        let f_start: i32 = first.next().unwrap().parse().unwrap();
        let f_end: i32 = first.next().unwrap().parse().unwrap();
        let mut second = s.next().unwrap().split('-');
        let s_start: i32 = second.next().unwrap().parse().unwrap();
        let s_end: i32 = second.next().unwrap().parse().unwrap();
        // count full overlaps
        if (f_start <= s_start && f_end >= s_end) || (s_start <= f_start && s_end >= f_end) {
            count += 1;
        }
    }
    count
}

fn p2(line_vec: Vec<String>) -> i32 {
    let mut count = 0;
    for line in line_vec {
        let mut s = line.split(',');
        let mut first = s.next().unwrap().split('-');
        let f_start: i32 = first.next().unwrap().parse().unwrap();
        let f_end: i32 = first.next().unwrap().parse().unwrap();
        let mut second = s.next().unwrap().split('-');
        let s_start: i32 = second.next().unwrap().parse().unwrap();
        let s_end: i32 = second.next().unwrap().parse().unwrap();
        // count partial overlaps
        if f_start == s_end || s_start == f_end || f_start == s_start || f_end == s_end
                || (f_start < s_start && f_end >= s_start)
                || (s_start < f_start && s_end >= f_start) {
            count += 1;
        }
    }
    count
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
