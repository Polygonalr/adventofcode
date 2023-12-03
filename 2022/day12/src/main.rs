use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

// for debugging!
fn print_grid(grid: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>) {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if visited[y][x] {
                print!("\x1b[91m{}\x1b[0m", c)
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

// BFS algo
fn p1(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new(); // (x, y, weight)
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    queue.push_back((start.0 as i32, start.1 as i32, 0));
    visited[start.1][start.0] = true;
    // println!("Pushed {}, {} ({})", start.0, start.1, 0);
    while let Some((x, y, weight)) = queue.pop_front() {
        // println!("Popped {}, {} ({})", x, y, weight);
        for (dx, dy) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let (nx, ny): (i32, i32) = (x + dx, y+ dy);
            if nx < 0 || nx >= grid[0].len() as i32 || ny < 0 || ny >= grid.len() as i32
                    || grid[ny as usize][nx as usize] as i32 - grid[y as usize][x as usize] as i32 > 1 {
                continue;
            }
            if (nx, ny) == (end.0 as i32, end.1 as i32) {
                return weight + 1;
            }
            if !visited[ny as usize][nx as usize] {
                queue.push_back((nx, ny, weight + 1));
                // println!("Pushed {}, {} ({})", nx, ny, weight + 1);
                visited[ny as usize][nx as usize] = true;
                // print_grid(grid, &visited);
            }
        }
    }
    panic!("Nothing left in the queue!");
}

// Same BFS algo, just loop from every possible start
fn p2(grid: &Vec<Vec<char>>, end: (usize, usize)) -> i32 {
    let mut starts: Vec<(usize, usize)> = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'a' {
                starts.push((x, y));
            }
        }
    }

    let mut min = i32::MAX;

    for start in starts {
        let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new(); // (x, y, weight)
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        queue.push_back((start.0 as i32, start.1 as i32, 0));
        visited[start.1][start.0] = true;
        let mut break_outer = false;
        // println!("Pushed {}, {} ({})", start.0, start.1, 0);
        while let Some((x, y, weight)) = queue.pop_front() {
            // println!("Popped {}, {} ({})", x, y, weight);
            for (dx, dy) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
                let (nx, ny): (i32, i32) = (x + dx, y+ dy);
                if nx < 0 || nx >= grid[0].len() as i32 || ny < 0 || ny >= grid.len() as i32
                        || grid[ny as usize][nx as usize] as i32 - grid[y as usize][x as usize] as i32 > 1 {
                    continue;
                }
                if (nx, ny) == (end.0 as i32, end.1 as i32) && weight + 1 < min {
                    min = weight + 1;
                    break_outer = true;
                    break;
                }
                if !visited[ny as usize][nx as usize] {
                    queue.push_back((nx, ny, weight + 1));
                    // println!("Pushed {}, {} ({})", nx, ny, weight + 1);
                    visited[ny as usize][nx as usize] = true;
                    // print_grid(grid, &visited);
                }
            }
            if break_outer {
                break;
            }
        }
    }
    min
}

fn main() {
    let filepath = "./input.txt";
    // let mut str_buf = "".to_owned();
    let mut grid: Vec<Vec<char>> = vec![];
    let mut start: (usize, usize) = (usize::MAX, usize::MAX);
    let mut end: (usize, usize) = (usize::MAX, usize::MAX);
    if let Ok(lines) = read_lines(filepath) {
        for (y, line) in lines.flatten().enumerate() {
            let mut row: Vec<char> = vec![];
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = (x, y);
                        row.push('a');
                    }
                    'E' => {
                        end = (x, y);
                        row.push('z');
                    }
                    _ => {
                        row.push(c);
                    }
                }
            }
            grid.push(row);
        }
    }
    let ans1 = p1(&grid, start, end);
    let ans2 = p2(&grid, end);
    println!("Part 1: {}\nPart 2: {}", ans1, ans2);
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
