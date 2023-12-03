/*
    OOP Solution, might be inefficient but this is a modular way to solve this question.
*/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod bingo;
pub use crate::bingo::bingo_grid::BingoGrid;

fn p1(mut bingo_grids: Vec<BingoGrid>, winning_numbers: Vec<i32>) -> i32 {
    for n in winning_numbers.iter().take(4) {
        for bingo_grid in &mut bingo_grids {
            bingo_grid.add_winning_number(*n);
        }
    }
    let (mut i, mut index) = (4, 0);
    let mut current_winning_number: i32;
    let mut done = false;
    loop {
        current_winning_number = winning_numbers[i];
        for (j, bingo_grid) in bingo_grids.iter_mut().enumerate() {
            if bingo_grid.add_winning_number(current_winning_number) 
            && bingo_grid.is_bingo() {
                index = j;
                done = true;
                break;
            }
        }
        if done {
            break;
        }
        i += 1;
    }
    bingo_grids[index].sum_of_non_winning_numbers() * current_winning_number
}

fn p2(mut bingo_grids: Vec<BingoGrid>, winning_numbers: Vec<i32>) -> i32 {
    for n in winning_numbers.iter().take(4) {
        for bingo_grid in &mut bingo_grids {
            bingo_grid.add_winning_number(*n);
        }
    }
    let mut i = 4;
    let mut current_winning_number: i32;
    loop {
        current_winning_number = winning_numbers[i];
        let mut indexes_to_remove: Vec<usize> = Vec::new();
        for (j, bingo_grid) in bingo_grids.iter_mut().enumerate() {
            if bingo_grid.add_winning_number(current_winning_number) 
            && bingo_grid.is_bingo() {
                indexes_to_remove.push(j);
            }
        }
        indexes_to_remove.reverse();
        for j in indexes_to_remove {
            bingo_grids.remove(j);
        }
        if bingo_grids.len() == 1 {
            break;
        }
        i += 1;
    }

    loop {
        current_winning_number = winning_numbers[i];
        if bingo_grids[0].add_winning_number(current_winning_number) 
        && bingo_grids[0].is_bingo() {
            break;
        }
        i += 1;
    }

    bingo_grids[0].sum_of_non_winning_numbers() * current_winning_number
}

fn main() {
    let filepath = "./input.txt";
    let mut winning_numbers: Vec<i32> = Vec::new();
    let mut bingo_grids: Vec<BingoGrid> = Vec::new();
    let (mut first_line_done, mut second_line_done) = (false, false);
    let mut bingo_buffer: [[i32; 5]; 5] = [[0; 5]; 5];
    let mut current_row = 0;

    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if !first_line_done {
                if let Ok(s) = line {
                    let s_vec: Vec<i32> = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
                    winning_numbers = s_vec.to_vec();
                    first_line_done = true;
                }
            } else if !second_line_done {
                second_line_done = true;
                continue;
            } else if let Ok(s) = line {
                // store bingo grids
                if s.is_empty() {
                    let new_bingo = BingoGrid::new(bingo_buffer);
                    bingo_grids.push(new_bingo);
                    bingo_buffer = [[0; 5]; 5];
                    current_row = 0;
                    continue;
                }
                let s_vec: Vec<i32> = s.split(' ').filter(|&x| !x.is_empty())
                        .map(|x| x.parse::<i32>().unwrap()).collect();
                // the line below this for loop replaces it
                // for i in 0..s_vec.len() {
                //     bingo_buffer[current_row][i] = s_vec[i];
                // }
                bingo_buffer[current_row][..s_vec.len()].clone_from_slice(&s_vec[..]);
                current_row += 1;
            }
        }
    }
    // let line_vec2 = line_vec.to_vec();
    println!("Part 1: {}\nPart 2: {}", p1(bingo_grids.to_vec(), winning_numbers.to_vec()),
                                       p2(bingo_grids.to_vec(), winning_numbers.to_vec()));
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
