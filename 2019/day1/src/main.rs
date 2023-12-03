use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total_fuel = 0;
    let filepath = "./input.txt";
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(mass_string) = line {
                let mut mass = mass_string.parse::<i32>().unwrap();
                mass = mass / 3 - 2;
                total_fuel += mass;
            }
        }
    }
    println!("Part 1: {}", total_fuel);

    total_fuel = 0;
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(mass_string) = line {
                let mut mass = mass_string.parse::<i32>().unwrap();
                mass = mass / 3 - 2;
                total_fuel += mass;
                loop {
                    mass = mass / 3 - 2;
                    if mass <= 0 { break; }
                    total_fuel += mass;
                }
            }
        }
    }
    println!("Part 2: {}", total_fuel);
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}