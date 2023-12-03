use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn p1(line_vec: Vec<String>) -> i32 {
    let mut score = 0;
    for round in line_vec {
        let rs = round.split(' ').collect::<Vec<&str>>();
        let opp = rs[0].chars().next().unwrap();
        let mine = rs[1].chars().next().unwrap();
        let outcome = (mine as i32 - 'X' as i32 - (opp as i32 - 'A' as i32)).rem_euclid(3);
        if outcome == 0 {
            // draw
            score += 3;
        } else if outcome == 1 {
            // win
            score += 6;
        }
        score +=  mine as i32 - 'X' as i32 + 1;
    }
    score
}

fn p2(line_vec: Vec<String>) -> i32 {
    let mut score = 0;
    for round in line_vec {
        let rs = round.split(' ').collect::<Vec<&str>>();
        let opp = rs[0].chars().next().unwrap();
        let outcome = rs[1].chars().next().unwrap();
        let (mine, outcome_score) = match outcome {
            'X' => ((opp as i32 - 'A' as i32 - 1).rem_euclid(3) + 1, 0), // lose
            'Y' => (opp as i32 - 'A' as i32 + 1, 3), // draw
            'Z' => ((opp as i32 - 'A' as i32 + 1).rem_euclid(3) + 1, 6), // win
            _ => (0, 0),
        };
        score += outcome_score + mine;
    }
    score
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
