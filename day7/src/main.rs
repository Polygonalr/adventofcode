use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::collections::VecDeque;

fn parse_cd(breadcrumb: &mut VecDeque<String>, dir_name: &str) {
    if dir_name.starts_with("..") {
        breadcrumb.pop_back();
    } else {
        breadcrumb.push_back(dir_name.to_owned() + "/");
    }
}

fn build_dir_size_hashmap(line_vec: Vec<String>) -> HashMap<String, i32> {
    let mut dir_sizes: HashMap<String, i32> = HashMap::new();
    let mut breadcrumb: VecDeque<String> = VecDeque::new();
    for line in line_vec {
        let l = line.split_whitespace().collect::<Vec<_>>();
        match l[..] {
            ["$", "cd", dir] => {
                parse_cd(&mut breadcrumb, dir);
            }
            ["$", "ls"] | ["dir", _] => {
                continue;
            }
            [file_size, _] => {
                let mut fullpath: String = "".to_owned();
                for dir in &breadcrumb {
                    fullpath += dir;
                    *dir_sizes.entry(fullpath.to_owned()).or_insert(0) += file_size.parse::<i32>().unwrap();
                }
            }
            _ => {
                continue;
            }
        }
    }
    dir_sizes
}

/**
 * DFS with backtracking & storing of directory sizes in a HashMap
 */
fn p1(dir_sizes: &HashMap<String, i32>) -> i32 {
    static MAX_DIR_SIZE: i32 = 100000;
    let mut sum = 0;
    for (_key, value) in dir_sizes.iter() {
        if value <= &MAX_DIR_SIZE {
            sum += value;
        }
    }
    sum
}

fn p2(dir_sizes: &HashMap<String, i32>) -> i32 {
    let space_to_free: i32 = dir_sizes["//"] - 40000000;
    let mut min = 70000000;
    for (_key, value) in dir_sizes.iter() {
        if value >= &space_to_free && value < &min {
            min = *value;
        }
    }
    min
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
    let dir_sizes = build_dir_size_hashmap(line_vec);
    println!("Part 1: {}\nPart 2: {}", p1(&dir_sizes), p2(&dir_sizes));
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
