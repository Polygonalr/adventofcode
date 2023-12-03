use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn p1(line_vec: Vec<String>) -> i32 {
    let mut set_bit_count: [i32; 12] = [0; 12];
    // log down number of set bits for each bit position
    for entry in line_vec.iter() {
        let mut chars = entry.chars();
        for i in &mut set_bit_count { 
            if chars.next() == Some('1') {
                *i += 1;
            }
        }
    }

    // compute gamma
    // theoratically I'll need only 1 value and bit-invert it to get the other,
    // but this is easier to do.
    let mut gamma_str = "".to_owned();
    let mut delta_str = "".to_owned();
    for b in set_bit_count.iter() {
        if *b > line_vec.len() as i32 / 2 {
            gamma_str.push('1');
            delta_str.push('0');
        } else {
            gamma_str.push('0');
            delta_str.push('1');
        }
    }
    let gamma = isize::from_str_radix(&gamma_str, 2).unwrap();
    let delta = isize::from_str_radix(&delta_str, 2).unwrap();
    (gamma * delta) as i32
}

// Simple binary search problem
fn p2(line_vec: Vec<String>) -> i32 {
    let mut curr_vec: Vec<String> = line_vec.to_vec();
    // Solve for O2
    for i in 0..12 {
        if (count_set_bits(i, curr_vec.to_vec()) as f64) >= curr_vec.len() as f64 / 2_f64 {
            curr_vec = filter_vec(curr_vec.to_vec(), i, '1');
        } else {
            curr_vec = filter_vec(curr_vec.to_vec(), i, '0');
        }
        if curr_vec.len() == 1 {
            break;
        } else if curr_vec.is_empty() {
            panic!("No solution found");
        }
    }
    let o2 = i32::from_str_radix(&curr_vec[0], 2).unwrap();

    // Solve for CO2
    curr_vec = line_vec.to_vec();
    for i in 0..12 {
        if (count_set_bits(i, curr_vec.to_vec()) as f64) < curr_vec.len() as f64 / 2_f64 {
            curr_vec = filter_vec(curr_vec.to_vec(), i, '1');
        } else {
            curr_vec = filter_vec(curr_vec.to_vec(), i, '0');
        }
        if curr_vec.len() == 1 {
            break;
        } else if curr_vec.is_empty() {
            panic!("No solution found");
        }
    }
    let co2 = i32::from_str_radix(&curr_vec[0], 2).unwrap();
    (o2 * co2) as i32
}

// Count the num of set bits in the nth digit of the binary representation of numbers in a vector
fn count_set_bits(n: usize, binary_vec: Vec<String>) -> i32 {
    let mut count = 0;
    for binary_str in binary_vec.iter() {
        if binary_str.chars().nth(n).unwrap() == '1' {
            count += 1;
        }
    }
    count
}

// Remove strings from a vector that do not have specified character c in the nth place
fn filter_vec(line_vec: Vec<String>, n: usize, c: char) -> Vec<String> {
    let mut filtered_vec: Vec<String> = Vec::new();
    for entry in line_vec.iter() {
        if entry.chars().nth(n).unwrap() == c {
            filtered_vec.push(entry.to_owned());
        }
    }
    if filtered_vec.is_empty() {
        return line_vec;
    }
    filtered_vec
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
