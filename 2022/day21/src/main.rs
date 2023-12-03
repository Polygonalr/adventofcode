use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn p1(line_vec: &Vec<String>) -> i64 {
    let mut vals: HashMap<String, Option<i64>> = HashMap::new();
    let mut operations: Vec<String> = vec![];
    for line in line_vec {
        let var_name = line.split(": ").next().unwrap();
        let s = line.split(' ').collect::<Vec<_>>();
        if s.len() == 2 {
            vals.insert(var_name.to_owned(), Some(s[1].parse::<i64>().unwrap()));
        } else {
            vals.insert(var_name.to_owned(), None);
            operations.push(line.to_string());
        }
    }
    loop {
        for op in &operations {
            let var_name = op.split(": ").next().unwrap();
            let s = op.split(' ').collect::<Vec<_>>();
            let (param1_s, param2_s) = (s[1].to_owned(), s[3].to_owned());
            if vals.get(&param1_s).unwrap().is_none() || vals.get(&param2_s).unwrap().is_none() {
                continue;
            }
            let (param1, param2) = (
                vals.get(&param1_s).unwrap().unwrap(),
                vals.get(&param2_s).unwrap().unwrap(),
            );
            let op = s[2].to_owned();
            match op.as_ref() {
                "*" => {
                    vals.entry(var_name.to_owned())
                        .and_modify(|v| *v = Some(param1 * param2));
                }
                "-" => {
                    vals.entry(var_name.to_owned())
                        .and_modify(|v| *v = Some(param1 - param2));
                }
                "+" => {
                    vals.entry(var_name.to_owned())
                        .and_modify(|v| *v = Some(param1 + param2));
                }
                "/" => {
                    vals.entry(var_name.to_owned())
                        .and_modify(|v| *v = Some(param1 / param2));
                }
                _ => {
                    panic!("Invalid operation");
                }
            }
            if var_name == "root" {
                return vals.get(var_name).unwrap().unwrap();
            }
        }
    }
}

/**
 * Build an equation string to be thrown into an equation solver online
 * like https://www.mathpapa.com/equation-solver/
 */
fn p2(line_vec: &Vec<String>) {
    let mut vals: HashMap<String, String> = HashMap::new();
    let mut operations: Vec<String> = vec![];
    for line in line_vec {
        let var_name = line.split(": ").next().unwrap();
        let s = line.split(' ').collect::<Vec<_>>();
        if s.len() == 2 {
            if var_name == "humn" {
                vals.insert(var_name.to_owned(), "x".to_owned());
            } else {
                vals.insert(var_name.to_owned(), s[1].to_owned());
            }
        } else if var_name == "root" {
            operations.push(line.replace('+', "="));
        } else {
            operations.push(line.to_string());
        }
    }

    while vals.len() != line_vec.len() {
        for op in &operations {
            let var_name = op.split(": ").next().unwrap();
            if vals.contains_key(var_name) {
                continue;
            }

            let s = op.split(' ').collect::<Vec<_>>();
            let (param1_s, param2_s) = (s[1].to_owned(), s[3].to_owned());
            if vals.get(&param1_s).is_none() || vals.get(&param2_s).is_none() {
                continue;
            }
            let (param1, param2) = (
                vals.get(&param1_s).unwrap().to_owned(),
                vals.get(&param2_s).unwrap().to_owned(),
            );
            let op = s[2].to_owned();
            vals.insert(
                var_name.to_owned(),
                "(".to_string() + &param1 + &op + &param2 + ")",
            );
        }
    }

    println!("{}", vals["root"]);
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
    println!("Part 1: {}", p1(&line_vec));
    println!("Part 2 equation:");
    p2(&line_vec);
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
