use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet, HashMap};

mod line;

pub use crate::line::Line;

// Overcomplicated O(nlogn) but memory efficient solution which computes the overlapping points only
fn p1(line_vec: Vec<String>) -> i32 {
    // replace all the line's ' -> ' to ',' and then split with ','
    let new_line_vec: Vec<Vec<String>> = line_vec.iter()
        .map(|x| x.replace(" -> ", ",")
        .split(',')
        .map(|x| x.to_owned()).collect())
        .collect();
    // convert to Vec<Vec<i32>> and filter only horizontal or vertical lines
    let filtered_vec: Vec<Vec<i32>> = new_line_vec.iter()
        .map(|x| x.iter().map(|y| y.parse::<i32>().unwrap()).collect())
        .filter(|x: &Vec<i32>| x[0] == x[2] || x[1] == x[3])
        .collect();
    // convert to Vec<Line>
    let filtered_line_vec: Vec<Line> = filtered_vec.iter()
        .map(|x| Line::new(x[0], x[2], x[1], x[3]))
        .collect();
    let mut intersection_set: HashSet<(i32,i32)> = HashSet::new();
    for i in 0..(filtered_line_vec.len() - 1) {
        for j in (i + 1)..filtered_line_vec.len() {
            if filtered_line_vec[i].is_overlap(&filtered_line_vec[j]) {
                intersection_set = filtered_line_vec[i].add_intersections(&filtered_line_vec[j], intersection_set).unwrap();
            }
        }
    }
    intersection_set.len() as i32
}

// Naive "store all the points" solution
fn p2(line_vec: Vec<String>) -> i32 {
    let mut points: Vec<(i32, i32)> = Vec::new();
    // replace all the line's ' -> ' to ',' and then split with ','
    let new_line_vec: Vec<Vec<String>> = line_vec.iter()
        .map(|x| x.replace(" -> ", ",")
        .split(',')
        .map(|x| x.to_owned()).collect())
        .collect();
    // convert to Vec<Vec<i32>>
    let filtered_vec: Vec<Vec<i32>> = new_line_vec.iter()
        .map(|x| x.iter().map(|y| y.parse::<i32>().unwrap()).collect())
        .collect();
    // convert to Vec<Line>
    let line_obj_vec: Vec<Line> = filtered_vec.iter()
        .map(|x| Line::new(x[0], x[2], x[1], x[3]))
        .collect();
    // Add each point of every line to the points vec
    for line in line_obj_vec.iter() {
        points.append(&mut line.naive_compile_points());
    }
    // Convert the vec into a HashMap of frequency of how many times each points appears
    let mut freq = points.iter().copied().fold(HashMap::new(), |mut map, val|{
        map.entry(val).and_modify(|frq|*frq += 1).or_insert(1);
        map
    });
    // Remove points which only shows up less than 2 times
    freq.retain(|_, e| *e > 1);
    freq.len() as i32
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
