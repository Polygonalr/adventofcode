use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn letter_to_priority(c: char) -> i32 {
    if c as i32 <= 'Z' as i32 {
        return c as i32 - 'A' as i32 + 27;
    }
    c as i32 - 'a' as i32 + 1
}

/**
 * Strategy is to split each line of string into equal halves,
 * then sort both halves and iterate through them after sorting
 * to see what item is common.
 */
fn p1(line_vec: Vec<String>) -> i32 {
    let mut sum = 0;
    for line in line_vec {
        let mid = line.len() / 2;
        let mut first: Vec<char> = line[..mid].chars().collect();
        first.sort();
        let mut second: Vec<char> = line[mid..].chars().collect();
        second.sort();
        assert!(first.len() == second.len());
        let mut i = 0;
        let mut j = 0;
        while i < mid && j < mid {
            if second[j].cmp(&first[i]) == Ordering::Equal {
                break;
            }
            if second[j].cmp(&first[i]) == Ordering::Less {
                j += 1;
            } else {
                i += 1;
            }
        }
        sum += letter_to_priority(first[i]);
    }
    sum
}

/**
 * Same strategy, but sort and match every 3 lines of string.
 */
fn p2(line_vec: Vec<String>) -> i32 {
    let mut sum = 0;
    let mut elf_count = 0;
    let mut group: Vec<Vec<char>> = Vec::new();
    let mut it: [usize; 3];
    for line in line_vec {
        let mut chars: Vec<char> = line.chars().collect();
        chars.sort();
        group.push(chars);
        elf_count += 1;
        if elf_count == 3 {
            it = [0, 0, 0];
            while it[0] < group[0].len() && it[1] < group[1].len() && it[2] < group[2].len() {
                if group[0][it[0]].cmp(&group[1][it[1]]) == Ordering::Equal && group[2][it[2]].cmp(&group[1][it[1]]) == Ordering::Equal {
                    break;
                }
                if group[0][it[0]].cmp(&group[1][it[1]]) == Ordering::Less || group[0][it[0]].cmp(&group[2][it[2]]) == Ordering::Less {
                    it[0] += 1;
                }
                if group[1][it[1]].cmp(&group[0][it[0]]) == Ordering::Less || group[1][it[1]].cmp(&group[2][it[2]]) == Ordering::Less {
                    it[1] += 1;
                }
                if group[2][it[2]].cmp(&group[1][it[1]]) == Ordering::Less || group[2][it[2]].cmp(&group[0][it[0]]) == Ordering::Less {
                    it[2] += 1;
                }
            }
            sum += letter_to_priority(group[0][it[0]]);
            group.clear();
            elf_count = 0;
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
