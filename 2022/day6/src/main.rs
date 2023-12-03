use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::hash::Hash;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

/** 
 * Iterate through the line with sliding window. For every character, check for uniqueness
 * within the sliding window.
 */
fn p1(line_vec: Vec<String>) -> i32 {
    static MAX_CONSECUTIVE_CHARS: usize = 4;
    let mut sum = 0;
    for line in line_vec {
        let mut deq: VecDeque<char> = VecDeque::new();
        let chars = line.chars();
        for c in chars {
            deq.push_back(c);
            sum += 1;
            if deq.len() == MAX_CONSECUTIVE_CHARS {
                if has_unique_elements(deq.iter()) {
                    break;
                }
                deq.pop_front();
            
            }
        }
    }
    sum
}

// Same solution as p1, except MAX_CONSECUTIVE_CHARS changed to 14
fn p2(line_vec: Vec<String>) -> i32 {
    static MAX_CONSECUTIVE_CHARS: usize = 14;
    let mut sum = 0;
    for line in line_vec {
        let mut deq: VecDeque<char> = VecDeque::new();
        let chars = line.chars();
        for c in chars {
            deq.push_back(c);
            sum += 1;
            if deq.len() == MAX_CONSECUTIVE_CHARS {
                if has_unique_elements(deq.iter()) {
                    break;
                }
                deq.pop_front();
            }
        }
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
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
