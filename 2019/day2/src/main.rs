use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut instr_pointer = 0;
    let filepath = "./input.txt";
    if let Ok(lines) = read_lines(filepath){
        for line in lines {
            if let Ok(s) = line{
                let mut vec:Vec<String> = s.split(",").map(|s| s.to_string()).collect();
                vec[1] = "65".to_string();
                vec[2] = "33".to_string();
                loop {
                    if vec[instr_pointer] == "1" {
                        let pos = (vec[instr_pointer+1].parse::<usize>().unwrap(), vec[instr_pointer+2].parse::<usize>().unwrap());
                        let sum = vec[pos.0].parse::<i32>().unwrap() + vec[pos.1].parse::<i32>().unwrap();
                        let sum_string = sum.to_string();
                        let pos_to_change = vec[instr_pointer+3].parse::<usize>().unwrap();
                        vec[pos_to_change] = sum_string;
                        instr_pointer += 4;
                        println!("1 - {}", vec[pos_to_change]);
                    }
                    else if vec[instr_pointer] == "2" {
                        let pos = (vec[instr_pointer+1].parse::<usize>().unwrap(), vec[instr_pointer+2].parse::<usize>().unwrap());
                        let sum = vec[pos.0].parse::<i32>().unwrap() * vec[pos.1].parse::<i32>().unwrap();
                        let sum_string = sum.to_string();
                        let pos_to_change = vec[instr_pointer+3].parse::<usize>().unwrap();
                        vec[pos_to_change] = sum_string;
                        instr_pointer += 4;
                        println!("2 - {}", vec[pos_to_change]);
                    }
                    else if vec[instr_pointer] == "99" {
                        break;
                    }
                }
                println!("{}", vec[0]);
            }
            break;
        }
    }
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}