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
                loop {
                    if vec[instr_pointer].len() > 2 {
                        // Immediate mode
                        let full_instruction = &vec[instr_pointer]; // includes the modes
                        let instruction = full_instruction[full_instruction.len()-2..].to_string();
                        if instruction == "01" {
                            let (mode_1, mode_2):(String, String);
                            mode_1 = full_instruction[full_instruction.len()-3..full_instruction.len()-2].to_string();
                            if full_instruction.len() == 4 {
                                mode_2 = full_instruction[full_instruction.len()-4..full_instruction.len()-3].to_string();
                            }
                            else {
                                mode_2 = "0".to_string();
                            }

                            let pos = (vec[instr_pointer+1].parse::<i32>().unwrap(), vec[instr_pointer+2].parse::<i32>().unwrap());
                            let mut values:(i32,i32) = (0, 0);
                            if mode_1 == "1"{
                                values.0 = pos.0 as i32;
                            }
                            else {
                                values.0 = vec[pos.0 as usize].parse::<i32>().unwrap();
                            }
                            if mode_2 == "1"{
                                values.1 = pos.1 as i32;
                            }
                            else {
                                values.1 = vec[pos.1 as usize].parse::<i32>().unwrap();
                            }
                            let sum = values.0 + values.1;
                            let sum_string = sum.to_string();
                            let pos_to_change = vec[instr_pointer+3].parse::<usize>().unwrap();
                            vec[pos_to_change] = sum_string;
                            instr_pointer += 4;
                        }
                        else if instruction == "02" {
                            let (mode_1, mode_2):(String, String);
                            mode_1 = full_instruction[full_instruction.len()-3..full_instruction.len()-2].to_string();
                            if full_instruction.len() == 4 {
                                mode_2 = full_instruction[full_instruction.len()-4..full_instruction.len()-3].to_string();
                            }
                            else {
                                mode_2 = "0".to_string();
                            }

                            let pos = (vec[instr_pointer+1].parse::<i32>().unwrap(), vec[instr_pointer+2].parse::<i32>().unwrap());
                            let mut values:(i32,i32) = (0, 0);
                            if mode_1 == "1"{
                                values.0 = pos.0 as i32;
                            }
                            else {
                                values.0 = vec[pos.0 as usize].parse::<i32>().unwrap();
                            }
                            if mode_2 == "1"{
                                values.1 = pos.1 as i32;
                            }
                            else {
                                values.1 = vec[pos.1 as usize].parse::<i32>().unwrap();
                            }
                            let sum = values.0 * values.1;
                            let sum_string = sum.to_string();
                            let pos_to_change = vec[instr_pointer+3].parse::<usize>().unwrap();
                            vec[pos_to_change] = sum_string;
                            instr_pointer += 4;
                        }
                        else if instruction == "04" {
                            let mode_1 = full_instruction[0..1].to_string();
                            let pos_to_print = vec[instr_pointer+1].parse::<i32>().unwrap();
                            if mode_1 == "1" {
                                println!("ANS: {}", pos_to_print);
                            }
                            else {
                                println!("ANS: {}", vec[pos_to_print as usize]);
                            }
                            instr_pointer += 2;
                        }
                    }
                    else if vec[instr_pointer] == "1" {
                        let pos = (vec[instr_pointer+1].parse::<usize>().unwrap(), vec[instr_pointer+2].parse::<usize>().unwrap());
                        let sum = vec[pos.0].parse::<i32>().unwrap() + vec[pos.1].parse::<i32>().unwrap();
                        let sum_string = sum.to_string();
                        let pos_to_change = vec[instr_pointer+3].parse::<usize>().unwrap();
                        vec[pos_to_change] = sum_string;
                        instr_pointer += 4;
                    }
                    else if vec[instr_pointer] == "2" {
                        let pos = (vec[instr_pointer+1].parse::<usize>().unwrap(), vec[instr_pointer+2].parse::<usize>().unwrap());
                        let sum = vec[pos.0].parse::<i32>().unwrap() * vec[pos.1].parse::<i32>().unwrap();
                        let sum_string = sum.to_string();
                        let pos_to_change = vec[instr_pointer+3].parse::<usize>().unwrap();
                        vec[pos_to_change] = sum_string;
                        instr_pointer += 4;
                    }
                    else if vec[instr_pointer] == "3" {
                        let pos_to_change = vec[instr_pointer+1].parse::<usize>().unwrap();
                        vec[pos_to_change] = 1.to_string();
                        instr_pointer += 2;
                    }
                    else if vec[instr_pointer] == "4" {
                        let pos_to_print = vec[instr_pointer+1].parse::<usize>().unwrap();
                        println!("ANS: {}", vec[pos_to_print]);
                        instr_pointer += 2;
                    }
                    else if vec[instr_pointer] == "99" {
                        break;
                    }
                    else {
                        println!("Invalid instruction {} at {} detected!", vec[instr_pointer], instr_pointer);
                        break;
                    }
                }
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