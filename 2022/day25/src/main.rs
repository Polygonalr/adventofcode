use std::{env, fs};

const TEST_FLAG: &str = "--test";
const INPUT_FILEPATH: &str = "./input.txt";
const TEST_FILEPATH: &str = "./test.txt";

fn p1(line_vec: &[String]) -> String {
    let mut total_sum = 0;
    for line in line_vec {
        let mut c_pow = 0;
        for c in line.chars().rev() {
            let to_add = match c {
                '2' => 5_i64.pow(c_pow) * 2,
                '1' => 5_i64.pow(c_pow),
                '0' => 0,
                '-' => 5_i64.pow(c_pow) * (-1),
                '=' => 5_i64.pow(c_pow) * (-2),
                _ => unreachable!()
            };
            total_sum += to_add;
            c_pow += 1;
        }
    }
    // convert total_sum to SNAFU
    let mut snafu: String = "".to_owned();
    while total_sum > 0 {
        let rem = (total_sum+2) % 5;
        total_sum = (total_sum+2) / 5;
        let to_add = match rem {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => unreachable!()
        };
        snafu.push(to_add);
    }
    snafu.chars().rev().collect::<String>()
}


fn p2(line_vec: &[String]) -> i32 {
    0
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
