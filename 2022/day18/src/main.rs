use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, VecDeque, HashSet};

const MIN_COORDS: i32 = -1;
const MAX_COORDS: i32 = 20;

fn p1(line_vec: Vec<String>) -> i32 {
    let mut cube_states: HashMap<(i32, i32, i32), bool> = HashMap::new();
    let mut area_sum = 0;
    for line in line_vec {
        if let [x, y, z] = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>()[..] {
            area_sum += 6;
            for (dx, dy, dz) in [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)] {
                let (nx, ny, nz) = (x+dx, y+dy, z+dz);
                if cube_states.get(&(nx, ny, nz)).is_some() {
                    area_sum -= 2;
                }
            }
            cube_states.insert((x,y,z), true);
        } else {
            panic!("Invalid input detected.");
        }
    }
    area_sum
}

fn p2(line_vec: Vec<String>) -> i32 {
    let mut cube_states: HashMap<(i32, i32, i32), bool> = HashMap::new();
    for line in line_vec {
        if let [x, y, z] = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>()[..] {
            cube_states.insert((x,y,z), true);
        } else {
            panic!("Invalid input detected.");
        }
    }
    // flood fill
    let mut area_sum = 0;
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut flood_queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    visited.insert((MIN_COORDS, MIN_COORDS, MIN_COORDS));
    flood_queue.push_back((MIN_COORDS, MIN_COORDS, MIN_COORDS));
    while let Some((x, y, z)) = flood_queue.pop_front() {
        for (dx, dy, dz) in [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)] {
            let (nx, ny, nz) = (x+dx, y+dy, z+dz);
            if !(MIN_COORDS..=MAX_COORDS).contains(&nx)
                || !(MIN_COORDS..=MAX_COORDS).contains(&ny)
                || !(MIN_COORDS..=MAX_COORDS).contains(&nz) {
                continue;
            }
            if cube_states.get(&(nx, ny, nz)).is_some() {
                area_sum += 1;
            } else if !visited.contains(&(nx, ny, nz)) {
                visited.insert((nx, ny, nz));
                flood_queue.push_back((nx, ny, nz));
            }
        }
    }
    area_sum
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
