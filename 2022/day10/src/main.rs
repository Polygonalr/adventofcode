use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn p1(line_vec: Vec<String>) -> i32 {
    static CYCLE_TO_RECORD_INCREMENT: i32 = 40;
    static MAX_CYCLE_TO_RECORD: i32 = 220;
    let mut cycle_to_record = 20;
    let mut cycle = 0;
    let mut register = 1;
    let mut sum = 0;
    for line in line_vec {
        let l = line.split_whitespace().collect::<Vec<_>>();
        match l[..] {
            ["addx", add_arg] => {
                cycle += 2;
                if cycle >= cycle_to_record {
                    sum += register * cycle_to_record;
                    if cycle_to_record == MAX_CYCLE_TO_RECORD {
                        return sum;
                    }
                    cycle_to_record += CYCLE_TO_RECORD_INCREMENT;
                }
                register += add_arg.parse::<i32>().unwrap();
            }
            ["noop"] => {
                cycle += 1;
                if cycle >= cycle_to_record {
                    sum += register * cycle_to_record;
                    if cycle_to_record == MAX_CYCLE_TO_RECORD {
                        return sum;
                    }
                    cycle_to_record += CYCLE_TO_RECORD_INCREMENT;
                }
            }
            _ => panic!("Invalid instruction detected.")
        }
        
    }
    sum
}

fn push_pixel(screen: &mut Vec<char>, register: i32, cycle: i32) {
    if register >= ((cycle - 1) % 40 - 1) && register <= ((cycle - 1) % 40 + 1) {
        (*screen).push('#');
    } else {
        (*screen).push('.');
    }
}

fn p2(line_vec: Vec<String>) {
    let mut screen: Vec<char> = vec![];
    let mut register = 1;
    let mut cycle = 0;
    for line in line_vec {
        let l = line.split_whitespace().collect::<Vec<_>>();
        match l[..] {
            ["addx", add_arg] => {
                cycle += 1;
                push_pixel(&mut screen, register, cycle);
                cycle += 1;
                push_pixel(&mut screen, register, cycle);
                register += add_arg.parse::<i32>().unwrap();
            }
            ["noop"] => {
                cycle += 1;
                push_pixel(&mut screen, register, cycle);
            }
            _ => panic!("Invalid instruction detected.")
        }
    }
    // print the screen
    for (i, c) in screen.iter().enumerate() {
        if i % 40 == 0 {
            println!();
        }
        print!("{}", c);
    }
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
    println!("Part 1: {}\nPart 2:", p1(line_vec));
    p2(line_vec2);
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
