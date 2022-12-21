use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn p1(line_vec: Vec<String>) -> i64 {
    // (original position, value)
    let mut original_order: Vec<i64> = vec![];
    let mut message: Vec<(usize, i64)> = Vec::new();
    for (i, line) in line_vec.iter().enumerate() {
        let v = line.parse::<i64>().unwrap();
        original_order.push(v);
        message.push((i, v));
    }
    for (i, &val) in original_order.iter().enumerate() {
        let curr_pos = message.iter().position(|&r| r == (i, val)).unwrap();
        let mut new_pos = val + curr_pos as i64;
        new_pos = new_pos.rem_euclid(message.len() as i64 - 1);
        message.remove(curr_pos);
        message.insert(new_pos as usize, (i, val));
        // println!("{:?}", message.iter().map(|e| e.1).collect::<Vec<_>>());
    }

    let zero_pos = message.iter().position(|&r| r.1 == 0).unwrap();
    let positions: Vec<usize> = vec![zero_pos + 1000, zero_pos + 2000, zero_pos + 3000]
        .iter()
        .map(|&v| v.rem_euclid(original_order.len()))
        .collect();

    let mut sum = 0;
    for pos in positions {
        sum += message[pos].1;
    }
    sum
}

/**
 * Same as p1, except multiplying all the entries by the decryption_key
 * and then mixing it 10 times
 */
fn p2(line_vec: Vec<String>) -> i64 {
    // (original position, value)
    let mut original_order: Vec<i64> = vec![];
    let mut message: Vec<(usize, i64)> = Vec::new();
    for (i, line) in line_vec.iter().enumerate() {
        let v = line.parse::<i64>().unwrap() * 811589153;
        original_order.push(v);
        message.push((i, v));
    }

    for _ in 0..10 {
        for (i, &val) in original_order.iter().enumerate() {
            let curr_pos = message.iter().position(|&r| r == (i, val)).unwrap();
            let mut new_pos = val + curr_pos as i64;
            new_pos = new_pos.rem_euclid(message.len() as i64 - 1);
            message.remove(curr_pos);
            message.insert(new_pos as usize, (i, val));
            // println!("{:?}", message.iter().map(|e| e.1).collect::<Vec<_>>());
        }
    }

    let zero_pos = message.iter().position(|&r| r.1 == 0).unwrap();
    let positions: Vec<usize> = vec![zero_pos + 1000, zero_pos + 2000, zero_pos + 3000]
        .iter()
        .map(|&v| v.rem_euclid(original_order.len()))
        .collect();
    let mut sum = 0;
    for pos in positions {
        sum += message[pos].1;
    }
    sum
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
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
