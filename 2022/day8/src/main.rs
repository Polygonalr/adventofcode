use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

// Taken from https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn p1(grid: &Vec<Vec<i32>>, grid_transpose: &Vec<Vec<i32>>) -> i32 {
    let mut hashset: HashSet<(usize, usize)> = HashSet::new();
    // note: this is very parallelizable in OpenMP - just that our input data is not large
    // enough to justify the need for parallelism
    // left to right
    for (y, row) in grid.iter().enumerate() {
        let mut high = -1;
        for (x, tree) in row.iter().enumerate() {
            if *tree > high {
                high = *tree;
                hashset.insert((x, y));
            }
        }
    }
    // right to left
    for (y, row) in grid.iter().enumerate() {
        let mut high = -1;
        for (x, tree) in row.iter().enumerate().rev() {
            if *tree > high {
                high = *tree;
                hashset.insert((x, y));
            }
        }
    }
    // up to down
    for (x, col) in grid_transpose.iter().enumerate() {
        let mut high = -1;
        for (y, tree) in col.iter().enumerate() {
            if *tree > high {
                high = *tree;
                hashset.insert((x, y));
            }
        }
    }
    // down to up
    for (x, col) in grid_transpose.iter().enumerate() {
        let mut high = -1;
        for (y, tree) in col.iter().enumerate().rev() {
            if *tree > high {
                high = *tree;
                hashset.insert((x, y));
            }
        }
    }

    // cool visualisation in the terminal to see which tree is visible
    for (y, row) in grid.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            if hashset.contains(&(x, y)) {
                print!("\x1b[91m{}\x1b[0m", *tree);
            } else {
                print!("{}", *tree);
            }
        }
        println!();
    }
    hashset.len() as i32
}

fn p2(grid: &Vec<Vec<i32>>, grid_transpose: &Vec<Vec<i32>>) -> i32 {
    0
}

fn main() {
    let filepath = "./input.txt";
    // let mut str_buf = "".to_owned();
    let mut grid: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines.flatten() {
            // Process each line...
            let mut row = vec![];
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as i32);
            }
            grid.push(row);
        }
    }
    let grid_transpose = transpose(&grid);
    let ans1 = p1(&grid, &grid_transpose);
    let ans2 = p2(&grid, &grid_transpose);
    println!("Part 1: {}\nPart 2: {}", ans1, ans2);
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
