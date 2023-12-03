use std::{env, fs};

const TEST_FLAG: &str = "--test";
const INPUT_FILEPATH: &str = "./input.txt";
const TEST_FILEPATH: &str = "./test.txt";

fn check_surr(indices: &Vec<(usize, usize)>, grid: &Vec<Vec<char>>) -> bool {
    for &(i, j) in indices.iter() {
        for di in [-1, 0, 1] {
            for dj in [-1, 0, 1] {
                if di == 0 && dj == 0 {
                    continue;
                }
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni < 0 || ni >= grid.len() as i32 || nj < 0 || nj >= grid[0].len() as i32 {
                    continue;
                }
                let (ni, nj) = (ni as usize, nj as usize);
                if !(grid[ni][nj].is_digit(10) || grid[ni][nj] == '.') {
                    return false;
                }
            }
        }
    }
    true
}

fn p1(line_vec: &[String]) -> i32 {
    let grid = line_vec
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut res = 0;
    for (i, row) in grid.iter().enumerate() {
        let mut curr_num = "".to_owned();
        let mut indices: Vec<(usize, usize)> = vec![];
        for (j, c) in row.iter().enumerate() {
            if c.is_digit(10) {
                curr_num.push(*c);
                indices.push((i, j));
            } else {
                if curr_num.len() != 0 && !check_surr(&indices, &grid) {
                    res += curr_num.parse::<i32>().unwrap();
                }
                // reset buffer
                curr_num = "".to_owned();
                indices = vec![];
            }
        }
        // do one final check for numbers lying at the end of the rows
        if curr_num.len() != 0 && !check_surr(&indices, &grid) {
            res += curr_num.parse::<i32>().unwrap();
        }
    }
    res
}

// lazy solution that assumes all numbers next to the same gear are unique
fn check_surr2((i, j): (usize, usize), grid: &Vec<Vec<char>>) -> i32 {
    let mut nums = vec![];
    for di in [-1, 0, 1] {
        for dj in [-1, 0, 1] {
            if di == 0 && dj == 0 {
                continue;
            }
            let (ni, nj) = (i as i32 + di, j as i32 + dj);
            if ni < 0 || ni >= grid.len() as i32 || nj < 0 || nj >= grid[0].len() as i32 {
                continue;
            }
            let (ni, mut nj) = (ni as usize, nj as usize);
            if grid[ni][nj].is_digit(10) {
                // backtrack
                let mut curr_num = "".to_owned();
                while nj != 0 && grid[ni][nj - 1].is_digit(10) {
                    nj -= 1;
                }
                while nj < grid[0].len() && grid[ni][nj].is_digit(10) {
                    curr_num.push(grid[ni][nj]);
                    nj += 1;
                }
                let curr_num = curr_num.parse::<i32>().unwrap();
                // lol
                if !nums.contains(&curr_num) {
                    nums.push(curr_num);
                }
            }
        }
    }

    if nums.len() == 2 {
        return nums[0] * nums[1];
    }
    0
}

fn p2(line_vec: &[String]) -> i32 {
    let grid = line_vec
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut res = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '*' {
                res += check_surr2((i, j), &grid)
            }
        }
    }
    res
}

fn main() {
    let filepath = if env::args().any(|x| x == *TEST_FLAG) {
        TEST_FILEPATH
    } else {
        INPUT_FILEPATH
    };
    let input = fs::read_to_string(filepath).unwrap();
    let mut line_vec: Vec<String> = Vec::new();
    for line in input.lines() {
        // Process each line...
        line_vec.push(line.to_string());

        // Use the following if only need to process digits
        // let to_push = line
        //     .chars()
        //     .filter(|c| c.is_ascii_digit() || c.is_ascii_whitespace())
        //     .collect::<String>();
        // line_vec.push(to_push);
    }
    println!("Part 1: {}\nPart 2: {}", p1(&line_vec), p2(&line_vec));
}
